use actix::prelude::*;
use failure::Error;
use futures::future::ok as FutureOk;
use futures::future::FutureResult;
use futures::{Async, Future, Poll};
use reqwest::r#async::Client as AsyncClient;
use reqwest::r#async::Response as AsyncResponse;
use reqwest::Error as ReqwestError;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io;
use tokio::prelude::*;

// GoogleBooksAPI
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndustryIdentifier {
    r#type: String,
    identifier: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInfo {
    title: String,
    page_count: i32,
    industry_identifiers: Vec<IndustryIdentifier>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    id: String,
    volume_info: VolumeInfo,
}

#[derive(Fail, Debug)]
pub enum BookApiError {
    #[fail(display = "ISBN13 not found")]
    NotFound,
}

impl TryFrom<Volume> for Book {
    type Error = Error;
    fn try_from(item: Volume) -> Result<Self, Self::Error> {
        let industry_identifier = item
            .volume_info
            .industry_identifiers
            .iter()
            .find(|industry_identifier| industry_identifier.r#type == "ISBN_13");
        // .ok_or("not found isbn_13");
        match industry_identifier {
            Some(identifier) => {
                Isbn::try_from(identifier.identifier.clone()).map(|isbn| Book {
                    id: item.id,
                    title: item.volume_info.title,
                    page_count: item.volume_info.page_count,
                    isbn: isbn, // isbn: Isbn::new(isbn.identifier).unwrap(),
                })
            }
            None => return Err(BookApiError::NotFound.into()),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Volumes {
    items: Vec<Volume>,
}

// model
trait BooksRepository {
    fn find_by_isbn(&self, isbn: Isbn) -> Option<Book>;
    fn add(&mut self, book: Book) -> bool;
    fn latest(&self) -> Option<&Book>;
    fn delete(&mut self, isbn: Isbn) -> bool;
}

pub struct InMemoryBooksRepository(pub Vec<Book>);

impl BooksRepository for InMemoryBooksRepository {
    fn find_by_isbn(&self, isbn: Isbn) -> Option<Book> {
        self.0.iter().find(|book| book.isbn == isbn).map(|book| book.clone())
    }
    fn add(&mut self, book: Book) -> bool {
        self.0.push(book);
        true
    }
    fn latest(&self) -> Option<&Book> {
        self.0.last().and_then(|book| Some(book))
    }
    fn delete(&mut self, isbn: Isbn) -> bool {
        if let Some(index) = self.0.iter_mut().position(|book| book.isbn == isbn) {
            self.0.remove(index);
            return true;
        }
        false
    }
}

#[derive(Clone, Debug)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub page_count: i32,
    pub isbn: Isbn,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Isbn(pub u64);
impl Isbn {
    pub fn new(isbn: u64) -> Result<Self, Error> {
        if !(9780000000000..=9799999999999).contains(&isbn) {
            return Err(IsbnError::RangeError(isbn).into());
        }
        Ok(Isbn(isbn))
    }
    pub fn code(&self) -> u64 {
        self.0
    }
}

#[derive(Fail, Debug)]
pub enum IsbnError {
    #[fail(display = "ISBN must be between 9780000000000 - 9799999999999: {}", _0)]
    RangeError(u64),
    #[fail(display = "parse error to u64: {}", _0)]
    ParseError(String),
}

impl TryFrom<String> for Isbn {
    type Error = Error;
    fn try_from(item: String) -> Result<Self, Self::Error> {
        match item.parse::<u64>() {
            Ok(code) => Self::new(code),
            Err(err) => Err(IsbnError::ParseError(err.to_string()).into()),
        }
    }
}
impl ToString for Isbn {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

fn search_from_isbn(isbn: Isbn) -> impl futures::future::Future<Item = Book, Error = reqwest::Error> {
    let client = AsyncClient::new();
    client
        .get(
            Url::parse(
                format!(
                    "https://www.googleapis.com/books/v1/volumes?q=isbn:{}",
                    isbn.to_string()
                )
                .as_str(),
            )
            .unwrap(),
        )
        .send()
        .and_then(|mut response| {
            dbg!("7");
            response.json::<Volumes>()
        })
        .map(|volumes: Volumes| {
            let book: Book = volumes
                .items
                .into_iter()
                .map(|volume: Volume| Book::try_from(volume))
                .nth(0)
                .unwrap() //nth(0)
                .unwrap(); //try_from
            book
        })
}

// Actor
impl Actor for InMemoryBooksRepository {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }
    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

// Message
pub struct Add {
    pub title: String,
    pub page_count: i32,
    pub isbn: Isbn,
}

impl Message for Add {
    type Result = Result<bool, io::Error>;
}

impl Handler<Add> for InMemoryBooksRepository {
    type Result = Result<bool, io::Error>;
    fn handle(&mut self, msg: Add, _ctx: &mut Context<Self>) -> Self::Result {
        println!("hadle Add");
        // self.add();
        Ok(true)
    }
}

pub struct SearchFromIsbn(pub Isbn);
#[derive(Debug)]
pub enum BookInfoLocation {
    Network,
    InMemory,
}
impl ToString for BookInfoLocation {
    fn to_string(&self) -> String {
        match *self {
            Self::Network => {
                dbg!("network");
                String::from("network")
            }
            Self::InMemory => {
                dbg!("inmemory");

                String::from("inmemory")
            }
        }
    }
}
// #[derive(Debug)]
pub type BookAndLocation = (Book, BookInfoLocation);
impl Message for SearchFromIsbn {
    type Result = Result<BookAndLocation, ReqwestError>;
}
impl Handler<SearchFromIsbn> for InMemoryBooksRepository {
    type Result = ResponseFuture<BookAndLocation, ReqwestError>;
    fn handle(&mut self, msg: SearchFromIsbn, _: &mut Context<Self>) -> Self::Result {
        let inmemory: FutureResult<Option<Book>, _> = FutureOk(self.find_by_isbn(msg.0));
        let api = search_from_isbn(msg.0);
        let pair = api.join(inmemory);
        let data = pair.map(|(netw, inme)| {
            if let Some(inmemory_book) = inme {
                (inmemory_book.clone(), BookInfoLocation::InMemory)
            } else {
                (netw, BookInfoLocation::Network)
            }
        });
        data.boxed()
    }
}

impl InMemoryBooksRepository {
    pub fn new() -> Self {
        InMemoryBooksRepository(vec![
            Book {
                id: "1".to_owned(),
                title: "a".to_owned(),
                page_count: 100,
                isbn: Isbn::new(9784797321943).expect("invalid isbn"),
            },
            Book {
                id: "2".to_owned(),
                title: "b".to_owned(),
                page_count: 200,
                isbn: Isbn::new(9780000000001).expect("invalid isbn"),
            },
        ])
    }
    fn search(&self, id: String) -> Vec<Book> {
        self.0.clone()
    }
    fn last(&self) -> Option<Book> {
        self.0.last().and_then(|book| Some(book.clone()))
    }
}

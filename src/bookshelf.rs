use actix::prelude::*;
use failure::Error;
use futures::future::ok as FutureOk;
use futures::future::FutureResult;
use futures::Future;
use reqwest::r#async::Client as AsyncClient;
use reqwest::Error as ReqwestError;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::io;

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

impl TryFrom<Volume> for BookInfo {
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
                Isbn::try_from(identifier.identifier.clone()).map(|isbn| BookInfo {
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
pub trait BooksRepository {
    fn find_by_isbn(&self, isbn: Isbn) -> Option<Book>;
    fn add(&mut self, book: Book) -> bool;
    fn latest(&self) -> Option<&Book>;
    fn delete(&mut self, isbn: Isbn) -> bool;
    fn find_by_id(&self, id: u32) -> Option<&Book>;
}

pub struct InMemoryBooksRepository(pub Vec<Book>);

impl BooksRepository for InMemoryBooksRepository {
    fn find_by_isbn(&self, isbn: Isbn) -> Option<Book> {
        dbg!("11");
        self.0.iter().find(|book| book.isbn() == isbn).map(|book| book.clone())
    }
    fn add(&mut self, book: Book) -> bool {
        self.0.push(book);
        println!("books {:?}", self.0);
        true
    }
    fn latest(&self) -> Option<&Book> {
        self.0.last().and_then(|book| Some(book))
    }
    fn delete(&mut self, isbn: Isbn) -> bool {
        if let Some(index) = self.0.iter_mut().position(|book| book.isbn() == isbn) {
            self.0.remove(index);
            return true;
        }
        false
    }
    fn find_by_id(&self, id: u32) -> Option<&Book> {
        self.0.iter().find(|book| book.id == id)
    }
}

#[derive(Clone, Debug)]
pub struct Book {
    pub id: u32,
    pub info: BookInfo,
}
impl Book {
    pub fn title(&self) -> &str {
        self.info.title.as_str()
    }
    pub fn page_count(&self) -> i32 {
        self.info.page_count
    }
    pub fn isbn(&self) -> Isbn {
        self.info.isbn
    }
}

#[derive(Clone, Debug)]
pub struct BookInfo {
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

fn search_from_isbn(isbn: Isbn) -> impl futures::future::Future<Item = BookInfo, Error = reqwest::Error> {
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
            let book: BookInfo = volumes
                .items
                .into_iter()
                .map(|volume: Volume| BookInfo::try_from(volume))
                .nth(0)
                .unwrap() //nth(0)
                .unwrap(); //try_from
            book
        })
}

// Actor
impl Actor for InMemoryBooksRepository {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("BooksRepository Actor is alive");
    }
    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("BooksRepository Actor is stopped");
    }
}

// Message

//  bookshelf
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
        let latest_book = self.0.iter().max_by_key(|book| book.id);
        let book = if let Some(latest_book) = latest_book {
            Book {
                id: latest_book.id + 1,
                info: BookInfo {
                    title: msg.title,
                    page_count: msg.page_count,
                    isbn: msg.isbn,
                },
            }
        } else {
            Book {
                id: 1,
                info: BookInfo {
                    title: msg.title,
                    page_count: msg.page_count,
                    isbn: msg.isbn,
                },
            }
        };
        self.add(book);
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
pub type BookAndLocation = (BookInfo, BookInfoLocation);
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
                (inmemory_book.info.clone(), BookInfoLocation::InMemory)
            } else {
                (netw, BookInfoLocation::Network)
            }
        });
        Box::new(data)
    }
}

impl InMemoryBooksRepository {
    pub fn new() -> Self {
        InMemoryBooksRepository(vec![Book {
            id: 1,
            info: BookInfo {
                title: "実践Rust入門".to_owned(),
                page_count: 576,
                isbn: Isbn::new(9784297105594).expect("invalid isbn"),
            },
        }])
    }
    fn search(&self, ids: Vec<u32>) -> Vec<Book> {
        self.0
            .iter()
            .filter(|book| ids.contains(&book.id))
            .map(|book| book.clone())
            .collect()
    }
}
pub struct FindByIsbn(pub Isbn);
impl Message for FindByIsbn {
    type Result = Option<Book>;
}
impl Handler<FindByIsbn> for InMemoryBooksRepository {
    type Result = Option<Book>;
    fn handle(&mut self, msg: FindByIsbn, _: &mut Context<Self>) -> Self::Result {
        self.find_by_isbn(msg.0)
    }
}

pub struct FindById(pub Vec<u32>);
impl Message for FindById {
    type Result = Result<Vec<Book>, io::Error>;
}
impl Handler<FindById> for InMemoryBooksRepository {
    type Result = Result<Vec<Book>, io::Error>;
    fn handle(&mut self, msg: FindById, _: &mut Context<Self>) -> Self::Result {
        Ok(self.search(msg.0))
    }
}

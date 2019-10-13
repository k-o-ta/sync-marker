use super::schema::Isbn as graphqlIsbn;
use actix::prelude::*;
use failure::Error;
use futures::{Async, Future, Poll};
use reqwest::r#async::Client as AsyncClient;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io;

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
    // type Error = &'static str;
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
                    name: item.volume_info.title,
                    page: item.volume_info.page_count,
                    page_in_progress: None,
                    isbn: isbn, // isbn: Isbn::new(isbn.identifier).unwrap(),
                })
            }
            None => return Err(BookApiError::NotFound.into()),
        }

        // .and_then(|_| Err(BookApiError::NotFound.into::<Error>()))
        // .ok_or(Err(BookApiError::NotFound.into()))
        // isbn.parse::<u64>() {

        // pub fn new(isbn: u64) -> Result<Self, Error> {
        // Book {
        //     id: item.id,
        //     name: item.volume_info.title,
        //     page: item.volume_info.page_count,
        //     page_in_progress: None,
        // }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Volumes {
    items: Vec<Volume>,
}
#[derive(Clone)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub page: i32,
    pub page_in_progress: Option<i32>,
    pub isbn: Isbn,
}

#[derive(Fail, Debug)]
pub enum IsbnError {
    #[fail(display = "ISBN must be between 9780000000000 - 9799999999999: {}", _0)]
    RangeError(u64),
    #[fail(display = "parse error to u64: {}", _0)]
    ParseError(String),
}
#[derive(Clone, Copy, PartialEq)]
pub struct Isbn(pub u64);
impl Isbn {
    pub fn new(isbn: u64) -> Result<Self, Error> {
        if !(9780000000000..=9799999999999).contains(&isbn) {
            // return Err(IsbnError::RangeError);
            return Err(IsbnError::RangeError(isbn).into());
        }
        Ok(Isbn(isbn))
    }
    pub fn code(&self) -> u64 {
        self.0
    }
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

pub trait BooksRepository<Item, Error> {
    fn search_from_isbn(isbn: Isbn) -> Box<dyn futures::future::Future<Item = Item, Error = Error>>;
    fn find_by_isbn(&self, isbn: Isbn) -> Option<&Book>;
    fn add(&mut self, book: Book) -> bool;
    fn delete(&mut self, isbn: Isbn) -> bool;
    fn latest(&self) -> Option<&Book>;
}
pub struct InMemoryBooksRepository(pub Vec<Book>);

impl Actor for InMemoryBooksRepository {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }
    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

// pub struct Add;
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
        self.add();
        Ok(true)
    }
}

pub struct SearchFromIsbn(pub Isbn);
impl Message for SearchFromIsbn {
    type Result = Option<Book>;
}
impl Handler<SearchFromIsbn> for InMemoryBooksRepository {
    type Result = Option<Book>;
    fn handle(&mut self, msg: SearchFromIsbn, _: &mut Context<Self>) -> Self::Result {
        super::bookshelf::InMemoryBooksRepository::search_from_isbn(msg.0)
    }
}

pub struct Search(pub String);
impl Message for Search {
    type Result = Result<Vec<Book>, io::Error>;
}
impl Handler<Search> for InMemoryBooksRepository {
    type Result = Result<Vec<Book>, io::Error>;
    fn handle(&mut self, msg: Search, _: &mut Context<Self>) -> Self::Result {
        Ok(self.search(msg.0))
    }
}

pub struct Last;
impl Message for Last {
    type Result = Option<Book>;
}
impl Handler<Last> for InMemoryBooksRepository {
    type Result = Option<Book>;
    fn handle(&mut self, msg: Last, _: &mut Context<Self>) -> Self::Result {
        self.last()
    }
}

impl InMemoryBooksRepository {
    pub fn new() -> Self {
        InMemoryBooksRepository(vec![
            Book {
                id: "1".to_owned(),
                name: "a".to_owned(),
                page: 100,
                page_in_progress: Some(1),
                isbn: Isbn::new(9780000000000).expect("invalid isbn"),
            },
            Book {
                id: "2".to_owned(),
                name: "b".to_owned(),
                page: 200,
                page_in_progress: Some(2),
                isbn: Isbn::new(9780000000001).expect("invalid isbn"),
            },
        ])
    }
    pub fn search_from_isbn_future(isbn: Isbn) -> impl futures::future::Future {
        // not lookup
        let client = AsyncClient::new();
        client
            .get(Url::parse("https://www.googleapis.com/books/v1/volumes?q=isbn:9784797321944").unwrap())
            .send()

        // let res = client
        //     .get(Url::parse("https://www.googleapis.com/books/v1/volumes?q=isbn:9784797321944").unwrap())
        //     .send();
        // res.and_then(|mut result| result.json::<Volumes>())
        //     .map(|volumes| volumes.items.into_iter().map(|v| v.into()))
    }
    pub fn find_by_isbn(&self, isbn: Isbn) -> Option<&Book> {
        self.0.iter().find(|book| book.isbn == isbn)
    }
    pub fn search_from_isbn(isbn: Isbn) -> Option<Book> {
        // not lookup
        let client = AsyncClient::new();
        let res = client
            .get(Url::parse("https://www.googleapis.com/books/v1/volumes?q=isbn:9784797321944").unwrap())
            .send();
        let json = res.and_then(|mut result| result.json::<Volumes>());
        let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");

        let result = rt.block_on(json);
        // let result = res.wait();
        match result {
            Ok(res) => println!("{:?}", res),
            Err(e) => println!("{:?}", e),
        }
        // println!("{:?}", result);
        Some(Book {
            id: "100".to_string(),
            name: "z".to_string(),
            page: 100,
            page_in_progress: None,
            isbn: isbn,
        })
    }
    fn search(&self, id: String) -> Vec<Book> {
        self.0.clone()
    }
    fn add(&mut self) -> bool {
        let next_id = match self.0.last() {
            Some(last) => last.id.parse::<i32>().expect("id must be num") + 1,
            None => 1,
        };
        self.0.push(Book {
            id: next_id.to_string(),
            name: "c".to_owned(),
            page: 300,
            page_in_progress: Some(3),
            isbn: Isbn::new(9780000000000).expect("invalid isbn"),
        });
        true
    }
    fn last(&self) -> Option<Book> {
        self.0.last().and_then(|book| Some(book.clone()))
    }
}
struct FindByIsbnFuture(Isbn);
impl Future for FindByIsbnFuture {
    type Item = Option<Book>;
    type Error = ();
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready(InMemoryBooksRepository::search_from_isbn(self.0)))
    }
}

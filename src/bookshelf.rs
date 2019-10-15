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
#[derive(Clone, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
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
impl ToString for Isbn {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub struct InMemoryBooksRepository(pub Vec<Book>);

impl InMemoryBooksRepository {
    // fn my_search(isbn: Isbn) -> impl Future<Item = Book, Error = Error> {}
    // fn double_search(isbn: Isbn) -> impl Future<Item = (Book, Book), Error = ()> {
    //     let client = AsyncClient::new();
    //     let request1 = client
    //         .get(Url::parse("https://www.googleapis.com/books/v1/volumes?q=isbn:9784797321944").unwrap())
    //         .send()
    //         .and_then(|mut response| response.json::<Volumes>());
    //     let request2 = client
    //         .get(Url::parse("https://www.googleapis.com/books/v1/volumes?q=isbn:9784797321944").unwrap())
    //         .send()
    //         .and_then(|mut response| response.json::<Volumes>());
    //     request1
    //         .join(request2)
    //         .map(|(res1, res2)| {
    //             let vol1 = res1
    //                 .items
    //                 .into_iter()
    //                 .map(|v| Book::try_from(v))
    //                 .collect::<Vec<Result<Book, Error>>>()
    //                 .first()
    //                 .unwrap()
    //                 .unwrap();
    //             let vol2 = res2
    //                 .items
    //                 .into_iter()
    //                 .map(|v| Book::try_from(v))
    //                 .collect::<Vec<Result<Book, Error>>>()
    //                 .first()
    //                 .unwrap()
    //                 .unwrap();
    //             // println!("{:?}", res1);
    //             // println!("{:?}", res2);
    //             (vol1, vol2)
    //         })
    //         .map_err(|err| {
    //             println!("stdout error: {}", err);
    //         })
    // }
    //
    // fn do_search(isbn: Isbn) -> Result<(Book, Book), ()> {
    //     let mut runtime = tokio::runtime::Runtime::new().unwrap();
    //     runtime.block_on(Self::double_search(isbn))
    //     // tokio::runtime::run(Self::double_search(isbn))
    // }
    fn search_from_isbn(isbn: Isbn) -> impl futures::future::Future<Item = Book, Error = reqwest::Error> {
        dbg!("5");
        // not lookup
        let client = AsyncClient::new();
        client
            .get(Url::parse("https://www.googleapis.com/books/v1/volumes?q=isbn:9784797321944").unwrap())
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
        //     .map(|volumes| volumes.items.into_iter().map(|v| v.into()))

        // let res = client
        //     .get(Url::parse("https://www.googleapis.com/books/v1/volumes?q=isbn:9784797321944").unwrap())
        //     .send();
        // res.and_then(|mut result| result.json::<Volumes>())
        //     .map(|volumes| volumes.items.into_iter().map(|v| v.into()))
    }
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
        // self.add();
        Ok(true)
    }
}

pub struct Hoge;
impl Message for Hoge {
    type Result = Result<i32, ()>;
}
impl Handler<Hoge> for InMemoryBooksRepository {
    type Result = ResponseFuture<i32, ()>;
    fn handle(&mut self, msg: Hoge, _: &mut Context<Self>) -> Self::Result {
        FutureOk(1).boxed()
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
    // type Result = Box<dyn Future<Item = BookAndLocation, Error = ReqwestError>>;
}
impl Handler<SearchFromIsbn> for InMemoryBooksRepository {
    // type Result = Result<BookAndLocation, ReqwestError>;
    // type Result = Box<dyn Future<Item = BookAndLocation, Error = ReqwestError>>;
    // type Result = impl Future<Future<Item = BookAndLocation, Error = ReqwestError>>;
    type Result = ResponseFuture<BookAndLocation, ReqwestError>;
    fn handle(&mut self, msg: SearchFromIsbn, _: &mut Context<Self>) -> Self::Result {
        // super::bookshelf::InMemoryBooksRepository::search_from_isbn(msg.0)
        dbg!("2");
        // let api = InMemoryBooksRepository::search_from_isbn(msg.0);
        // let data = api.map(|api| (api, BookInfoLocation::Network));
        // let (tx, rx) = tokio::sync::mpsc::channel(1);
        // let (tx, rx) = futures::sync::oneshot::channel();
        // Arbiter::spawn(
        //     api.map(|api| {
        //         tx.send(api.name.clone());
        //         println!("{:?}", api)
        //     })
        //     .map_err(|e| println!("not spawned")),
        // );
        // Arbiter::spawn(
        //     rx.map(|rec| println!("got value: {}", rec))
        //         .map_err(|e| println!("rx spawn error")),
        // );
        // rx.for_each(|value| {
        //     println!("got value: {}", value);
        //     Ok(())
        // });
        dbg!("3");
        let inmemory: FutureResult<Option<Book>, _> = FutureOk(self.find_by_isbn(msg.0));
        // let data = inmemory.map(|inmemory| (inmemory.unwrap(), BookInfoLocation::InMemory));
        dbg!("4");
        let api = InMemoryBooksRepository::search_from_isbn(msg.0);
        let pair = api.join(inmemory);
        let data = pair.map(|(netw, inme)| {
            dbg!("8");
            if let Some(inmemory_book) = inme {
                dbg!("8.5");
                (inmemory_book.clone(), BookInfoLocation::InMemory)
            } else {
                dbg!("8.6");
                (netw, BookInfoLocation::Network)
            }
        });
        // rx.wait()
        //     .map(|rec| println!("got value: {}", rec))
        //     .map_err(|e| println!("rx spawn error"));
        // dbg!("40");
        // data.wait()
        data.boxed()
        // let mut runtime = tokio::runtime::Runtime::new().unwrap();
        // dbg!("6");
        // runtime.block_on(data)
        // tokio::runtime::run(
        //     api.join(inmemory)
        //         .map(|(res1, res2)| {
        //             dbg!("6");
        //         })
        //         .map_err(|err| {
        //             println!("err");
        //         }),
        // );
        // let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");
        // match rt.block_on(pair) {
        //     Err(err) => panic!(err),
        //     Ok(mut result) => {
        //         dbg!("6");
        //         if result.1.is_some() {
        //             let book = result.1.unwrap();
        //             return Ok((book.clone(), BookInfoLocation::InMemory));
        //         } else {
        //             let volumes = result
        //                 .0
        //                 // .and_then(|mut result| result.json::<Volumes>())
        //                 .json::<Volumes>()
        //                 .wait()?;
        //             let book = volumes
        //                 .items
        //                 .into_iter()
        //                 .map(|v| Book::try_from(v))
        //                 .collect::<Vec<Result<Book, Error>>>()
        //                 .pop();
        //             if let Some(book) = book {
        //                 // let b = book?;
        //                 return Ok((book?, BookInfoLocation::Network));
        //             } else {
        //                 return Err(format_err!(""));
        //             }
        //             // .items?;
        //             // .map(|volumes| volumes.items.into_iter().map(|v| v.into()))
        //             // .wait();
        //         }
        //     }
        // }
        // Err(format_err!(""))
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
                isbn: Isbn::new(9784797321943).expect("invalid isbn"),
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
    // pub fn search_from_isbn(isbn: Isbn) -> Option<Book> {
    //     // not lookup
    //     let client = AsyncClient::new();
    //     let res = client
    //         .get(
    //             Url::parse(
    //                 format!(
    //                     "https://www.googleapis.com/books/v1/volumes?q=isbn:{}",
    //                     isbn.to_string()
    //                 )
    //                 .as_str(),
    //             )
    //             .unwrap(),
    //         )
    //         .send();
    //     let json = res.and_then(|mut result| result.json::<Volumes>());
    //     let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");
    //
    //     let result = rt.block_on(json);
    //     // let result = res.wait();
    //     match result {
    //         Ok(res) => println!("{:?}", res),
    //         Err(e) => println!("{:?}", e),
    //     }
    //     // println!("{:?}", result);
    //     Some(Book {
    //         id: "100".to_string(),
    //         name: "z".to_string(),
    //         page: 100,
    //         page_in_progress: None,
    //         isbn: isbn,
    //     })
    // }
    fn search(&self, id: String) -> Vec<Book> {
        self.0.clone()
    }
    // fn add(&mut self) -> bool {
    //     let next_id = match self.0.last() {
    //         Some(last) => last.id.parse::<i32>().expect("id must be num") + 1,
    //         None => 1,
    //     };
    //     self.0.push(Book {
    //         id: next_id.to_string(),
    //         name: "c".to_owned(),
    //         page: 300,
    //         page_in_progress: Some(3),
    //         isbn: Isbn::new(9780000000000).expect("invalid isbn"),
    //     });
    //     true
    // }
    fn last(&self) -> Option<Book> {
        self.0.last().and_then(|book| Some(book.clone()))
    }
}
// struct FindByIsbnFuture(Isbn);
// impl Future for FindByIsbnFuture {
//     type Item = Option<Book>;
//     type Error = ();
//     fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
//         Ok(Async::Ready(InMemoryBooksRepository::search_from_isbn(self.0)))
//     }
// }

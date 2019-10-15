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
trait BooksRepository {
    fn find_by_isbn(&self, isbn: Isbn) -> Option<Book>;
    fn add(&mut self, book: Book) -> bool;
    fn latest(&self) -> Option<&Book>;
    fn delete(&mut self, isbn: Isbn) -> bool;
    fn find_by_id(&self, id: u32) -> Option<&Book>;
}

pub struct InMemoryBooksRepository(pub Vec<Book>);

impl BooksRepository for InMemoryBooksRepository {
    fn find_by_isbn(&self, isbn: Isbn) -> Option<Book> {
        self.0.iter().find(|book| book.isbn() == isbn).map(|book| book.clone())
    }
    fn add(&mut self, book: Book) -> bool {
        self.0.push(book);
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
impl InMemoryBookmarksRepository {
    pub fn new() -> Self {
        InMemoryBookmarksRepository(Vec::new())
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

trait BookmarksRepository {
    fn progress(
        &mut self,
        user_id: u32,
        book_id: u32,
        page_in_progress: u16,
        users_repository: &dyn UsersRepository,
        books_repository: &dyn BooksRepository,
    ) -> Result<(), ProgressBookmarkRepositoryError>;
}
pub struct InMemoryBookmarksRepository(Vec<Bookmark>);
impl BookmarksRepository for InMemoryBookmarksRepository {
    fn progress(
        &mut self,
        user_id: u32,
        book_id: u32,
        page_in_progress: u16,
        users_repository: &dyn UsersRepository,
        books_repository: &dyn BooksRepository,
    ) -> Result<(), ProgressBookmarkRepositoryError> {
        if users_repository.find_by_id(user_id).is_none() {
            return Err(ProgressBookmarkRepositoryError::UserNotFoundError(user_id).into());
        }
        if books_repository.find_by_id(book_id).is_none() {
            return Err(ProgressBookmarkRepositoryError::BookNotFoundError(book_id).into());
        }
        if let Some(bookmark) = self
            .0
            .iter_mut()
            .find(|bookmark| bookmark.user_id == user_id && bookmark.book_id == book_id)
        {
            bookmark.page_in_progress = page_in_progress
        } else {
            let latest_bookmark = self.0.iter().max_by_key(|bookmark| bookmark.id);
            let bookmark = if let Some(latest_bookmark) = latest_bookmark {
                Bookmark {
                    id: latest_bookmark.id + 1,
                    user_id,
                    book_id,
                    page_in_progress,
                }
            } else {
                Bookmark {
                    id: 1,
                    user_id,
                    book_id,
                    page_in_progress,
                }
            };
            self.0.push(bookmark)
        }
        Ok(())
    }
}
#[derive(Fail, Debug)]
enum ProgressBookmarkRepositoryError {
    #[fail(display = "User Not Found")]
    UserNotFoundError(u32),
    #[fail(display = "Book Not Found")]
    BookNotFoundError(u32),
    #[fail(display = "page_cuont max is {}, but entered {}", _1, _0)]
    PageCountOverFlowError(u16, u16),
}
struct Bookmark {
    id: u64,
    user_id: u32,
    book_id: u32,
    page_in_progress: u16,
}

trait UsersRepository {
    fn add(&mut self, email: String, password: String) -> Result<(), AddUserRepositoryError>;
    fn find_by_user_info(&self, email: String, password: String) -> Option<&User>;
    fn find_by_session(&self, session_id: String) -> Option<&User>;
    fn find_by_id(&self, user_id: u32) -> Option<&User>;
}
pub struct InMemoryUsersRepository(Vec<User>);
impl UsersRepository for InMemoryUsersRepository {
    fn add(&mut self, email: String, password: String) -> Result<(), AddUserRepositoryError> {
        if self.0.iter().find(|user| user.email == email).is_some() {
            Err(AddUserRepositoryError::DuplicatedUserError(email).into())
        } else {
            let latest_user = self.0.iter().max_by_key(|user| user.id);
            let user = if let Some(latest_user) = latest_user {
                User {
                    id: latest_user.id + 1,
                    email,
                    password,
                    session_id: "".to_string(),
                }
            } else {
                User {
                    id: 1,
                    email,
                    password,
                    session_id: "".to_string(),
                }
            };
            self.0.push(user);
            Ok(())
        }
    }
    fn find_by_user_info(&self, email: String, password: String) -> Option<&User> {
        self.0
            .iter()
            .find(|user| user.email == email && user.password == password)
    }
    fn find_by_session(&self, session_id: String) -> Option<&User> {
        self.0.iter().find(|user| user.session_id == session_id)
    }
    fn find_by_id(&self, user_id: u32) -> Option<&User> {
        self.0.iter().find(|user| user.id == user_id)
    }
}
struct User {
    id: u32,
    email: String,
    password: String,
    session_id: String,
}

#[derive(Fail, Debug)]
enum AddUserRepositoryError {
    #[fail(display = "the email address have been already taken: {}", _0)]
    DuplicatedUserError(String),
}
impl InMemoryUsersRepository {
    pub fn new() -> Self {
        InMemoryUsersRepository(Vec::new())
    }
}

// Actor
impl Actor for InMemoryBooksRepository {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("BooksRepository Actor is alive");
    }
    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("BooksRepository Actor is stopped");
    }
}
impl Actor for InMemoryUsersRepository {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("UsersRepository Actor is alive");
    }
    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("UsersRepository Actor is stopped");
    }
}
impl Actor for InMemoryBookmarksRepository {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("BookmarksRepository Actor is alive");
    }
    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("BookmarksRepository Actor is stopped");
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
        data.boxed()
    }
}

impl InMemoryBooksRepository {
    pub fn new() -> Self {
        InMemoryBooksRepository(vec![
            Book {
                id: 1,
                info: BookInfo {
                    title: "a".to_owned(),
                    page_count: 100,
                    isbn: Isbn::new(9784797321943).expect("invalid isbn"),
                },
            },
            Book {
                id: 2,
                info: BookInfo {
                    title: "b".to_owned(),
                    page_count: 200,
                    isbn: Isbn::new(9780000000001).expect("invalid isbn"),
                },
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
//   user
pub struct AddUser {
    pub email: String,
    pub password: String,
}

impl Message for AddUser {
    type Result = Result<bool, io::Error>;
}

impl Handler<AddUser> for InMemoryUsersRepository {
    type Result = Result<bool, io::Error>;
    fn handle(&mut self, msg: AddUser, _ctx: &mut Context<Self>) -> Self::Result {
        println!("hadle Add");
        self.add(msg.email, msg.password);
        Ok(true)
    }
}

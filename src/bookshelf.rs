use super::schema::Isbn as graphqlIsbn;
use actix::prelude::*;
use failure::Error;
use std::io;

#[derive(GraphQLObject, Clone)]
pub struct Book {
    id: String,
    name: String,
    page: i32,
    page_in_progress: Option<i32>,
}

#[derive(Fail, Debug)]
pub enum IsbnError {
    #[fail(display = "ISBN must be between 9780000000000 - 9799999999999: {}", _0)]
    RangeError(u64),
}
pub struct Isbn(u64);
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

pub struct BookRepository(pub Vec<Book>);

impl Actor for BookRepository {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }
    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

pub struct Add;
impl Message for Add {
    type Result = Result<bool, io::Error>;
}

impl Handler<Add> for BookRepository {
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
impl Handler<SearchFromIsbn> for BookRepository {
    type Result = Option<Book>;
    fn handle(&mut self, msg: SearchFromIsbn, _: &mut Context<Self>) -> Self::Result {
        super::bookshelf::BookRepository::search_from_isbn(msg.0)
    }
}

pub struct Search(pub String);
impl Message for Search {
    type Result = Result<Vec<Book>, io::Error>;
}
impl Handler<Search> for BookRepository {
    type Result = Result<Vec<Book>, io::Error>;
    fn handle(&mut self, msg: Search, _: &mut Context<Self>) -> Self::Result {
        Ok(self.search(msg.0))
    }
}

pub struct Last;
impl Message for Last {
    type Result = Option<Book>;
}
impl Handler<Last> for BookRepository {
    type Result = Option<Book>;
    fn handle(&mut self, msg: Last, _: &mut Context<Self>) -> Self::Result {
        self.last()
    }
}

impl BookRepository {
    pub fn new() -> Self {
        BookRepository(vec![
            Book {
                id: "1".to_owned(),
                name: "a".to_owned(),
                page: 100,
                page_in_progress: Some(1),
            },
            Book {
                id: "2".to_owned(),
                name: "b".to_owned(),
                page: 200,
                page_in_progress: Some(2),
            },
        ])
    }
    pub fn search_from_isbn(isbn: Isbn) -> Option<Book> {
        // not lookup
        Some(Book {
            id: "100".to_string(),
            name: "z".to_string(),
            page: 100,
            page_in_progress: None,
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
        });
        true
    }
    fn last(&self) -> Option<Book> {
        self.0.last().and_then(|book| Some(book.clone()))
    }
}

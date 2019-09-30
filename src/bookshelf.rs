use actix::prelude::*;

pub struct Book {
    id: String,
    name: String,
    page: i32,
    page_in_progress: Option<i32>,
}

pub struct Bookshelf(pub Vec<Book>);

impl Actor for Bookshelf {
    type Context = Context<Self>;
}

pub struct Add(Vec<Book>);
impl Message for Add {
    type Result = bool;
}

pub struct Search(String);
impl Message for Search {
    type Result = Option<Book>;
}
impl Handler<Search> for Bookshelf {
    type Result = Option<Book>;
    fn handle(&mut self, msg: Search, _: &mut Context<Self>) -> Self::Result {
        self.search()
    }
}

impl Bookshelf {
    pub fn new() -> Self {
        Bookshelf(vec![
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
    fn search(&self) -> Option<Book> {
        Some(Book {
            id: "1".to_owned(),
            name: "a".to_owned(),
            page: 100,
            page_in_progress: Some(1),
        })
    }
}

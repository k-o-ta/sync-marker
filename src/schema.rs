use super::bookshelf::Book as TBook;
use super::bookshelf::Isbn as TIsbn;
use super::bookshelf::{BookInfoLocation, InMemoryBooksRepository, IsbnError};
use actix::prelude::*;
use actix::Addr;
use futures::Future;
use juniper::FieldError;
use juniper::FieldResult;
use juniper::RootNode;
use std::convert::TryFrom;

pub struct Context {
    pub addr: Addr<InMemoryBooksRepository>,
}
impl juniper::Context for Context {}

#[derive(GraphQLObject, Clone)]
pub struct Isbn {
    code: String,
}

impl From<TIsbn> for Isbn {
    fn from(value: TIsbn) -> Self {
        Isbn {
            code: value.0.to_string(),
        }
    }
}

#[derive(GraphQLObject)]
pub struct Book {
    id: String,
    name: String,
    page: i32,
    isbn: Isbn,
    data_source: String,
}
impl TBook {
    fn into_graphql_book(self, data_source: String) -> Book {
        Book {
            id: self.id,
            name: self.title,
            page: self.page_count,
            isbn: Isbn::from(self.isbn),
            data_source,
        }
    }
}

// impl From<TBook> for Book {
//     fn from(item: TBook) -> Self {
//         Book {
//             id: item.id,
//             name: item.name,
//             page: item.page,
//             isbn: Isbn::from(item.isbn),
//
//         }
//     }
// }

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn book_from_isbn(context: &Context, isbn: String) -> FieldResult<(Book)> {
        dbg!("0");
        let isbn = TIsbn::try_from(isbn);
        if let Ok(isbn) = isbn {
            dbg!("1");
            let res_future = context.addr.send(super::bookshelf::SearchFromIsbn(isbn));
            dbg!("10");
            let res = res_future.wait();
            match res {
                Ok(res) => match res {
                    Ok(book_info) => {
                        dbg!("11");
                        dbg!("{:?}", &book_info.1);
                        return Ok(book_info.0.into_graphql_book(book_info.1.to_string()));
                    }
                    Err(err) => {
                        dbg!("12");
                        return Err(FieldError::new(
                            err.to_string(),
                            graphql_value!({"isbn_error": "isbn error"}),
                        ));
                    }
                },
                Err(err) => {
                    dbg!("13");
                    return Err(FieldError::new(
                        err.to_string(),
                        graphql_value!({"isbn_error": "isbn error"}),
                    ));
                }
            };
        };
        dbg!("14");
        Err(FieldError::new(
            "isbn error",
            graphql_value!({"isbn_error": "isbn error"}),
        ))
    }
}

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn createBook(context: &Context, title: String, page_count: i32, isbn: String) -> FieldResult<bool> {
        let isbn = TIsbn::try_from(isbn);
        if let Ok(isbn) = isbn {
            let res_future = context.addr.send(super::bookshelf::Add {
                title,
                page_count,
                isbn,
            });
            let res = res_future.wait();
            match res {
                Ok(resutl) => {
                    return Ok(true);
                }
                Err(err) => {
                    println!("ng");
                    return Err(FieldError::new(err, graphql_value!({"isbn_error": "isbn error"})));
                }
            };
        }
        Err(FieldError::new(
            "isbn error",
            graphql_value!({"isbn_error": "isbn error"}),
        ))
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;
pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}

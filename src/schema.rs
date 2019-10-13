use super::bookshelf::Book as TBook;
use super::bookshelf::Isbn as TIsbn;
use super::bookshelf::{InMemoryBooksRepository, IsbnError};
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
}

impl From<TBook> for Book {
    fn from(item: TBook) -> Self {
        Book {
            id: item.id,
            name: item.name,
            page: item.page,
            isbn: Isbn::from(item.isbn),
        }
    }
}

// #[derive(GraphQLObject)]
// pub struct Isbn {
//     code: String,
// }
//
// impl From<IsbnDigits> for Isbn {
//     fn from(isbn: IsbnDigits) -> Self {
//         Isbn {
//             code: isbn.code().to_string(),
//         }
//     }
// }

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn book_from_isbn(context: &Context, isbn: String) -> FieldResult<Book> {
        let isbn = TIsbn::try_from(isbn);
        // return Ok(super::bookshelf::InMemoryBooksRepository::search_from_isbn(isbn.unwrap())
        //     .unwrap()
        //     .into());
        match isbn {
            Ok(isbn) => match InMemoryBooksRepository::search_from_isbn(isbn) {
                Some(book) => Ok(book.into()),
                None => Err(FieldError::new(
                    "no such book",
                    graphql_value!({"not_found_error": "book not found"}),
                )),
            },
            Err(err) => Err(FieldError::new(
                err.to_string(),
                graphql_value!({"isbn_error": "isbn error"}),
            )),
        }
        // let isbn = match isbn.parse::<u64>() {
        //     Ok(code) => match TIsbn::new(code) {
        //         Ok(isbn) => isbn,
        //         Err(err) => {
        //             return Err(FieldError::new(
        //                 err,
        //                 graphql_value!({"range_error": "ISBN range error"}),
        //             ))
        //         }
        //     },
        //     Err(err) => {
        //         return Err(FieldError::new(
        //             "ISBN must be 13 digit number",
        //             graphql_value!({"format_error": "ISBN parse error"}),
        //         ))
        //     }
        // };
        // match super::bookshelf::InMemoryBooksRepository::search_from_isbn(isbn) {
        //     Some(book) => Ok(book),
        //     None => {
        //         return Err(FieldError::new(
        //             "no such book",
        //             graphql_value!({"not_found_error": "book not found"}),
        //         ))
        //     }
        // }
    }

    fn books(context: &Context, user_id: String) -> FieldResult<Vec<Book>> {
        let res_future = context.addr.send(super::bookshelf::Search(user_id));
        let res = res_future.wait();
        match res {
            Ok(result) => match result {
                Ok(result) => Ok(result.into_iter().map(|book| book.into()).collect()),
                Err(err) => panic!("{}", err),
            },
            Err(err) => panic!("{}", err),
        }
        // Ok(vec![
        //     Book {
        //         id: "1".to_owned(),
        //         name: "a".to_owned(),
        //         page: 100,
        //         page_in_progress: Some(1),
        //     },
        //     Book {
        //         id: "2".to_owned(),
        //         name: "b".to_owned(),
        //         page: 200,
        //         page_in_progress: Some(2),
        //     },
        // ])
    }
}

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn createBook(context: &Context, title: String, page_count: i32, isbn: String) -> FieldResult<Book> {
        let isbn = TIsbn::try_from(isbn);
        if let Ok(isbn) = isbn {
            let res_future = context.addr.send(super::bookshelf::Add {
                title,
                page_count,
                isbn,
            });
            let res = res_future.wait();
            match res {
                Ok(resutl) => println!("ok"),
                Err(err) => println!("ng"),
            };
            let res_last = context.addr.send(super::bookshelf::Last);
            let res = res_last.wait();
            match res {
                Ok(result) => return Ok(result.expect("cannot fetch last book").into()),
                Err(err) => panic!("{}", err),
            }
        };
        Err(FieldError::new(
            "isbn error",
            graphql_value!({"isbn_error": "isbn error"}),
        ))
        // let res_future = context.addr.send(super::bookshelf::Add);
        // Arbiter::spawn({
        //     println!("spawn1");
        //     res_future
        //         .map(|res| {
        //             println!("spawn2");
        //             match res {
        //                 Ok(result) => println!("ok"),
        //                 Err(err) => println!("no"),
        //             }
        //         })
        //         .map_err(|e| {
        //             println!("Actor is probably dead: {}", e);
        //         })
        // });

        // Ok(super::bookshelf::Book {
        //     id: "1".to_owned(),
        //     name: "a".to_owned(),
        //     page: 100,
        //     page_in_progress: Some(1),
        // })
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;
pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}

use super::bookshelf::Isbn as IsbnDigits;
use super::bookshelf::IsbnError;
use actix::prelude::*;
use actix::Addr;
use futures::Future;
use juniper::FieldError;
use juniper::FieldResult;
use juniper::RootNode;

pub struct Context {
    pub addr: Addr<super::bookshelf::BookRepository>,
}
impl juniper::Context for Context {}

// #[derive(GraphQLObject)]
// struct Book {
//     id: String,
//     name: String,
//     page: i32,
//     page_in_progress: Option<i32>,
// }

#[derive(GraphQLObject)]
pub struct Isbn {
    code: String,
}

impl From<IsbnDigits> for Isbn {
    fn from(isbn: IsbnDigits) -> Self {
        Isbn {
            code: isbn.code().to_string(),
        }
    }
}

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn book_from_isbn(context: &Context, isbn: String) -> FieldResult<super::bookshelf::Book> {
        let isbn = match isbn.parse::<u64>() {
            Ok(code) => match IsbnDigits::new(code) {
                Ok(isbn) => isbn,
                Err(IsbnError::RangeError) => {
                    return Err(FieldError::new(
                        "ISBN must be between 9780000000000 - 9799999999999",
                        graphql_value!({"range_error": "ISBN range error"}),
                    ))
                }
            },
            Err(err) => {
                return Err(FieldError::new(
                    "ISBN must be 13 digit number",
                    graphql_value!({"format_error": "ISBN parse error"}),
                ))
            }
        };
        match super::bookshelf::BookRepository::search_from_isbn(isbn) {
            Some(book) => Ok(book),
            None => {
                return Err(FieldError::new(
                    "no such book",
                    graphql_value!({"not_found_error": "book not found"}),
                ))
            }
        }
        // let isbn = IsbnDigits::
    }

    fn books(context: &Context, user_id: String) -> FieldResult<Vec<super::bookshelf::Book>> {
        let res_future = context.addr.send(super::bookshelf::Search(user_id));
        let res = res_future.wait();
        match res {
            Ok(result) => match result {
                Ok(result) => Ok(result),
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
    fn createBook(context: &Context, book_id: String) -> FieldResult<super::bookshelf::Book> {
        let res_future = context.addr.send(super::bookshelf::Add);
        let res = res_future.wait();
        match res {
            Ok(resutl) => println!("ok"),
            Err(err) => println!("ng"),
        };
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
        let res_last = context.addr.send(super::bookshelf::Last);
        let res = res_last.wait();
        match res {
            Ok(result) => Ok(result.expect("cannot fetch last book")),
            Err(err) => panic!("{}", err),
        }

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

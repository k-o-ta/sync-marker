use super::bookshelf::BookInfo as TBookInfo;
use super::bookshelf::Isbn as TIsbn;
use super::bookshelf::{
    BookInfoLocation, InMemoryBookmarksRepository, InMemoryBooksRepository, InMemoryUsersRepository, IsbnError,
};
use super::session::InMemorySessionsRepository;
use actix::prelude::*;
use actix::Addr;
use actix_session::Session;
use futures::Future;
use juniper::FieldError;
use juniper::FieldResult;
use juniper::RootNode;
use std::convert::TryFrom;
use std::sync::Arc;

pub struct Context {
    pub books_repository_addr: Addr<InMemoryBooksRepository>,
    pub users_repository_addr: Addr<InMemoryUsersRepository>,
    pub bookmarks_repository_addr: Addr<InMemoryBookmarksRepository>,
    pub sessions_repository_addr: Addr<InMemorySessionsRepository>,
    // session: Arc<Session>,
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
    name: String,
    page: i32,
    isbn: Isbn,
    data_source: String,
}
impl TBookInfo {
    fn into_graphql_book(self, data_source: String) -> Book {
        Book {
            name: self.title,
            page: self.page_count,
            isbn: Isbn::from(self.isbn),
            data_source,
        }
    }
}

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn book_from_isbn(context: &Context, isbn: String) -> FieldResult<(Book)> {
        let isbn = TIsbn::try_from(isbn);
        if let Ok(isbn) = isbn {
            let res_future = context
                .books_repository_addr
                .send(super::bookshelf::SearchFromIsbn(isbn));
            let res = res_future.wait();
            match res {
                Ok(res) => match res {
                    Ok(book_info) => {
                        return Ok(book_info.0.into_graphql_book(book_info.1.to_string()));
                    }
                    Err(err) => {
                        return Err(FieldError::new(
                            err.to_string(),
                            graphql_value!({"create_error": "create error"}),
                        ));
                    }
                },
                Err(err) => {
                    return Err(FieldError::new(
                        err.to_string(),
                        graphql_value!({"create_error": "create error"}),
                    ));
                }
            };
        };
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
            let res_future = context.books_repository_addr.send(super::bookshelf::Add {
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
    fn createUser(context: &Context, email: String, password: String) -> FieldResult<bool> {
        let res_future = context
            .users_repository_addr
            .send(super::bookshelf::AddUser { email, password });
        let res = res_future.wait();
        match res {
            Ok(result) => return Ok(true),
            Err(err) => {
                return Err(FieldError::new(
                    err,
                    graphql_value!({"create_user": "create user error"}),
                ));
            }
        }
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;
pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}

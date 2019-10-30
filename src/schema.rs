use super::bookmark::{Bookmark as TBookmark, InMemoryBookmarksRepository, Progress};
use super::bookshelf::InMemoryBooksRepository;
use super::bookshelf::Isbn as TIsbn;
use super::bookshelf::{Book as TBook, BookInfo as TBookInfo};
use super::session::{Add as AddSessionDigest, FindUserId, InMemorySessionsRepository, SessionDigest};
use super::user::{AddUser, FindBooks, FindByUserInfo, InMemoryUsersRepository};
use actix::Addr;
use failure;
use futures::future;
use futures::{future::Either, Future};
use juniper::FieldError;
use juniper::FieldResult;
use juniper::RootNode;
use std::cell::RefCell;
use std::convert::TryFrom;

pub struct Context {
    pub books_repository_addr: Addr<InMemoryBooksRepository>,
    pub users_repository_addr: Addr<InMemoryUsersRepository>,
    pub bookmarks_repository_addr: Addr<InMemoryBookmarksRepository>,
    pub sessions_repository_addr: Addr<InMemorySessionsRepository>,
    pub session_digest: RefCell<(Option<SessionDigest>, bool)>, // pub session_digest: Option<String>,
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

#[derive(GraphQLObject)]
pub struct Bookmark {
    id: i32,
    title: String,
    page_count: i32,
    isbn: Isbn,
    page_in_progress: i32,
}
impl Bookmark {
    fn new(book: TBook, bookmark: TBookmark) -> Result<Self, failure::Error> {
        if book.id != bookmark.book_id {
            return Err(format_err!("internal error book_id didn't match"));
        } else {
            Ok(Bookmark {
                id: bookmark.id as i32,
                title: book.title().to_string(),
                page_count: book.page_count(),
                isbn: Isbn::from(book.isbn()),
                page_in_progress: bookmark.page_in_progress as i32,
            })
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
    fn logged_in(context: &Context) -> FieldResult<bool> {
        if let Some(session_digest) = context.session_digest.borrow().0 {
            let session_fut = context.sessions_repository_addr.send(FindUserId(session_digest));
            let session = session_fut.wait();
            match session {
                Ok(result) => Ok(result.is_some()),
                Err(err) => Err(FieldError::new(
                    "session error",
                    graphql_value!({"session_error": "cannot get session_digest2"}),
                )),
            }
        // Ok(true)
        } else {
            Err(FieldError::new(
                "session error",
                graphql_value!({"session_error": "cannot get session_digest1"}),
            ))
        }
    }
    fn bookmarks(context: &Context) -> FieldResult<Vec<Bookmark>> {
        if let Some(session_digest) = context.session_digest.borrow().0 {
            let session = context
                .sessions_repository_addr
                .send(FindUserId(session_digest))
                .map_err(|_| "error1");
            let books = session.then(|user_id| {
                let user_id2 = user_id.map_err(|err| "error2").and_then(|user_id| match user_id {
                    Some(user_id) => Ok(user_id),
                    None => Err("error3"),
                });
                match user_id2 {
                    Ok(user_id) => Either::A(
                        context
                            .users_repository_addr
                            .send(FindBooks {
                                bookmarks_repository: context.bookmarks_repository_addr.clone(),
                                books_repository: context.books_repository_addr.clone(),
                                user_id,
                            })
                            .map_err(|_| "error5"),
                    ),
                    Err(err) => Either::B(future::err("erro4")),
                }
            });
            Ok(books
                .wait()
                .unwrap()
                .unwrap()
                .into_iter()
                .map(|book| Bookmark::new(book.0, book.1).unwrap())
                .collect())
        } else {
            Err(FieldError::new(
                "session error",
                graphql_value!({"session_error": "cannot get session_digest1"}),
            ))
        }
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
        let res_future = context.users_repository_addr.send(AddUser { email, password });
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
    fn login(mut context: &Context, email: String, password: String) -> FieldResult<bool> {
        let res_future = context.users_repository_addr.send(FindByUserInfo { email, password });
        let res = res_future.wait();
        match res {
            Ok(res) => match res {
                Some(res) => {
                    use rand::{thread_rng, Rng};
                    let mut arr = [0u8; 20];
                    thread_rng().fill(&mut arr[..]);
                    let mut my_ref = context.session_digest.borrow_mut();
                    *my_ref = (Some(arr.clone()), true);

                    let res_session_future = context.sessions_repository_addr.send(AddSessionDigest {
                        session_digest: arr,
                        user_id: res.id,
                    });
                    match res_session_future.wait() {
                        Ok(session) => Ok(true),
                        Err(e) => {
                            return Err(FieldError::new(e, graphql_value!({"login": "login_error"})));
                        }
                    }
                }
                None => Ok(false),
            },
            Err(err) => {
                return Err(FieldError::new(err, graphql_value!({"login": "login_error"})));
            }
        }
    }

    fn progress(context: &Context, isbn: String, page_count: i32) -> FieldResult<bool> {
        dbg!("1");
        let isbn = TIsbn::try_from(isbn);
        let isbn = if let Ok(isbn) = isbn {
            isbn
        } else {
            return Err(FieldError::new(
                "isbn error",
                graphql_value!({"progress error": "isbn error"}),
            ));
        };
        dbg!("2");
        let page_in_progress = if ((std::u16::MIN as i32)..=(std::u16::MAX as i32)).contains(&page_count) {
            page_count as u16
        } else {
            return Err(FieldError::new(
                "page range error",
                graphql_value!({"progress": "progress_error"}),
            ));
        };
        dbg!("3");
        let session_digest = if let Some(session_digest) = context.session_digest.borrow().0 {
            session_digest
        } else {
            return Err(FieldError::new(
                "session digest error",
                graphql_value!({"progress": "session_digest_error"}),
            ));
        };

        dbg!("4");
        let res_future = context.bookmarks_repository_addr.send(Progress {
            isbn,
            page_in_progress,
            session_digest,
            sessions_repository: context.sessions_repository_addr.clone(),
            users_repository: context.users_repository_addr.clone(),
            books_repository: context.books_repository_addr.clone(),
        });
        dbg!("5");
        let res = res_future.wait();
        match res {
            Ok(res) => match res {
                Ok(res) => Ok(true),
                Err(err) => {
                    return Err(FieldError::new(
                        "progress error",
                        graphql_value!({"progress": "progress_error"}),
                    ));
                }
            },
            Err(err) => {
                return Err(FieldError::new(
                    "mail box error",
                    graphql_value!({"progress": "mail_box_error"}),
                ));
            }
        }
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;
pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}

use super::bookmark::{Bookmark, FindByUserId as FindBookmarksByUserId, InMemoryBookmarksRepository};

use super::bookshelf::{Book, FindById as FindBooksById, InMemoryBooksRepository};
use actix::prelude::*;
use std::io;
use std::io::{Error, ErrorKind};

pub trait UsersRepository {
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
        dbg!("14");
        self.0.iter().find(|user| user.id == user_id)
    }
}
#[derive(Clone)]
pub struct User {
    pub id: u32,
    email: String,
    password: String,
    session_id: String,
}

#[derive(Fail, Debug)]
pub enum AddUserRepositoryError {
    #[fail(display = "the email address have been already taken: {}", _0)]
    DuplicatedUserError(String),
}
impl InMemoryUsersRepository {
    pub fn new() -> Self {
        InMemoryUsersRepository(vec![User {
            id: 1,
            email: String::from("foo@example.com"),
            password: String::from("123abcdef"),
            session_id: String::from(""),
        }])
    }
}

impl Actor for InMemoryUsersRepository {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("UsersRepository Actor is alive");
    }
    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("UsersRepository Actor is stopped");
    }
}

// Message

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
pub struct FindByUserInfo {
    pub email: String,
    pub password: String,
}

impl Message for FindByUserInfo {
    type Result = Option<User>;
}

impl Handler<FindByUserInfo> for InMemoryUsersRepository {
    type Result = Option<User>;
    fn handle(&mut self, msg: FindByUserInfo, _ctx: &mut Context<Self>) -> Self::Result {
        self.find_by_user_info(msg.email, msg.password).map(|user| user.clone())
    }
}

pub struct FindById(pub u32);
impl Message for FindById {
    type Result = Option<User>;
}
impl Handler<FindById> for InMemoryUsersRepository {
    type Result = Option<User>;
    fn handle(&mut self, msg: FindById, _ctx: &mut Context<Self>) -> Self::Result {
        dbg!("13");
        self.find_by_id(msg.0).map(|user| user.clone())
    }
}

pub struct FindBooks {
    pub bookmarks_repository: Addr<InMemoryBookmarksRepository>,
    pub books_repository: Addr<InMemoryBooksRepository>,
    pub user_id: u32,
}
impl Message for FindBooks {
    type Result = Result<Vec<(Book, Bookmark)>, io::Error>;
}

impl Handler<FindBooks> for InMemoryUsersRepository {
    // type Result = Option<Vec<Book>>;
    type Result = ResponseFuture<Vec<(Book, Bookmark)>, io::Error>;
    fn handle(&mut self, msg: FindBooks, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            FindBooks {
                bookmarks_repository,
                books_repository,
                user_id,
            } => {
                let books = bookmarks_repository
                    .send(FindBookmarksByUserId(user_id))
                    .map_err(|e| Error::new(ErrorKind::Other, "oh no!"))
                    .and_then(move |bookmarks| {
                        let bookmarks2 = bookmarks.unwrap();
                        books_repository
                            .send(FindBooksById(
                                bookmarks2.iter().map(|bookmark| bookmark.book_id).collect(),
                            ))
                            .map(move |books| {
                                // println!("{:?}", bookmarks);
                                let mut books = books.unwrap();
                                books.sort_by_key(|book| book.id);
                                books.into_iter().zip(bookmarks2).collect()
                                // books.unwrap()
                            })
                            .map_err(|e| Error::new(ErrorKind::Other, "oh no!"))
                    });
                return Box::new(books);
            }
        }
    }
}

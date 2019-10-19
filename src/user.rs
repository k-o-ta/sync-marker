use super::session::SessionDigest;
use actix::prelude::*;
use std::io;
use tokio::prelude::*;

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
enum AddUserRepositoryError {
    #[fail(display = "the email address have been already taken: {}", _0)]
    DuplicatedUserError(String),
}
impl InMemoryUsersRepository {
    pub fn new() -> Self {
        InMemoryUsersRepository(Vec::new())
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
        self.find_by_id(msg.0).map(|user| user.clone())
    }
}

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

trait SessionsRepository {
    fn find_session(&self, session_digest: String) -> Option<&u32>;
    fn add_session(&mut self, session_digest: String, user_id: u32);
    fn remove_session(&mut self, session_digest: String) -> bool;
}

pub struct InMemorySessionsRepository(HashMap<String, u32>);

impl SessionsRepository for InMemorySessionsRepository {
    fn find_session(&self, session_digest: String) -> Option<&u32> {
        self.0.get(&session_digest)
    }

    fn add_session(&mut self, session_digest: String, user_id: u32) {
        self.0.insert(session_digest, user_id);
    }

    fn remove_session(&mut self, session_digest: String) -> bool {
        self.0.remove(&session_digest).is_some()
    }
}

impl Actor for InMemorySessionsRepository {
    type Context = Context<Self>;
    #[cfg(debug_assertions)]
    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("SessionRepository Actor is alive");
    }
    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("SessionsRepository Actor is stopped");
    }
}

impl InMemorySessionsRepository {
    pub fn new() -> Self {
        InMemorySessionsRepository(HashMap::new())
    }
}

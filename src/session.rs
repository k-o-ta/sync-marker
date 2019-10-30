use actix::prelude::*;
use std::collections::HashMap;
use std::io;

pub type SessionDigest = [u8; 20];
trait SessionsRepository {
    fn find_session(&self, session_digest: SessionDigest) -> Option<&u32>;
    fn add_session(&mut self, session_digest: SessionDigest, user_id: u32);
    fn remove_session(&mut self, session_digest: SessionDigest) -> bool;
}

pub struct InMemorySessionsRepository(HashMap<SessionDigest, u32>);

impl SessionsRepository for InMemorySessionsRepository {
    fn find_session(&self, session_digest: SessionDigest) -> Option<&u32> {
        self.0.get(&session_digest)
    }

    fn add_session(&mut self, session_digest: SessionDigest, user_id: u32) {
        self.0.insert(session_digest, user_id);
    }

    fn remove_session(&mut self, session_digest: SessionDigest) -> bool {
        self.0.remove(&session_digest).is_some()
    }
}

impl Actor for InMemorySessionsRepository {
    type Context = Context<Self>;
    #[cfg(debug_assertions)]
    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("SessionRepository Actor is alive");
    }
    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("SessionsRepository Actor is stopped");
    }
}

impl InMemorySessionsRepository {
    pub fn new() -> Self {
        InMemorySessionsRepository(HashMap::new())
    }
}

// message
pub struct Add {
    pub session_digest: [u8; 20],
    pub user_id: u32,
}

impl Message for Add {
    type Result = Result<(), io::Error>;
}

impl Handler<Add> for InMemorySessionsRepository {
    type Result = Result<(), io::Error>;
    fn handle(&mut self, msg: Add, _ctx: &mut Context<Self>) -> Self::Result {
        self.add_session(msg.session_digest, msg.user_id);
        Ok(())
    }
}

pub struct FindUserId(pub SessionDigest);
impl Message for FindUserId {
    type Result = Option<u32>;
}
impl Handler<FindUserId> for InMemorySessionsRepository {
    type Result = Option<u32>;
    fn handle(&mut self, msg: FindUserId, _ctx: &mut Context<Self>) -> Self::Result {
        self.find_session(msg.0).map(|user_id| user_id.clone())
    }
}

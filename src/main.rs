#[macro_use]
extern crate failure;

#[macro_use]
extern crate actix;

use actix::*;

use actix_session::{CookieSession, Session};
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use futures::future::Future;
use std::sync::Arc;

#[macro_use]
extern crate juniper;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod bookshelf;
mod schema;
mod session;

use crate::bookshelf::InMemoryBookmarksRepository;
use crate::bookshelf::InMemoryBooksRepository;
use crate::bookshelf::InMemoryUsersRepository;
use crate::schema::{create_schema, Schema};
use crate::session::InMemorySessionsRepository;

// use std::sync::Arc;

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}

fn graphql(
    // st: web::Data<Arc<Schema>>,
    state: web::Data<State>,
    data: web::Json<GraphQLRequest>,
    session: Session,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // let arc_session = Arc::new(session);
    web::block(move || {
        return match state.get_ref() {
            State {
                schema,
                books_repository_addr,
                users_repository_addr,
                bookmarks_repository_addr,
                sessions_repository_addr,
            } => {
                let ctx = schema::Context {
                    books_repository_addr: books_repository_addr.clone(),
                    users_repository_addr: users_repository_addr.clone(),
                    bookmarks_repository_addr: bookmarks_repository_addr.clone(),
                    sessions_repository_addr: sessions_repository_addr.clone(),
                    // session: arc_session.clone(),
                };
                let res = data.execute(&schema, &ctx);
                Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
            }
        };
        // let (schema, addr) = state;
        // let ctx = schema::Context { addr };
        // let ctx = schema::Context {
        //     addr: state.addr.clone(),
        // };
        // let res = data.execute(&st, &ctx);
        // let res = data.execute(&(state.schema), &ctx);
        // let res = data.execute(&schema, &ctx);
        // Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(move |user| {
        session.set("counter", 1);
        Ok(HttpResponse::Ok().content_type("application/json").body(user))
    })
}

fn index(session: Session) -> impl Responder {
    match session.get::<i32>("counter") {
        Ok(count) => match count {
            Some(count) => {
                println!("SESSION value: {}", count);
                session.set("counter", count + 1);
            }
            None => {
                session.set("counter", 1);
                dbg!("no counter");
            }
        },
        Err(err) => {
            dbg!("{}", err);
        }
    };
    // if let Some(count) = session.get::<i32>("counter")? {
    //     println!("SESSION value: {}", count);
    //     session.set("counter", count + 1)?;
    // } else {
    //     session.set("counter", 1)?;
    // }
    HttpResponse::Ok().body("Hello world!")
}

struct State {
    schema: Arc<Schema>,
    books_repository_addr: actix::Addr<InMemoryBooksRepository>,
    users_repository_addr: actix::Addr<InMemoryUsersRepository>,
    bookmarks_repository_addr: actix::Addr<InMemoryBookmarksRepository>,
    sessions_repository_addr: actix::Addr<InMemorySessionsRepository>,
}

fn main() -> std::io::Result<()> {
    let sys = actix::System::new("sync-marker");

    let books_repository_addr = InMemoryBooksRepository::new().start();
    let users_repository_addr = InMemoryUsersRepository::new().start();
    let bookmarks_repository_addr = InMemoryBookmarksRepository::new().start();
    let sessions_repository_addr = InMemorySessionsRepository::new().start();
    let schema = std::sync::Arc::new(create_schema());
    // let arc_session = Arc::new(
    //     CookieSession::signed(&[0; 32])
    //         // .domain("127.0.0.1:8080")
    //         // .domain("")
    //         .secure(false)
    //         .name("actix_session")
    //         .path("/"),
    // );
    // let state = State {
    //     schema: schema.clone(),
    //     addr: addr.clone(),
    // };
    HttpServer::new(move || {
        App::new()
            // .data(schema.clone())
            .data(State {
                schema: schema.clone(),
                books_repository_addr: books_repository_addr.clone(),
                users_repository_addr: users_repository_addr.clone(),
                bookmarks_repository_addr: bookmarks_repository_addr.clone(),
                sessions_repository_addr: sessions_repository_addr.clone(),
            })
            .wrap(
                CookieSession::signed(&[0; 32])
                    // .domain("127.0.0.1:8080")
                    // .domain("")
                    .secure(false)
                    .name("actix_session")
                    .path("/"),
            )
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .start();
    sys.run()
}

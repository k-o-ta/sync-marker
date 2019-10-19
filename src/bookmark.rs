use super::bookshelf::{BooksRepository, FindByIsbn, InMemoryBooksRepository, Isbn};
use super::session::{FindUserId, InMemorySessionsRepository, SessionDigest};
use super::user::{FindById, InMemoryUsersRepository, UsersRepository};
use actix::prelude::*;
use actix::Addr;
use std::io;
use tokio::prelude::*;

impl InMemoryBookmarksRepository {
    pub fn new() -> Self {
        InMemoryBookmarksRepository(Vec::new())
    }
}

trait BookmarksRepository {
    fn progress(
        &mut self,
        user_id: u32,
        isbn: Isbn,
        page_in_progress: u16,
        session_digest: SessionDigest,
        sessions_repository: Addr<InMemorySessionsRepository>,
        users_repository: Addr<InMemoryUsersRepository>,
        books_repository: Addr<InMemoryBooksRepository>,
    ) -> Result<(), ProgressBookmarkRepositoryError>;
}

pub struct InMemoryBookmarksRepository(Vec<Bookmark>);

impl BookmarksRepository for InMemoryBookmarksRepository {
    fn progress(
        &mut self,
        user_id: u32,
        isbn: Isbn,
        page_in_progress: u16,
        session_digest: SessionDigest,
        sessions_repository: Addr<InMemorySessionsRepository>,
        users_repository: Addr<InMemoryUsersRepository>,
        books_repository: Addr<InMemoryBooksRepository>,
    ) -> Result<(), ProgressBookmarkRepositoryError> {
        let session = sessions_repository.send(FindUserId(session_digest));
        let user = session
            .then(|user_id| user_id.map(|user_id| user_id.map(|user_id| users_repository.send(FindById(user_id)))));
        // let user = session.and_then(|user_id| match user_id {
        //     Some(user_id) => users_repository.send(FindById(user_id)),
        //     None => panic!(""),
        // });
        // let user_future = users_repository.send(FindById)
        let book = books_repository.send(FindByIsbn(isbn));
        let user_and_book = user.join(book);
        let user_and_book = user_and_book.wait().map(|(user, book)| {
            // match user {
            //     Some(user) => user.th,
            //     None => {
            //         return Err(ProgressBookmarkRepositoryError::UserNotFoundError(user_id).into());
            //     }
            // }
            if user.is_none() {
                return Err(ProgressBookmarkRepositoryError::UserNotFoundError(user_id).to_string());
            }
            if book.is_none() {
                return Err(ProgressBookmarkRepositoryError::BookNotFoundError(isbn).to_string());
            }
            Ok((user.unwrap(), book.unwrap()))
        });

        // if users_repository.find_by_id(user_id).is_none() {
        //     return Err(ProgressBookmarkRepositoryError::UserNotFoundError(user_id).into());
        // }
        // if books_repository.find_by_isbn(isbn).is_none() {
        //     return Err(ProgressBookmarkRepositoryError::BookNotFoundError(book_id).into());
        // }
        let (user, book) = user_and_book.unwrap().unwrap();
        if let Some(bookmark) = self
            .0
            .iter_mut()
            .find(|bookmark| bookmark.user_id == user_id && bookmark.book_id == book.id)
        {
            bookmark.page_in_progress = page_in_progress
        } else {
            let latest_bookmark = self.0.iter().max_by_key(|bookmark| bookmark.id);
            let bookmark = if let Some(latest_bookmark) = latest_bookmark {
                Bookmark {
                    id: latest_bookmark.id + 1,
                    user_id,
                    book_id: book.id,
                    page_in_progress,
                }
            } else {
                Bookmark {
                    id: 1,
                    user_id,
                    book_id: book.id,
                    page_in_progress,
                }
            };
            self.0.push(bookmark)
        }
        Ok(())
    }
}

#[derive(Fail, Debug)]
enum ProgressBookmarkRepositoryError {
    #[fail(display = "Session Not Found")]
    SessionNotFoundError,
    #[fail(display = "User Not Found")]
    UserNotFoundError(u32),
    #[fail(display = "Book Not Found")]
    BookNotFoundError(Isbn),
    #[fail(display = "page_cuont max is {}, but entered {}", _1, _0)]
    PageCountOverFlowError(u16, u16),
}
struct Bookmark {
    id: u64,
    user_id: u32,
    book_id: u32,
    page_in_progress: u16,
}
impl Actor for InMemoryBookmarksRepository {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("BookmarksRepository Actor is alive");
    }
    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("BookmarksRepository Actor is stopped");
    }
}

// message
pub struct Progress {
    user_id: u32,
    isbn: Isbn,
    page_in_progress: u16,
    session_digest: SessionDigest,
    sessions_repository: Addr<InMemorySessionsRepository>,
    users_repository: Addr<InMemoryUsersRepository>,
    books_repository: Addr<InMemoryBooksRepository>,
}

impl Message for Progress {
    type Result = Result<(), io::Error>;
}

impl Handler<Progress> for InMemoryBookmarksRepository {
    type Result = Result<(), io::Error>;
    fn handle(&mut self, msg: Progress, _ctx: &mut Context<Self>) -> Self::Result {
        self.progress(
            msg.user_id,
            msg.isbn,
            msg.page_in_progress,
            msg.session_digest,
            msg.sessions_repository,
            msg.users_repository,
            msg.books_repository,
        );
        Ok(())
    }
}

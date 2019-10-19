use super::bookshelf::{BooksRepository, UsersRepository};
use actix::prelude::*;
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
        book_id: u32,
        page_in_progress: u16,
        users_repository: &dyn UsersRepository,
        books_repository: &dyn BooksRepository,
    ) -> Result<(), ProgressBookmarkRepositoryError>;
}

pub struct InMemoryBookmarksRepository(Vec<Bookmark>);

impl BookmarksRepository for InMemoryBookmarksRepository {
    fn progress(
        &mut self,
        user_id: u32,
        book_id: u32,
        page_in_progress: u16,
        users_repository: &dyn UsersRepository,
        books_repository: &dyn BooksRepository,
    ) -> Result<(), ProgressBookmarkRepositoryError> {
        if users_repository.find_by_id(user_id).is_none() {
            return Err(ProgressBookmarkRepositoryError::UserNotFoundError(user_id).into());
        }
        if books_repository.find_by_id(book_id).is_none() {
            return Err(ProgressBookmarkRepositoryError::BookNotFoundError(book_id).into());
        }
        if let Some(bookmark) = self
            .0
            .iter_mut()
            .find(|bookmark| bookmark.user_id == user_id && bookmark.book_id == book_id)
        {
            bookmark.page_in_progress = page_in_progress
        } else {
            let latest_bookmark = self.0.iter().max_by_key(|bookmark| bookmark.id);
            let bookmark = if let Some(latest_bookmark) = latest_bookmark {
                Bookmark {
                    id: latest_bookmark.id + 1,
                    user_id,
                    book_id,
                    page_in_progress,
                }
            } else {
                Bookmark {
                    id: 1,
                    user_id,
                    book_id,
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
    #[fail(display = "User Not Found")]
    UserNotFoundError(u32),
    #[fail(display = "Book Not Found")]
    BookNotFoundError(u32),
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

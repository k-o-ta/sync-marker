use juniper::FieldResult;
use juniper::RootNode;

#[derive(GraphQLObject)]
struct Book {
    id: String,
    name: String,
    page: i32,
    page_in_progress: Option<i32>,
}

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field books(&executor, user_id: String) -> FieldResult<Vec<Book>> {
        Ok(
            vec![
            Book{id: "1".to_owned(), name: "a".to_owned(), page: 100, page_in_progress: Some(1)},
            Book{id: "2".to_owned(), name: "b".to_owned(), page: 200, page_in_progress: Some(2)},

            ]
            )
    }
});

pub struct MutationRoot;
graphql_object!(MutationRoot: () |&self| {
    field createBook(&executor, book_id: String) -> FieldResult<()> {
    Ok(
            // Book{id: "1".to_owned(), name: "a".to_owned(), page: 100, page_in_progress: Some(1)}
            ()
        )
    }
});

// pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;
pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

use actix::Addr;
use juniper::FieldResult;
use juniper::RootNode;

pub struct Context {
    // addr: Addr<super::bookshelf::Bookshelf>,
}
impl juniper::Context for Context {}

#[derive(GraphQLObject)]
struct Book {
    id: String,
    name: String,
    page: i32,
    page_in_progress: Option<i32>,
}

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn books(context: &Context, user_id: String) -> FieldResult<Vec<Book>> {
        Ok(vec![
            Book {
                id: "1".to_owned(),
                name: "a".to_owned(),
                page: 100,
                page_in_progress: Some(1),
            },
            Book {
                id: "2".to_owned(),
                name: "b".to_owned(),
                page: 200,
                page_in_progress: Some(2),
            },
        ])
    }
}

// pub struct QueryRoot;
//
// graphql_object!(QueryRoot: () |&self| {
//     field books(&executor, user_id: String) -> FieldResult<Vec<Book>> {
//         Ok(
//             vec![
//             Book{id: "1".to_owned(), name: "a".to_owned(), page: 100, page_in_progress: Some(1)},
//             Book{id: "2".to_owned(), name: "b".to_owned(), page: 200, page_in_progress: Some(2)},
//
//             ]
//             )
//     }
// });

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn createBook(book_id: String) -> FieldResult<Book> {
        Ok(
            Book {
                id: "1".to_owned(),
                name: "a".to_owned(),
                page: 100,
                page_in_progress: Some(1),
            }, // (),
        )
    }
}

// pub struct MutationRoot;
// graphql_object!(MutationRoot: () |&self| {
//     field createBook(&executor, book_id: String) -> FieldResult<()> {
//     Ok(
//             // Book{id: "1".to_owned(), name: "a".to_owned(), page: 100, page_in_progress: Some(1)}
//             ()
//         )
//     }
// });

// pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;
// pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;
// pub type Schema = RootNode<'static, Query, juniper::EmptyMutation<Context>>;
pub type Schema = RootNode<'static, Query, Mutation>;
pub fn create_schema() -> Schema {
    // Schema::new(Query {}, MutationRoot {})
    // Schema::new(Query, juniper::EmptyMutation::new())
    Schema::new(Query, Mutation)
}

use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};

use crate::schema::auth::Token;
use crate::service::Service;

pub struct Context {
    pub service: Service,
}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }
}

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    #[graphql(description = "Register a new user")]
    pub async fn register(ctx: &Context, username: String, password: String) -> FieldResult<Token> {
        ctx.service
            .auth
            .register(username, password)
            .await
            .map_err(|e| e.into())
    }
    #[graphql(description = "Authenticate a user")]
    pub async fn login(ctx: &Context, username: String, password: String) -> FieldResult<Token> {
        ctx.service
            .auth
            .login(username, password)
            .await
            .map_err(|e| e.into())
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}

use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};

use crate::schema::{auth::Token, submission::Submission};
use crate::service::Service;
use crate::util::authenticate;

#[derive(Clone)]
pub struct Context {
    pub service: Service,
    pub auth: Option<String>,
}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }
    #[graphql(description = "Get all submissions")]
    pub async fn submissions(ctx: &Context) -> FieldResult<Vec<Submission>> {
        ctx.service
            .submission
            .get_submissions()
            .await
            .map_err(|e| e.into())
    }
    #[graphql(description = "Get a submission by its ID")]
    pub async fn submission_by_id(ctx: &Context, id: String) -> FieldResult<Submission> {
        ctx.service
            .submission
            .get_submission_by_id(&id)
            .await
            .map_err(|e| e.into())
    }
}

pub struct Mutation;

#[graphql_object(context = Context)]
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
    #[graphql(description = "Submit a submission")]
    pub async fn submit_submission(
        ctx: &Context,
        question: String,
        answer: String,
    ) -> FieldResult<Submission> {
        ctx.service
            .submission
            .submit_submission(&authenticate(ctx)?, &question, &answer)
            .await
            .map_err(|e| e.into())
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}

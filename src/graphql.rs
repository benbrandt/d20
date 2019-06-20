use crate::{
    dice_roller::{self, RollInstruction, RollResult},
    State,
};
use http_service::Body;
use juniper::{graphiql::graphiql_source, FieldResult};
use tide::{
    error::ResultExt,
    http::{header, Response, StatusCode},
    response, Context, EndpointResult,
};

impl juniper::Context for State {}

struct Query;

#[juniper::object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    Context = State,
)]
impl Query {
    // Arguments to resolvers can either be simple types or input objects.
    // To gain access to the context, we specify a argument
    // that is a reference to the Context type.
    // Juniper automatically injects the correct context here.

    #[graphql(
        arguments(
            num(description = "Number of dice to roll"),
            die(description = "Number of sides on the die"),
            modifier(
                default = 0,
                description = "Additional modifier to the roll",
            ),
        )
    )]
    fn roll(context: &State, num: i32, die: i32, modifier: i32) -> FieldResult<RollResult> {
        Ok(dice_roller::roll(RollInstruction { num, die, modifier })?)
    }
}

// Now, we do the same for our Mutation type.

struct Mutation;

#[juniper::object(
    Context = State,
)]
impl Mutation {}

// Adding `Query` and `Mutation` together we get `Schema`, which describes, well, the whole GraphQL
// schema.
type Schema = juniper::RootNode<'static, Query, Mutation>;

// Finally, we'll bridge between Tide and Juniper. `GraphQLRequest` from Juniper implements
// `Deserialize`, so we use `Json` extractor to deserialize the request body.
pub async fn handle_graphql(mut cx: Context<State>) -> EndpointResult {
    let query: juniper::http::GraphQLRequest = cx.body_json().await.client_err()?;
    let schema = Schema::new(Query, Mutation);
    let response = query.execute(&schema, cx.app_data());
    let status = if response.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    let mut resp = response::json(response);
    *resp.status_mut() = status;
    Ok(resp)
}

pub async fn handle_graphiql(_: Context<State>) -> EndpointResult {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime::TEXT_HTML.as_ref())
        .body(Body::from(graphiql_source("/graphql")))
        .expect("failed to build graphiql"))
}

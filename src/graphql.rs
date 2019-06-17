use crate::State;
use http_service::Body;
use juniper::{graphiql::graphiql_source, graphql_object};
use std::sync::atomic;
use tide::{
    error::ResultExt,
    http::{header, Response, StatusCode},
    response, Context, EndpointResult,
};

// We define `Query` unit struct here. GraphQL queries will refer to this struct. The struct itself
// doesn't have any associated state (and there's no need to do so), but instead it exposes the
// accumulator state from the context.
struct Query;

graphql_object!(Query: State |&self| {
    // GraphQL integers are signed and 32 bits long.
    field accumulator(&executor) -> i32 as "Current value of the accumulator" {
        executor.context().0.load(atomic::Ordering::Relaxed) as i32
    }
});

// Here is `Mutation` unit struct. GraphQL mutations will refer to this struct. This is similar to
// `Query`, but it provides the way to "mutate" the accumulator state.
struct Mutation;

graphql_object!(Mutation: State |&self| {
    field add(&executor, by: i32) -> i32 as "Add given value to the accumulator." {
        executor.context().0.fetch_add(by as isize, atomic::Ordering::Relaxed) as i32 + by
    }
});

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

pub async fn handle_graphiql(cx: Context<State>) -> EndpointResult {
    let html = graphiql_source(
        &cx.request()
            .uri()
            .to_string()
            .replace("graphiql", "graphql"),
    );
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime::TEXT_HTML.as_ref())
        .body(Body::from(html))
        .expect("failed to build graphiql"))
}

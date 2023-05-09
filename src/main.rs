mod controllers;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, ObjectType, Request, Response, Schema,
    SubscriptionType,
};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    http::{HeaderMap},
    routing::get,
    Router, Server,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use controllers::{Query, Mutation};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>{
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish();
    let app = App{schema};
    std::fs::write("schema.graphql", app.sdl())?;

    let graphql_http = Router::new()
        .route(
            "/graphql",
            get(graphql_playground).post(graphql_handler::<Query, Mutation, EmptySubscription>),
        )
        .layer(Extension(app));
    let health = Router::new().route("/health", get(health));

    let graphql_http = graphql_http.merge(health);

    println!("Server started on port {}", "8080");

    Server::bind(&([0, 0, 0, 0], 8080).into())
        .serve(graphql_http.into_make_service())
        .await?;

    Ok(())
}

#[derive(Clone)]
pub struct App<TQuery, TMutation, TSubscription>
where
    TQuery: ObjectType + Sized + 'static,
    TMutation: ObjectType + Sized + 'static,
    TSubscription: SubscriptionType + Sized + 'static,
{
    schema: Schema<TQuery, TMutation, TSubscription>,
}

#[allow(clippy::type_complexity)]
impl<TQuery, TMutation, TSubscription> App<TQuery, TMutation, TSubscription>
where
    TQuery: ObjectType + Sized + 'static,
    TMutation: ObjectType + Sized + 'static,
    TSubscription: SubscriptionType + Sized + 'static,
{
    pub fn sdl(&self) -> String {
        self.schema.sdl()
    }

    pub async fn execute(&self, _: HeaderMap, req: Request) -> Response {
        self.schema.execute(req).await
    }

}

pub async fn graphql_handler<TQuery, TMutation, TSubscription>(
    Extension(app): Extension<App<TQuery, TMutation, TSubscription>>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse
where
    TQuery: ObjectType + Sized + 'static,
    TMutation: ObjectType + Sized + 'static,
    TSubscription: SubscriptionType + Sized + 'static,
{
    app.execute(headers, req.into_inner()).await.into()
}

#[allow(clippy::unused_async)]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[allow(clippy::unused_async)]
async fn health() -> Html<&'static str> {
    Html("Ok")
}

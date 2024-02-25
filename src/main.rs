use askama::Template;
use axum::extract::Query;
use axum::response::Html;
use axum::{routing::get, Router};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct RootQueryParams {
    name: Option<String>,
}
async fn root(Query(query_params): Query<RootQueryParams>) -> Html<String> {
    let name_or_defualt = query_params.name.unwrap_or("World".to_string());
    let template = HelloTemplate {
        name: &name_or_defualt,
    };

    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

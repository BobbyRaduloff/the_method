use askama::Template;
use axum::extract::{Form, Path, State};
use axum::response::Html;
use axum::routing::post;
use axum::{routing::get, Router};
use entity::todo_item as TodoItem;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait};
use serde::Deserialize;
use std::process::Command;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();

    Command::new("npx")
        .arg("tailwindcss")
        .arg("-i")
        .arg("./assets/css/main.css")
        .arg("-o")
        .arg("./assets/css/tailwind.css")
        .spawn()
        .expect("Failed to run tailwindcss.");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
    let db: DatabaseConnection = Database::connect(db_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    let state = AppState { db };

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(root).post(add_item))
        .route("/flip/:item_id", post(flip_item))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root(State(state): State<AppState>) -> Html<String> {
    if let Ok(all_todo_items) = TodoItem::Entity::find().all(&state.db).await {
        let template = IndexTemplate {
            items: all_todo_items,
        };
        return Html(template.render().unwrap());
    }

    let template = ErrorTemplate;
    Html(template.render().unwrap())
}

#[derive(Deserialize)]
struct AddItem {
    text: String,
}
async fn add_item(State(state): State<AppState>, Form(form): Form<AddItem>) -> Html<String> {
    let new_item = TodoItem::ActiveModel {
        text: sea_orm::ActiveValue::Set(form.text),
        ..Default::default()
    };

    let active_model_result: Result<TodoItem::Model, _> = new_item.insert(&state.db).await;
    if let Err(_) = active_model_result {
        let template = ErrorTemplate;
        return Html(template.render().unwrap());
    }

    let template = ItemTemplate {
        item: active_model_result.unwrap().into(),
    };
    Html(template.render().unwrap())
}

async fn flip_item(State(state): State<AppState>, Path(item_id): Path<i32>) -> Html<String> {
    let item_result = TodoItem::Entity::find_by_id(item_id).one(&state.db).await;
    if let Err(_) = item_result {
        let template = ErrorTemplate;
        return Html(template.render().unwrap());
    }

    let item = item_result.unwrap().unwrap();

    let mut new_item: TodoItem::ActiveModel = item.clone().into();
    let previous_value: bool = item.done;
    new_item.done = sea_orm::ActiveValue::Set(!previous_value);

    let active_model_result: Result<TodoItem::Model, _> = new_item.update(&state.db).await;
    if let Err(_) = active_model_result {
        let template = ErrorTemplate;
        return Html(template.render().unwrap());
    }

    let template = ItemTemplate {
        item: active_model_result.unwrap().into(),
    };
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    items: Vec<TodoItem::Model>,
}

#[derive(Template)]
#[template(path = "item.html")]
struct ItemTemplate {
    item: TodoItem::Model,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate;

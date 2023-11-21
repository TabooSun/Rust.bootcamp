#[macro_use]
extern crate rocket;

// TODO: import log, pretty_env_logger, dotenv, and PgPoolOptions

use std::env;
use dotenvy::dotenv;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use cors::*;
use handlers::*;

mod cors;
mod handlers;
mod models;

#[launch]
async fn rocket() -> _ {
    // TODO: Initialize pretty_env_logger
    pretty_env_logger::init();
    // TODO: Initialize dotenv
    dotenv().ok();

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    let pg_pool_options = PgPoolOptions::default().max_connections(5);
    let database_url = env::var("DATABASE_URL").expect("No env found");
    // Use dotenv to get the database url.
    // Use the `unwrap` or `expect` method instead of handling errors. If an
    // error occurs at this stage the server should be terminated. 
    // See examples on GitHub page: https://github.com/launchbadge/sqlx
    let pool = pg_pool_options.connect(&database_url).await.unwrap();

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.
    let recs = sqlx::query!(
        r#"
        SELECT * FROM questions
        "#
    ).fetch_all(&pool).await.unwrap();

    info!("********* Question Records *********");
    // TODO: Log recs with debug formatting using the info! macro
    info!("{:?}", recs);

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
}

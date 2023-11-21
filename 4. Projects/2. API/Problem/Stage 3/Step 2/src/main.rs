#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use dotenvy::dotenv;

use sqlx::postgres::PgPoolOptions;

mod cors;
mod handlers;
mod models;
mod persistance;

use cors::*;
use handlers::*;
use crate::persistance::answers_dao::AnswersDaoImpl;
use crate::persistance::questions_dao::{QuestionsDao, QuestionsDaoImpl};

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    let questions_dao = QuestionsDaoImpl::new(pool.clone()); // create a new instance of QuestionsDaoImpl passing in `pool` (use the clone method)
    let answers_dao = AnswersDaoImpl::new(pool); // create a new instance of AnswersDaoImpl passing in `pool`

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
        // The manage method allows us to add state to the state managed by this instance of Rocket. Then we can use this state in the handlers.
        .manage(Box::new(questions_dao) as Box<dyn Send + Sync + QuestionsDao>) // pass in `questions_dao` as a boxed trait object. hint: you must cast `questions_dao` to a trait object.
        .manage(Box::new(answers_dao)) // pass in `answers_dao` as a boxed trait object. hint: you must cast `answers_dao` to a trait object.
}

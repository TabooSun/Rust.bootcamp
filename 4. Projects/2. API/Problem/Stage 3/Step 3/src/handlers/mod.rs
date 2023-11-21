use rocket::{serde::json::Json, State};

use crate::{
    models::*,
    persistance::{answers_dao::AnswersDao, questions_dao::QuestionsDao},
};

mod handlers_inner;

use handlers_inner::*;

#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalError(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(s) => Self::BadRequest(s),
            HandlerError::InternalError(s) => Self::InternalError(s),
        }
    }
}

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<QuestionDetail>, APIError> { // TODO: update return type to be of type `Result`. Use `APIError` for the error case.
    // TODO: Replace the fake data below with a call to `handlers_inner::create_question`.
    let result = handlers_inner::create_question(question.into_inner(), questions_dao).await?;
    // Return the result wrapped in JSON in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    Ok(Json(result))
}

#[get("/questions")]
pub async fn read_questions(
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<Vec<QuestionDetail>>, APIError> { // TODO: update return type to be of type `Result`. Use `APIError` for the error case.
    // TODO: Replace the fake data below with a call to `handlers_inner::read_questions`.
    let result = handlers_inner::read_questions(questions_dao).await?;
    // Return the result wrapped in JSON in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    Ok(Json(result))
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<(), APIError> { // TODO: return `Result` from this function. Use a unit type in the success case and `APIError` in the error case.
    // TODO: Make a call to `handlers_inner::delete_question`.
    handlers_inner::delete_question(question_uuid.into_inner(), questions_dao).await?;
    // Return a unit type in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    Ok(())
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Result<Json<AnswerDetail>, APIError> { // TODO: update return type to be of type `Result`. Use `APIError` for the error case.
    // TODO: Replace the fake data below with a call to `handlers_inner::create_answer`.
    let result = handlers_inner::create_answer(answer.into_inner(), answers_dao).await?;
    // Return the result wrapped in JSON in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    Ok(Json(result))
}

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(
    question_uuid: Json<QuestionId>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Result<Json<Vec<AnswerDetail>>, APIError> { // TODO: update return type to be of type `Result`. Use `APIError` for the error case.
    // TODO: Replace the fake data below with a call to `handlers_inner::read_answers`.
    let result = handlers_inner::read_answers(question_uuid.into_inner(), answers_dao).await?;
    // Return the result wrapped in JSON in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    Ok(Json(result))
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Result<(), APIError> { // TODO: return `Result` from this function. Use a unit type in the success case and `APIError` in the error case.
    // TODO: Make a call to `handlers_inner::delete_answer`.
    handlers_inner::delete_answer(answer_uuid.into_inner(), answers_dao).await?;
    // Return a unit type in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    Ok(())
}

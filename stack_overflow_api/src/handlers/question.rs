use crate::{app::error::AppError, app::state::AppState, dto::question_dto::{CreateQuestionResponse, QuestionsListResponse}};



pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/v1/questions", axum::routing::post(create_question))
        .route("/v1/questions", axum::routing::get(get_questions))
}

async fn create_question() -> Result<CreateQuestionResponse, AppError> {
    // Logic to create a new question
    Err(AppError::InternalServerError("Question creation not implemented".to_string()))
}

async fn get_questions() -> Result<QuestionsListResponse, AppError> {
    // Logic to retrieve a list of questions
    Err(AppError::InternalServerError("Question retrieval not implemented".to_string()))
}

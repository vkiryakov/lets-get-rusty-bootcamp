use axum::extract::State;
use tracing::info;

use crate::{
    app::{error::AppError, extractors::valid_json::ValidJson, state::AppState},
    dto::question_dto::{CreateQuestionRequest, CreateQuestionResponse, QuestionsListResponse},
};

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/v1/questions", axum::routing::post(create_question))
        .route("/v1/questions", axum::routing::get(get_questions))
}

/// Handler for creating a new question.
#[tracing::instrument(name = "Create Question", skip(state))]
async fn create_question(
    State(state): State<AppState>,
    ValidJson(payload): ValidJson<CreateQuestionRequest>,
) -> Result<CreateQuestionResponse, AppError> {

    // Insert question to database and return the created question with its ID
    let question_id = state.question_repo
        .create_question(&payload.title, &payload.body)
        .await
        .map_err(|err| {
            tracing::error!("Failed to insert question: {:?}", err);
            AppError::InternalServerError("Failed to create question".to_string())
        })?;

    info!("Question created with ID: {}", question_id);
    Ok(CreateQuestionResponse::new(question_id))

}

/// Handler for retrieving a list of questions.
async fn get_questions(
) -> Result<QuestionsListResponse, AppError> {
    // Logic to retrieve a list of questions
    Err(AppError::InternalServerError(
        "Question retrieval not implemented".to_string(),
    ))
}

use axum::extract::State;
use tracing::info;

use crate::{
    app::{
        error::AppError,
        extractors::{valid_json::ValidJson, valid_query::ValidQuery},
        state::AppState,
    },
    dto::{common::LimitOffset, question_dto::{CreateQuestionRequest, CreateQuestionResponse, QuestionsListResponse}},
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
#[tracing::instrument(name = "Get Questions", skip(state))]
async fn get_questions(
    State(state): State<AppState>,
    ValidQuery(pagination): ValidQuery<LimitOffset>,
) -> Result<QuestionsListResponse, AppError> {

    let questions = state.question_repo
        .get_questions(pagination.limit, pagination.offset)
        .await
        .map_err(|err| {
            tracing::error!("Failed to fetch questions: {:?}", err);
            AppError::InternalServerError("Failed to fetch questions".to_string())
        })?;

    Ok(QuestionsListResponse::new(questions))
}

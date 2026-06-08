use crate::dto::question_dto::CreateQuestionResponse;



pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/v1/questions", axum::routing::post(create_question))
        .route("/v1/questions", axum::routing::get(get_questions))
}

async fn create_question() -> CreateQuestionResponse {
    // Logic to create a new question
    todo!()
}

async fn get_questions() {
    // Logic to retrieve a list of questions
    todo!()
}

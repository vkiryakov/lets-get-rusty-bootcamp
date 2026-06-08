use validator::Validate;


#[derive(Debug, serde::Serialize, serde::Deserialize, Validate)]
pub struct LimitOffset {
    #[validate(range(min = 1, max = 100, message = "limit must be between 1 and 100"))]
    pub limit: i64,

    #[validate(range(min = 0, message = "offset must be 0 or greater"))]
    pub offset: i64,
}
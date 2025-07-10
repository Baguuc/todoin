#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error)
}

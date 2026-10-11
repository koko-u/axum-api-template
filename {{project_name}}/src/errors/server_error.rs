use axum::http;
use axum::response;
use tokio::sync;
use tokio::task;

use crate::shared::responses;

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum ServerError {
    #[display("Database error: {}", _0)]
    Database(#[error(source)] sqlx::Error),
    #[display("concurrency error: {}", _0)]
    Semaphore(#[error(source)] sync::AcquireError),
    #[display("task error: {}", _0)]
    Task(#[error(source)] task::JoinError),
    #[display("Validation error: {}", _0)]
    Validation(#[error(source)] garde::Report),
}

impl response::IntoResponse for ServerError {
    fn into_response(self) -> response::Response {
        match self {
            /*INTERNAL SERVER ERROR*/
            Self::Database(err) => {
                tracing::error!("Database error: {err:?}");
                let problem_details = responses::ProblemDetails::simple(
                    "Database Access Error",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            Self::Semaphore(err) => {
                tracing::error!("Semaphore error: {}", err);
                let problem_details = responses::ProblemDetails::simple(
                    "Semaphore Error",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            Self::Task(err) => {
                tracing::error!("Task error: {}", err);
                let problem_details = responses::ProblemDetails::simple(
                    "Task Join Error",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            /*BAD REQUEST*/
            Self::Validation(err) => {
                tracing::warn!("Validation error: {err:?}");
                let problem_details = responses::ProblemDetails::from(err);
                (problem_details.status, axum::Json(problem_details)).into_response()
            }
        }
    }
}

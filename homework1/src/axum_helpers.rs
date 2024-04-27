//helper axum functions -- IntoResponse and Response
use axum::{response::{Response, IntoResponse}, Json, http::StatusCode};
use serde::Serialize;
use crate::question::{Question, QuestionId}

pub enum ApiResponse {
    OK,
    Created,
    JsonData(Question),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response()
        }
    }
}
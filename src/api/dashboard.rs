use axum::{Router, extract::State, routing::get};

use crate::AppState;

pub fn dashboard_router() -> Router<AppState> {
    Router::new()
        .route("/dashboard/submission-stats", get(submission_stats))
        .route("/dashboard/recent-submissions", get(recent_submissions))
}

async fn submission_stats(State(state): State<AppState>) -> String {
    todo!()
}

async fn recent_submissions(State(state): State<AppState>) -> String {
    todo!()
}

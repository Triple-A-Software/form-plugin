use axum::{Router, extract::Request, response::IntoResponse, routing::get};
use tower::util::ServiceExt;
use tower_http::services::ServeFile;

use crate::AppState;

pub fn dashboard_router() -> Router<AppState> {
    Router::new()
        .route("/dashboard/submission-stats", get(submission_stats))
        .route("/dashboard/recent-submissions", get(recent_submissions))
}

async fn submission_stats(req: Request) -> impl IntoResponse {
    let service = ServeFile::new("ui/dist/dashboard/submission-stats.html");
    service.oneshot(req).await.unwrap()
}

async fn recent_submissions(req: Request) -> impl IntoResponse {
    let service = ServeFile::new("ui/dist/dashboard/recent-submissions.html");
    service.oneshot(req).await.unwrap()
}

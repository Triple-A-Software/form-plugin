use axum::{extract::Request, response::IntoResponse};
use tower::util::ServiceExt;
use tower_http::services::ServeFile;

pub async fn route_submission_stats(req: Request) -> impl IntoResponse {
    let service = ServeFile::new("ui/dist/dashboard/submission-stats.html");
    service.oneshot(req).await.unwrap()
}

pub async fn route_recent_submissions(req: Request) -> impl IntoResponse {
    let service = ServeFile::new("ui/dist/dashboard/recent-submissions.html");
    service.oneshot(req).await.unwrap()
}

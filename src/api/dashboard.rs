use axum::{extract::Request, response::IntoResponse};
use tower::util::ServiceExt;
use tower_http::services::ServeFile;

use crate::utils::AppResult;

pub async fn dashboard_card_submission_stats(req: Request) -> AppResult<impl IntoResponse> {
    let service = ServeFile::new("ui/dist/dashboard/submission-stats.html");
    Ok(service.oneshot(req).await?)
}

pub async fn dashboard_card_recent_submissions(req: Request) -> AppResult<impl IntoResponse> {
    let service = ServeFile::new("ui/dist/dashboard/recent-submissions.html");
    Ok(service.oneshot(req).await?)
}

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

pub fn health() -> HealthResponse {
    // TODO: Check database connectivity before reporting healthy.
    HealthResponse { status: "ok" }
}

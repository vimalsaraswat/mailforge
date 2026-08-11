use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    controllers::errors::EmailTemplateError,
    dto::email_template::{
        CreateEmailTemplateRequest, EmailTemplateResponse, UpdateEmailTemplateRequest,
    },
    middleware::AuthenticatedUser,
    services::email_template::EmailTemplateService,
    state::AppState,
};

pub async fn list(
    AuthenticatedUser { user }: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<EmailTemplateResponse>>, EmailTemplateError> {
    let service = EmailTemplateService::new(&state.db);

    let templates = service.list(user.id).await?;

    let response = templates
        .into_iter()
        .map(EmailTemplateResponse::from)
        .collect();

    Ok(Json(response))
}

pub async fn get(
    AuthenticatedUser { user }: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<EmailTemplateResponse>, EmailTemplateError> {
    let service = EmailTemplateService::new(&state.db);

    let template = service
        .get(user.id, id)
        .await?
        .ok_or(EmailTemplateError::NotFound)?;

    Ok(Json(EmailTemplateResponse::from(template)))
}

pub async fn create(
    AuthenticatedUser { user }: AuthenticatedUser,
    State(state): State<AppState>,
    Json(request): Json<CreateEmailTemplateRequest>,
) -> Result<(StatusCode, Json<EmailTemplateResponse>), EmailTemplateError> {
    let service = EmailTemplateService::new(&state.db);

    let template = service
        .create(user.id, &request.name, &request.subject, &request.body)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(EmailTemplateResponse::from(template)),
    ))
}

pub async fn update(
    AuthenticatedUser { user }: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateEmailTemplateRequest>,
) -> Result<Json<EmailTemplateResponse>, EmailTemplateError> {
    let service = EmailTemplateService::new(&state.db);

    let template = service
        .update(user.id, id, &request.name, &request.subject, &request.body)
        .await?
        .ok_or(EmailTemplateError::NotFound)?;

    Ok(Json(EmailTemplateResponse::from(template)))
}

pub async fn delete(
    AuthenticatedUser { user }: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, EmailTemplateError> {
    let service = EmailTemplateService::new(&state.db);

    if !service.delete(user.id, id).await? {
        return Err(EmailTemplateError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

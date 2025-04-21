use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::WebAuthExt;

#[command]
pub async fn authenticate<R: tauri::Runtime>(
    app: AppHandle<R>,
    payload: AuthenticateRequest,
) -> crate::Result<AuthenticateResponse> {
    app.web_auth().authenticate(payload)
}

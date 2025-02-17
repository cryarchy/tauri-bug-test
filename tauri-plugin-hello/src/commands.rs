use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::HelloExt;
use crate::Result;

#[tauri::command]
#[specta::specta]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.hello().ping(payload)
}

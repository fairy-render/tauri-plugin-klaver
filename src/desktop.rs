use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::{models::*, state::KlaverState};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Klaver<R>> {
    Ok(Klaver {
        app: app.clone(),
        state: Default::default(),
    })
}

/// Access to the klaver APIs.
pub struct Klaver<R: Runtime> {
    app: AppHandle<R>,
    pub state: KlaverState,
}

impl<R: Runtime> Klaver<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }
}

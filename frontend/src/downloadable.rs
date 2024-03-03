use owning_ref::MutexGuardRef;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Downloadable<T>
where
    T: serde::de::DeserializeOwned + Send + Sync,
{
    data: Arc<Mutex<Option<T>>>,
    state: Arc<Mutex<DownloadState>>,
}

impl<T> Default for Downloadable<T>
where
    T: serde::de::DeserializeOwned + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub enum DownloadState {
    None,
    InProgress,
    Done(ehttp::Result<ehttp::Response>),
}

impl<T> Downloadable<T>
where
    T: serde::de::DeserializeOwned + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(DownloadState::None)),
        }
    }

    pub fn get_data(&self) -> MutexGuardRef<'_, Option<T>, Option<T>> {
        MutexGuardRef::new(self.data.lock().unwrap())
    }

    pub fn get_response(&self) -> Option<ehttp::Response> {
        let state = &*self.state.lock().unwrap();
        match state {
            DownloadState::Done(response) if response.is_ok() => response.clone().ok(),
            _ => None,
        }
    }

    pub fn clear_data(&self) {
        *self.data.lock().unwrap() = None;
    }

    pub fn show_ui(
        &self,
        ui: &mut egui::Ui,
        ui_ok_fn: impl FnOnce(&mut egui::Ui, &T),
        ui_err_fn: impl FnOnce(&mut egui::Ui, &ehttp::Response),
    ) {
        let state = {
            let s = &*self.state.lock().unwrap();
            s.clone()
        };
        match state {
            DownloadState::None => {
                ui.label("No data");
            }
            DownloadState::InProgress => {
                ui.label("Download in progress");
            }
            DownloadState::Done(res) => match res {
                Err(_err) => {
                    ui.label("Could not fetch data");
                }
                Ok(response) => {
                    if response.status != 200 {
                        ui_err_fn(ui, &response);
                    } else {
                        let data = &*self.data.lock().unwrap();
                        match data {
                            Some(obj) => ui_ok_fn(ui, obj),
                            None => {
                                ui.label("Deserialization failed");
                            }
                        }
                    }
                }
            },
        }
    }

    pub fn start_download(&self, req: ehttp::Request, egui_context: egui::Context) {
        {
            let state: &DownloadState = &self.state.lock().unwrap();
            if matches!(state, DownloadState::InProgress) {
                return;
            }
        }

        {
            *self.data.lock().unwrap() = None;
            *self.state.lock().unwrap() = DownloadState::InProgress;
        }
        let state = self.state.clone();
        let data = self.data.clone();
        ehttp::fetch(req, move |response| {
            let d = match &response {
                Ok(response) if response.status == 200 => response.json::<T>().ok(),
                _ => None,
            };
            *data.lock().unwrap() = d;
            *state.lock().unwrap() = DownloadState::Done(response);
            egui_context.request_repaint();
        });
    }
}

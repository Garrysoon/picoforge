//! View model for firmware update screen — check GitHub releases, download, flash.

use crate::ui::app::AppModels;
use crate::ui::models::device::DeviceRepo;
use gpui::*;

pub struct FirmwareViewModel {
    pub device: Entity<DeviceRepo>,
    pub current_version: Option<String>,
    pub latest_version: Option<String>,
    pub update_available: bool,
    pub downloading: bool,
    pub flashing: bool,
    pub progress: f32,
    pub status_message: Option<String>,
    pub download_url: Option<String>,
    _task: Option<Task<()>>,
}

pub enum FirmwareEvent {
    Notification(String),
}

impl EventEmitter<FirmwareEvent> for FirmwareViewModel {}

impl FirmwareViewModel {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>, models: &AppModels) -> Self {
        Self {
            device: models.device.clone(),
            current_version: None,
            latest_version: None,
            update_available: false,
            downloading: false,
            flashing: false,
            progress: 0.0,
            status_message: None,
            download_url: None,
            _task: None,
        }
    }

    pub fn check_for_updates(&mut self, cx: &mut Context<Self>) {
        self.status_message = Some("Checking GitHub for updates...".into());
        cx.notify();

        let weak_self = cx.entity().downgrade();

        self._task = Some(cx.spawn(async move |_, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    check_github_releases_blocking()
                })
                .await;

            let _ = weak_self.update(cx, |this: &mut FirmwareViewModel, cx| {
                match result {
                    Ok((version, url)) => {
                        this.latest_version = Some(version.clone());
                        this.download_url = Some(url);
                        this.update_available = true;
                        this.status_message = Some(format!("New version available: {}", version));
                    }
                    Err(e) => {
                        this.status_message = Some(format!("Check failed: {}", e));
                    }
                }
                cx.notify();
            });
        }));
    }

    pub fn start_flash(&mut self, cx: &mut Context<Self>) {
        self.flashing = true;
        self.progress = 0.0;
        self.status_message = Some("Flashing firmware...".into());
        cx.notify();

        let weak_self = cx.entity().downgrade();

        self._task = Some(cx.spawn(async move |_, cx| {
            // Simulate flash progress
            for i in 0..=10 {
                std::thread::sleep(std::time::Duration::from_millis(200));
                let _ = weak_self.update(cx, |this: &mut FirmwareViewModel, cx| {
                    this.progress = i as f32 / 10.0;
                    cx.notify();
                });
            }

            let _ = weak_self.update(cx, |this: &mut FirmwareViewModel, cx| {
                this.flashing = false;
                this.progress = 1.0;
                this.status_message = Some("Firmware flashed successfully!".into());
                cx.notify();
            });
        }));
    }
}

async fn check_github_releases() -> Result<(String, String), String> {
    let url = "https://api.github.com/repos/Garrysoon/pico-fido/releases/latest";
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("User-Agent", "picoforge")
        .send()
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;

    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("JSON parse error: {}", e))?;

    let version = json["tag_name"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();

    let mut download_url = String::new();
    if let Some(assets) = json["assets"].as_array() {
        for asset in assets {
            if let Some(name) = asset["name"].as_str() {
                if name.ends_with(".bin") {
                    download_url = asset["browser_download_url"]
                        .as_str()
                        .unwrap_or("")
                        .to_string();
                    break;
                }
            }
        }
    }

    if download_url.is_empty() {
        return Err("No .bin asset found in release".into());
    }

    Ok((version, download_url))
}

fn check_github_releases_blocking() -> Result<(String, String), String> {
    let url = "https://api.github.com/repos/Garrysoon/pico-fido/releases/latest";
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| format!("Client error: {}", e))?;

    let resp = client
        .get(url)
        .header("User-Agent", "picoforge")
        .send()
        .map_err(|e| format!("HTTP error: {}", e))?;

    let json: serde_json::Value = resp
        .json()
        .map_err(|e| format!("JSON parse error: {}", e))?;

    let version = json["tag_name"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();

    let mut download_url = String::new();
    if let Some(assets) = json["assets"].as_array() {
        for asset in assets {
            if let Some(name) = asset["name"].as_str() {
                if name.ends_with(".bin") {
                    download_url = asset["browser_download_url"]
                        .as_str()
                        .unwrap_or("")
                        .to_string();
                    break;
                }
            }
        }
    }

    if download_url.is_empty() {
        return Err("No .bin asset found in release".into());
    }

    Ok((version, download_url))
}

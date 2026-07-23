use crate::ui::models::device::DeviceRepo;
use gpui::*;

pub struct VendorViewModel {
    pub device: Entity<DeviceRepo>,
    pub logs: Vec<String>,
    pub loading: bool,
    pub export_status: Option<String>,
    _task: Option<Task<()>>,
}

pub enum VendorEvent {
    Notification(String),
}

impl EventEmitter<VendorEvent> for VendorViewModel {}

impl VendorViewModel {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>, models: &crate::ui::app::AppModels) -> Self {
        Self {
            device: models.device.clone(),
            logs: Vec::new(),
            loading: false,
            export_status: None,
            _task: None,
        }
    }

    pub fn add_log(&mut self, msg: String) {
        let timestamp = chrono::Utc::now().format("%H:%M:%S").to_string();
        self.logs.push(format!("[{}] {}", timestamp, msg));
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }

    pub fn export_oath_credentials(&mut self, cx: &mut Context<Self>) {
        self.add_log("Exporting OATH credentials...".into());
        self.export_status = Some("Exporting...".into());
        cx.notify();

        let weak_self = cx.entity().downgrade();

        self._task = Some(cx.spawn(async move |_, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let path = dirs::data_local_dir()
                        .unwrap_or_else(|| std::path::PathBuf::from("."))
                        .join("picoforge")
                        .join("oath_credentials.json");

                    if path.exists() {
                        let data = std::fs::read_to_string(&path)
                            .map_err(|e| format!("Read error: {}", e))?;
                        let _export_path = std::path::PathBuf::from("oath_export.json");
                        std::fs::write("oath_export.json", &data)
                            .map_err(|e| format!("Write error: {}", e))?;
                        Ok("OATH credentials exported to oath_export.json".to_string())
                    } else {
                        Err("No OATH credentials found".to_string())
                    }
                })
                .await;

            let _ = weak_self.update(cx, |this: &mut VendorViewModel, cx| {
                match result {
                    Ok(msg) => {
                        this.add_log(format!("[OK] {}", msg));
                        this.export_status = Some(msg);
                    }
                    Err(e) => {
                        this.add_log(format!("[ERROR] {}", e));
                        this.export_status = Some(format!("Error: {}", e));
                    }
                }
                cx.notify();
            });
        }));
    }

    pub fn export_otp_credentials(&mut self, cx: &mut Context<Self>) {
        self.add_log("Exporting OTP credentials...".into());
        self.export_status = Some("Exporting...".into());
        cx.notify();

        let weak_self = cx.entity().downgrade();

        self._task = Some(cx.spawn(async move |_, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let path = dirs::data_local_dir()
                        .unwrap_or_else(|| std::path::PathBuf::from("."))
                        .join("picoforge")
                        .join("otp_credentials.json");

                    if path.exists() {
                        let data = std::fs::read_to_string(&path)
                            .map_err(|e| format!("Read error: {}", e))?;
                        std::fs::write("otp_export.json", &data)
                            .map_err(|e| format!("Write error: {}", e))?;
                        Ok("OTP credentials exported to otp_export.json".to_string())
                    } else {
                        Err("No OTP credentials found".to_string())
                    }
                })
                .await;

            let _ = weak_self.update(cx, |this: &mut VendorViewModel, cx| {
                match result {
                    Ok(msg) => {
                        this.add_log(format!("[OK] {}", msg));
                        this.export_status = Some(msg);
                    }
                    Err(e) => {
                        this.add_log(format!("[ERROR] {}", e));
                        this.export_status = Some(format!("Error: {}", e));
                    }
                }
                cx.notify();
            });
        }));
    }

    pub fn clear_logs(&mut self, cx: &mut Context<Self>) {
        self.logs.clear();
        self.add_log("Logs cleared".into());
        cx.notify();
    }
}

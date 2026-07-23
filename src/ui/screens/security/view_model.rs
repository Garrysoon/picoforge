//! View model for the security screen — secure boot and attestation state.

use crate::hal;
use crate::ui::app::AppModels;
use crate::ui::models::device::DeviceRepo;
use gpui::*;

pub struct SecurityViewModel {
    pub device: Entity<DeviceRepo>,
    pub secure_boot_enabled: bool,
    pub secure_lock_enabled: bool,
    pub loading: bool,
    pub status_message: Option<String>,
    _task: Option<Task<()>>,
}

pub enum SecurityEvent {
    Notification(String),
}

impl EventEmitter<SecurityEvent> for SecurityViewModel {}

impl SecurityViewModel {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>, models: &AppModels) -> Self {
        let mut vm = Self {
            device: models.device.clone(),
            secure_boot_enabled: false,
            secure_lock_enabled: false,
            loading: false,
            status_message: None,
            _task: None,
        };
        vm.refresh_status(cx);
        vm
    }

    pub fn refresh_status(&mut self, cx: &mut Context<Self>) {
        self.loading = true;
        cx.notify();

        let weak_self = cx.entity().downgrade();

        self._task = Some(cx.spawn(async move |_, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { hal::io::read_device_details() })
                .await;

            let _ = weak_self.update(cx, |this: &mut SecurityViewModel, cx| {
                match result {
                    Ok(status) => {
                        this.secure_boot_enabled = status.secure_boot;
                        this.secure_lock_enabled = status.secure_lock;
                        this.loading = false;
                        this.status_message = None;
                    }
                    Err(e) => {
                        this.loading = false;
                        this.status_message = Some(format!("Error: {}", e));
                    }
                }
                cx.notify();
            });
        }));
    }

    pub fn toggle_secure_boot(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.loading = true;
        self.status_message = Some(
            if enabled {
                "Enabling secure boot..."
            } else {
                "Disabling secure boot..."
            }
            .into(),
        );
        cx.notify();

        let weak_self = cx.entity().downgrade();

        self._task = Some(cx.spawn(async move |_, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { hal::io::enable_secure_boot(enabled) })
                .await;

            let _ = weak_self.update(cx, |this: &mut SecurityViewModel, cx| {
                match result {
                    Ok(msg) => {
                        this.secure_boot_enabled = enabled;
                        this.loading = false;
                        this.status_message = Some(msg);
                    }
                    Err(e) => {
                        this.loading = false;
                        this.status_message = Some(format!("Error: {}", e));
                    }
                }
                cx.notify();
            });
        }));
    }
}

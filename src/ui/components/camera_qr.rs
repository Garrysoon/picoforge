use std::sync::{Arc, Mutex};
use std::thread;

use gpui::*;
use gpui_component::{
    ActiveTheme,
    input::{Input, InputState},
    v_flex,
};

use crate::t;

pub struct CameraQrScanner {
    url_input: Entity<InputState>,
    camera_status: Arc<Mutex<Option<String>>>,
    found_uri: Arc<Mutex<Option<String>>>,
}

impl CameraQrScanner {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let url_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("otpauth://totp/...")
        });

        let camera_status = Arc::new(Mutex::new(None::<String>));
        let found_uri = Arc::new(Mutex::new(None::<String>));

        let cs = camera_status.clone();
        let fu = found_uri.clone();
        thread::spawn(move || {
            start_camera_thread(cs, fu);
        });

        Self {
            url_input,
            camera_status,
            found_uri,
        }
    }

    pub fn get_uri(&self, cx: &App) -> Option<String> {
        let val = self.url_input.read(cx).value().to_string();
        if val.starts_with("otpauth://") {
            Some(val)
        } else {
            None
        }
    }
}

fn start_camera_thread(
    status: Arc<Mutex<Option<String>>>,
    found_uri: Arc<Mutex<Option<String>>>,
) {
    use nokhwa::pixel_format::RgbFormat;
    use nokhwa::utils::{ApiBackend, RequestedFormat, RequestedFormatType};

    *status.lock().unwrap() = Some("Scanning...".into());

    let cameras = match nokhwa::query(ApiBackend::Auto) {
        Ok(c) => c,
        Err(e) => {
            *status.lock().unwrap() = Some(format!("Error: {}", e));
            return;
        }
    };

    if cameras.is_empty() {
        *status.lock().unwrap() = Some("No camera found".into());
        return;
    }

    let cam_desc = &cameras[0];
    let idx = cam_desc.index().clone();

    let mut camera = match nokhwa::Camera::new(
        idx,
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate),
    ) {
        Ok(c) => c,
        Err(e) => {
            *status.lock().unwrap() = Some(format!("Error: {}", e));
            return;
        }
    };

    if let Err(e) = camera.open_stream() {
        *status.lock().unwrap() = Some(format!("Error: {}", e));
        return;
    }

    let mut found = false;

    for _ in 0..120 {
        if found_uri.lock().unwrap().is_some() {
            break;
        }

        thread::sleep(std::time::Duration::from_millis(250));

        let frame = match camera.frame() {
            Ok(f) => f,
            Err(_) => continue,
        };

        let rgb_image = match frame.decode_image::<RgbFormat>() {
            Ok(img) => img,
            Err(_) => continue,
        };

        let (w, h) = rgb_image.dimensions();
        let w = w as usize;
        let h = h as usize;

        let mut prepared =
            rqrr::PreparedImage::prepare_from_greyscale(w, h, |x, y| {
                let pixel = rgb_image.get_pixel(x as u32, y as u32);
                let r = pixel[0] as f32;
                let g = pixel[1] as f32;
                let b = pixel[2] as f32;
                (0.299 * r + 0.587 * g + 0.114 * b) as u8
            });

        for grid in prepared.detect_grids() {
            if let Ok((_meta, content)) = grid.decode() {
                if content.starts_with("otpauth://") {
                    *found_uri.lock().unwrap() = Some(content);
                    *status.lock().unwrap() = Some("Found!".into());
                    found = true;
                    break;
                }
            }
        }

        if found {
            break;
        }
    }

    if !found && found_uri.lock().unwrap().is_none() {
        *status.lock().unwrap() = Some("No QR code detected".into());
    }
    let _ = camera.stop_stream();
}

impl Render for CameraQrScanner {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Check for found URI before borrowing cx for theme
        if let Some(uri) = self.found_uri.lock().unwrap().take() {
            self.url_input.update(cx, |inp, cx| {
                inp.set_value(uri.clone(), window, cx);
            });
            *self.camera_status.lock().unwrap() = Some(format!("Found: {}", uri));
        }

        let theme = cx.theme();

        let status = self
            .camera_status
            .lock()
            .unwrap()
            .clone()
            .unwrap_or_default();
        let scanning = status == "Scanning...";

        v_flex()
            .gap_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h(px(200.0))
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.border.clone())
                    .bg(theme.muted.clone())
                    .child(
                        v_flex()
                            .gap_2()
                            .items_center()
                            .justify_center()
                            .w_full()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.muted_foreground.clone())
                                    .child(t!("oath-import-qr-camera-hint")),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if scanning {
                                        theme.primary.clone()
                                    } else {
                                        theme.muted_foreground.clone()
                                    })
                                    .child(status),
                            ),
                    ),
            )
            .child(div().h_px().bg(theme.border.clone()))
            .child(
                v_flex()
                    .gap_2()
                    .child(t!("oath-import-qr-paste"))
                    .child(Input::new(&self.url_input)),
            )
    }
}

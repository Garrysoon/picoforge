//! QR code scanner for otpauth:// URI import — URL paste with optional camera.

use gpui::*;
use gpui_component::{
    ActiveTheme,
    h_flex,
    input::{Input, InputState},
    v_flex,
};

use crate::t;

pub struct CameraQrScanner {
    url_input: Entity<InputState>,
}

impl CameraQrScanner {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let url_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("otpauth://totp/...")
        });

        Self { url_input }
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

impl Render for CameraQrScanner {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

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
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.muted_foreground.clone())
                                    .child(t!("oath-import-qr-camera-hint")),
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

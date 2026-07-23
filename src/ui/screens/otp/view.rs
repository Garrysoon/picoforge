use crate::ui::components::page_view::PageView;
use crate::ui::screens::otp::view_model::{OtpCredential, OtpType, OtpViewModel};
use crate::t;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    ActiveTheme, Icon, Sizable, StyledExt, WindowExt,
    button::{Button, ButtonVariants},
    h_flex,
    input::{Input, InputState},
    v_flex,
};

impl OtpViewModel {
    fn render_credential_row(&self, cred: &OtpCredential, cx: &mut Context<Self>) -> impl IntoElement {
        let code = Self::generate_code(cred);
        let name = cred.name.clone();
        let otp_type = match cred.otp_type {
            OtpType::Totp => "TOTP",
            OtpType::Hotp => "HOTP",
        };
        let theme = cx.theme().clone();
        let bg = theme.secondary.clone();

        let delete_listener = cx.listener(move |this, _, _, cx| {
            this.remove_credential(&name);
            cx.notify();
        });

        div()
            .id(SharedString::from(format!("otp-{}", cred.name)))
            .border_1()
            .border_color(theme.border.clone())
            .rounded_lg()
            .bg(bg)
            .p_4()
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .child(
                        h_flex()
                            .gap_3()
                            .items_center()
                            .child(
                                div()
                                    .size_10()
                                    .rounded_md()
                                    .bg(rgb(0x3b3b3e))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::default()
                                            .path("icons/key-round.svg")
                                            .text_color(theme.primary.clone())
                                            .size_5(),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .child(div().font_semibold().child(cred.name.clone()))
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.muted_foreground.clone())
                                                    .child(otp_type),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.muted_foreground.clone())
                                                    .child(format!("{} digits", cred.digits)),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_3()
                            .items_center()
                            .child(
                                div()
                                    .font_family("Mono")
                                    .text_lg()
                                    .font_bold()
                                    .child(code),
                            )
                            .child(
                                Button::new(SharedString::from(format!("otp-del-{}", cred.name)))
                                    .ghost()
                                    .small()
                                    .child(
                                        Icon::default()
                                            .path("icons/trash-2.svg")
                                            .size_4()
                                            .text_color(theme.muted_foreground.clone()),
                                    )
                                    .on_click(delete_listener),
                            ),
                    ),
            )
    }
}

impl Render for OtpViewModel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();

        let has_creds = !self.credentials.is_empty();
        let cred_count = self.credentials.len();

        let creds_element: AnyElement = if has_creds {
            let rows: Vec<AnyElement> = self
                .credentials
                .iter()
                .map(|c| self.render_credential_row(c, cx).into_any_element())
                .collect();
            v_flex().gap_3().children(rows).into_any_element()
        } else {
            v_flex()
                .items_center()
                .gap_4()
                .child(
                    div()
                        .rounded_full()
                        .bg(theme.muted.clone())
                        .p_4()
                        .child(
                            Icon::default()
                                .path("icons/key-round.svg")
                                .size_8()
                                .text_color(theme.muted_foreground.clone()),
                        ),
                )
                .child(div().text_lg().font_semibold().child(t!("otp-no-credentials")))
                .child(
                    div()
                        .text_color(theme.muted_foreground.clone())
                        .text_sm()
                        .text_center()
                        .max_w(px(384.0))
                        .child(t!("otp-no-credentials-desc")),
                )
                .into_any_element()
        };

        let content = v_flex()
            .gap_4()
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.muted_foreground.clone())
                            .child(t!("oath-credentials-stored").replace("{count}", &cred_count.to_string())),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("add-otp-btn")
                                    .label(t!("common-add"))
                                    .primary()
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_add_dialog(window, cx);
                                    })),
                            )
                            .child(
                                Button::new("import-qr-btn")
                                    .label(t!("oath-import-qr"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_qr_import_dialog(window, cx);
                                    })),
                            ),
                    ),
            )
            .child(
                div()
                    .border_1()
                    .border_color(theme.border.clone())
                    .rounded_xl()
                    .when(!has_creds, |d| d.py_12())
                    .child(creds_element),
            );

        PageView::build(
            t!("otp-title"),
            t!("otp-subtitle"),
            content,
            &theme,
        )
        .into_any_element()
    }
}

impl OtpViewModel {
    fn open_add_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let name_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("e.g. Steam:account")
        });
        let secret_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("Base32 secret key")
        });

        let entity = cx.entity().clone();
        let name_for_dialog = name_input.clone();
        let secret_for_dialog = secret_input.clone();

        window.open_dialog(cx, move |dialog, _, _| {
            let name_clone = name_for_dialog.clone();
            let secret_clone = secret_for_dialog.clone();
            let entity_clone = entity.clone();

            dialog
                .title(t!("otp-add"))
                .child(
                    v_flex()
                        .gap_4()
                        .child(
                            v_flex()
                                .gap_2()
                                .child(t!("oath-account-name"))
                                .child(Input::new(&name_clone)),
                        )
                        .child(
                            v_flex()
                                .gap_2()
                                .child(t!("oath-secret-key"))
                                .child(Input::new(&secret_clone)),
                        ),
                )
                .on_ok(move |_, _, cx| {
                    let name = name_clone.read(cx).value().to_string();
                    let secret_b32 = secret_clone.read(cx).value().to_string();
                    if name.is_empty() || secret_b32.is_empty() {
                        return true;
                    }
                    let secret = base32_decode(&secret_b32);
                    if secret.is_empty() {
                        return true;
                    }
                    let _ = entity_clone.update(cx, |this: &mut OtpViewModel, cx| {
                        this.add_credential(OtpCredential {
                            name,
                            secret,
                            otp_type: OtpType::Totp,
                            digits: 6,
                            period: 30,
                            counter: 0,
                        });
                        cx.notify();
                    });
                    true
                })
        });
    }

    fn open_qr_import_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let url_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("otpauth://totp/...")
        });
        let entity = cx.entity().clone();
        let url_for_dialog = url_input.clone();

        window.open_dialog(cx, move |dialog, _, _| {
            let url_clone = url_for_dialog.clone();
            let entity_clone = entity.clone();

            dialog
                .title(t!("oath-import-qr"))
                .child(
                    v_flex()
                        .gap_4()
                        .child(
                            v_flex()
                                .gap_2()
                                .child(t!("oath-import-qr-paste"))
                                .child(Input::new(&url_clone)),
                        ),
                )
                .on_ok(move |_, _, cx| {
                    let url = url_clone.read(cx).value().to_string();
                    if url.is_empty() {
                        return true;
                    }
                    let _ = entity_clone.update(cx, |this: &mut OtpViewModel, cx| {
                        if let Some(cred) = OtpViewModel::parse_otpauth_uri(&url) {
                            this.add_credential(cred);
                        }
                        cx.notify();
                    });
                    true
                })
        });
    }
}

fn base32_decode(s: &str) -> Vec<u8> {
    use base64::Engine;
    let s = s.trim_end_matches('=');
    let mut padded = s.to_string();
    let padding = (8 - padded.len() % 8) % 8;
    padded.push_str(&"=".repeat(padding));
    base64::engine::general_purpose::STANDARD
        .decode(padded.as_bytes())
        .unwrap_or_default()
}

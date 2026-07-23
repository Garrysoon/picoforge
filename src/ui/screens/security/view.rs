use crate::ui::components::page_view::PageView;
use crate::ui::screens::security::view_model::SecurityViewModel;
use crate::t;
use gpui::*;
use gpui_component::{
    ActiveTheme, Disableable, Icon, StyledExt,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    h_flex,
    switch::Switch,
    v_flex,
};

impl Render for SecurityViewModel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let fg = theme.foreground.clone();
        let muted_fg = theme.muted_foreground.clone();
        let border = theme.border.clone();
        let card_bg = theme.secondary.clone();

        let destructive_red = rgb(0xef4444);
        let destructive_red_hover = rgb(0xdc2626);
        let destructive_red_active = rgb(0xb91c1c);
        let destructive_border = rgba(0xef44444d);
        let destructive_bg_muted = rgba(0xef44441a);

        let secure_boot = self.secure_boot_enabled;
        let secure_lock = self.secure_lock_enabled;
        let loading = self.loading;
        let status_msg = self.status_message.clone();
        let is_error = status_msg.as_ref().map(|m| m.starts_with("Error")).unwrap_or(false);

        let content = v_flex()
            .gap_6()
            .w_full()
            .children(status_msg.map(|msg| {
                let (icon_path, icon_color, _bg_color, border_color) = if is_error {
                    ("icons/triangle-alert.svg", destructive_red, destructive_bg_muted, destructive_border)
                } else if msg.contains("...") {
                    ("icons/loader.svg", rgb(0xf59e0b), rgba(0xf59e0b1a), rgba(0xf59e0b4d))
                } else {
                    ("icons/check-circle.svg", rgb(0x22c55e), rgba(0x22c55e1a), rgba(0x22c55e4d))
                };

                h_flex()
                    .w_full()
                    .p_4()
                    .gap_3()
                    .items_center()
                    .border_1()
                    .border_color(border_color)
                    .bg(card_bg.clone())
                    .rounded_md()
                    .child(Icon::default().path(icon_path).text_color(icon_color))
                    .child(
                        div()
                            .text_sm()
                            .font_medium()
                            .child(msg),
                    )
                    .into_any_element()
            }))
            .child(
                v_flex()
                    .w_full()
                    .border_1()
                    .border_color(border.clone())
                    .bg(card_bg.clone())
                    .rounded_xl()
                    .overflow_hidden()
                    .child(
                        div().p_6().child(
                            div()
                                .text_lg()
                                .font_bold()
                                .text_color(fg.clone())
                                .child(t!("security-title")),
                        ),
                    )
                    .child(
                        v_flex()
                            .px_6()
                            .pb_6()
                            .gap_6()
                            .child(
                                h_flex()
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_medium()
                                                    .child(t!("security-enable-secure-boot")),
                                            )
                                            .child(
                                                div().text_xs().text_color(muted_fg.clone()).child(
                                                    t!("security-verify-firmware"),
                                                ),
                                            ),
                                    )
                                    .child({
                                        let sb = secure_boot;
                                        Switch::new("secure-boot-switch")
                                            .checked(sb)
                                            .disabled(loading)
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.toggle_secure_boot(!sb, cx);
                                            }))
                                    }),
                            )
                            .child(
                                h_flex()
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .child(
                                                div().text_sm().font_medium().child(t!("security-secure-lock")),
                                            )
                                            .child(div().text_xs().text_color(muted_fg.clone()).child(
                                                t!("security-prevent-debug"),
                                            )),
                                    )
                                    .child({
                                        let sl = secure_lock;
                                        Switch::new("secure-lock-switch")
                                            .checked(sl)
                                            .disabled(loading)
                                    }),
                            )
                            .child(div().h_px().bg(border.clone()))
                            .child(
                                h_flex()
                                    .items_center()
                                    .gap_4()
                                    .p_4()
                                    .rounded_md()
                                    .bg(destructive_bg_muted)
                                    .border_1()
                                    .border_color(destructive_border)
                                    .child(
                                        Switch::new("confirm-switch").checked(false).disabled(true),
                                    )
                                    .child(
                                        div()
                                            .font_medium()
                                            .text_color(destructive_red)
                                            .child(t!("security-understand-risks")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .border_t_1()
                            .border_color(border.clone())
                            .bg(gpui::rgba(0x00000033))
                            .px_6()
                            .py_4()
                            .flex()
                            .justify_end()
                            .child(
                                Button::new("lock-device-btn")
                                    .custom(
                                        ButtonCustomVariant::new(cx)
                                            .color(destructive_red.into())
                                            .hover(destructive_red_hover.into())
                                            .active(destructive_red_active.into()),
                                    )
                                    .disabled(true)
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .items_center()
                                            .child(Icon::default().path("icons/lock.svg").size_4())
                                            .child(t!("security-permanently-lock")),
                                    ),
                            ),
                    ),
            );

        PageView::build(
            t!("security-title"),
            t!("security-subtitle"),
            content,
            &theme,
        )
        .into_any_element()
    }
}

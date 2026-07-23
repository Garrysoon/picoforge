use crate::ui::components::page_view::PageView;
use crate::ui::screens::firmware::view_model::FirmwareViewModel;
use crate::t;
use gpui::*;
use gpui_component::{
    ActiveTheme, Disableable, StyledExt,
    button::{Button, ButtonVariants},
    h_flex,
    progress::Progress,
    v_flex,
};

impl Render for FirmwareViewModel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let fg = theme.foreground.clone();
        let muted_fg = theme.muted_foreground.clone();
        let card_bg = theme.secondary.clone();
        let border = theme.border.clone();

        let current_ver = self.current_version.clone().unwrap_or_else(|| t!("firmware-no-version").to_string());
        let latest_ver = self.latest_version.clone().unwrap_or_else(|| t!("firmware-not-checked").to_string());
        let has_update = self.update_available;
        let flashing = self.flashing;
        let progress = self.progress;
        let status_msg = self.status_message.clone();

        let content = v_flex()
            .gap_6()
            .w_full()
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
                                .child(t!("firmware-information")),
                        ),
                    )
                    .child(
                        v_flex()
                            .px_6()
                            .pb_6()
                            .gap_4()
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(div().text_sm().text_color(muted_fg.clone()).child(t!("firmware-current-version")))
                                    .child(div().text_sm().font_medium().child(current_ver)),
                            )
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(div().text_sm().text_color(muted_fg.clone()).child(t!("firmware-latest-version")))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_medium()
                                            .child(latest_ver),
                                    ),
                            )
                            .child(div().h_px().bg(border.clone()))
                            .child(
                                h_flex()
                                    .gap_3()
                                    .child(
                                        Button::new("check-updates-btn")
                                            .label(t!("firmware-check-updates"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.check_for_updates(cx);
                                            })),
                                    )
                                    .child(
                                        Button::new("flash-btn")
                                            .label(t!("firmware-flash"))
                                            .primary()
                                            .disabled(!has_update || flashing)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.start_flash(cx);
                                            })),
                                    ),
                            ),
                    ),
            )
            .children(status_msg.map(|msg| {
                h_flex()
                    .gap_2()
                    .items_center()
                    .p_4()
                    .border_1()
                    .border_color(theme.muted.clone())
                    .rounded_lg()
                    .bg(card_bg.clone())
                    .child(div().text_sm().child(msg))
                    .into_any_element()
            }))
            .children(if flashing || progress > 0.0 {
                Some(
                    v_flex()
                        .gap_2()
                        .child(
                            h_flex()
                                .justify_between()
                                .child(div().text_sm().text_color(muted_fg.clone()).child(t!("firmware-flash-progress")))
                                .child(div().text_sm().child(format!("{}%", (progress * 100.0) as u32))),
                        )
                        .child(Progress::new().value(progress)),
                )
            } else {
                None
            });

        PageView::build(
            t!("firmware-title"),
            t!("firmware-subtitle"),
            content,
            &theme,
        )
        .into_any_element()
    }
}

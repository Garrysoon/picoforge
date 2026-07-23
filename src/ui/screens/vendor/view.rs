use crate::ui::components::page_view::PageView;
use crate::ui::screens::vendor::view_model::VendorViewModel;
use crate::t;
use gpui::*;
use gpui_component::{
    ActiveTheme, StyledExt,
    button::Button,
    h_flex,
    scroll::ScrollableElement,
    v_flex,
};

impl Render for VendorViewModel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let fg = theme.foreground.clone();
        let muted_fg = theme.muted_foreground.clone();
        let card_bg = theme.secondary.clone();
        let border = theme.border.clone();

        let export_status = self.export_status.clone();
        let log_count = self.logs.len();

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
                                .child(t!("vendor-operations")),
                        ),
                    )
                    .child(
                        v_flex()
                            .px_6()
                            .pb_6()
                            .gap_4()
                            .child(
                                h_flex()
                                    .gap_3()
                                    .child(
                                        Button::new("export-oath-btn")
                                            .label(t!("vendor-export-oath"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.export_oath_credentials(cx);
                                            })),
                                    )
                                    .child(
                                        Button::new("export-otp-btn")
                                            .label(t!("vendor-export-otp"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.export_otp_credentials(cx);
                                            })),
                                    )
                                    .child(
                                        Button::new("vendor-backup")
                                            .label(t!("vendor-backup")),
                                    )
                                    .child(
                                        Button::new("vendor-restore")
                                            .label(t!("vendor-restore")),
                                    ),
                            )
                            .children(export_status.map(|status| {
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .p_3()
                                    .rounded_md()
                                    .bg(gpui::rgba(0x22c55e1a))
                                    .border_1()
                                    .border_color(gpui::rgba(0x22c55e4d))
                                    .child(div().text_sm().child(status))
                                    .into_any_element()
                            })),
                    ),
            )
            .child(
                v_flex()
                    .w_full()
                    .border_1()
                    .border_color(border.clone())
                    .bg(card_bg.clone())
                    .rounded_xl()
                    .overflow_hidden()
                    .child(
                        h_flex()
                            .justify_between()
                            .items_center()
                            .p_6()
                            .child(
                                div()
                                    .text_lg()
                                    .font_bold()
                                    .text_color(fg.clone())
                                    .child(t!("vendor-logs")),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(muted_fg.clone())
                                            .child(t!("vendor-logs-entries").replace("{count}", &log_count.to_string())),
                                    )
                                    .child(
                                        Button::new("clear-logs-btn")
                                            .label(t!("vendor-logs-clear"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.clear_logs(cx);
                                            })),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .p_4()
                            .max_h(px(400.0))
                            .overflow_y_scrollbar()
                            .children(self.logs.iter().enumerate().rev().map(|(i, log)| {
                                div()
                                    .id(("log-entry", i))
                                    .font_family("Mono")
                                    .text_xs()
                                    .py_1()
                                    .border_b_1()
                                    .border_color(gpui::rgba(0xffffff0d))
                                    .child(log.clone())
                                    .into_any_element()
                            })),
                    ),
            );

        PageView::build(
            t!("vendor-title"),
            t!("vendor-subtitle"),
            content,
            &theme,
        )
        .into_any_element()
    }
}

use egui::{panel::TopBottomSide, Context, CursorIcon, TopBottomPanel, Visuals};

pub fn nav_panel(_client: &mut crate::YorCloudApp, ctx: &Context) {
    TopBottomPanel::new(TopBottomSide::Top, "nav_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Yor Cloud")
                .on_hover_ui(|ui| {
                    ui.label("thank you for using YorCloud!");
                })
                .on_hover_cursor(CursorIcon::Text);

            let is_dark = ui.visuals().dark_mode;
            let text = if !is_dark { "🌙" } else { "☀" };
            if ui.selectable_label(false, text).clicked() {
                if is_dark {
                    ctx.set_visuals(Visuals::light());
                } else {
                    ctx.set_visuals(Visuals::dark());
                }
            };
        });
    });
}

use egui::Context;

pub fn status_bar(_client: &mut crate::YorCloudApp, ctx: &Context) {
    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Logged in as: Qat");

            ui.label("connected to https://cloud.qatto.dev");
            //                self.assets.status[&Status::Completed].show_size(ui, ICON_SIZE / 1.8);
        });
    });
}

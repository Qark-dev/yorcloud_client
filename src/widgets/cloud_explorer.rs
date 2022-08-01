use egui::Context;

pub fn cloud_explorer(_client: &mut crate::YorCloudApp, ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("hello");
    });
}

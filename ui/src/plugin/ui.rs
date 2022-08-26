use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, FontData, FontDefinitions, FontFamily, RichText, Rounding},
    EguiContext,
};

use crate::resource::Log;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_startup_system(configure_visuals)
            .add_system(ui);
    }
}

fn setup(mut egui_ctx: ResMut<EguiContext>) {
    let font = include_bytes!("../../assets/fonts/Silkscreen-Regular.ttf");

    let mut fonts = FontDefinitions::default();

    fonts
        .font_data
        .insert("default".to_string(), FontData::from_static(font));

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "default".to_string());

    egui_ctx.ctx_mut().set_fonts(fonts);
}

fn ui(mut egui_ctx: ResMut<EguiContext>, windows: Res<Windows>, log: Res<Log>) {
    let window = match windows.get_primary() {
        Some(it) => it,
        None => return,
    };

    let text = RichText::new(log.read()).size(18.0);

    let width = window.width();
    let height = window.height();
    let margin = 5.0;

    egui::Window::new("Messages")
        .anchor(Align2::CENTER_BOTTOM, [0.0, -margin])
        .fixed_size((width - margin * 4.0, height * 0.25))
        .title_bar(false)
        .frame(egui::Frame {
            stroke: (2.0, egui::Color32::GRAY).into(),
            fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 204),
            inner_margin: 5.0.into(),
            ..default()
        })
        .vscroll(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            egui::ScrollArea::vertical()
                .always_show_scroll(true)
                .stick_to_bottom(true)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.label(text);
                });
        });
}

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: Rounding::none(),
        ..Default::default()
    });
}

use crate::{AdminCommandState, EguiAction, EguiActions};
use common::cmd::ServerChatCommand;
use egui::{CollapsingHeader, Context, Resize, Slider, Ui, Vec2, Window, style::Margin};
use lazy_static::lazy_static;

lazy_static! {
    static ref ITEM_SPECS: Vec<String> = {
        let mut item_specs = common::cmd::ITEM_SPECS
            .iter()
            .map(|item_desc| item_desc.replace("common.items.", ""))
            .collect::<Vec<String>>();
        item_specs.sort();
        item_specs
    };
}
lazy_static! {
    static ref ENTITY_CONFIGS: Vec<String> = {
        let mut entity_configs = common::cmd::ENTITY_CONFIGS
            .iter()
            .map(|entity_desc| entity_desc.replace("common.entity.", ""))
            .collect::<Vec<String>>();
        entity_configs.sort();
        entity_configs
    };
}

pub fn draw_admin_commands_window(
    ctx: &Context,
    state: &mut AdminCommandState,
    open: &mut bool,
    egui_actions: &mut EguiActions,
) {
    Window::new("Admin Commands")
        .open(open)
        .default_width(400.0)
        .default_height(600.0)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(10.0, 10.0);
            ui.vertical(|ui| {
                CollapsingHeader::new("Give Items")
                    .default_open(true)
                    .show(ui, |ui| {
                        draw_give_items(ui, state, egui_actions);
                    });
                CollapsingHeader::new("Kits")
                    .default_open(false)
                    .show(ui, |ui| {
                        draw_kits(ui, state, egui_actions);
                    });
                CollapsingHeader::new("Spawn Entities")
                    .default_open(false)
                    .show(ui, |ui| {
                        draw_spawn_entities(ui, state, egui_actions);
                    })
            });
        });
}

fn draw_kits(ui: &mut Ui, state: &mut AdminCommandState, egui_actions: &mut EguiActions) {
    ui.vertical(|ui| {
        if ui.button("Give Kit").clicked() {
            egui_actions.actions.push(EguiAction::ChatCommand {
                cmd: ServerChatCommand::Kit,
                args: vec![common::cmd::KITS[state.kits_selected_idx].clone()],
            });
        };
        crate::widgets::filterable_list(ui, &common::cmd::KITS, "", &mut state.kits_selected_idx)
    });
}

fn draw_give_items(ui: &mut Ui, state: &mut AdminCommandState, egui_actions: &mut EguiActions) {
    ui.spacing_mut().window_margin = Margin::same(10.0);
    Resize::default()
        .default_size([400.0, 200.0])
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    Slider::new(&mut state.give_item_qty, 1..=100000)
                        .logarithmic(true)
                        .clamp_to_range(true)
                        .text("Qty"),
                );
                if ui.button("Give Items").clicked() {
                    egui_actions.actions.push(EguiAction::ChatCommand {
                        cmd: ServerChatCommand::GiveItem,
                        args: vec![
                            format!(
                                "common.items.{}",
                                ITEM_SPECS[state.give_item_selected_idx].clone()
                            ),
                            format!("{}", state.give_item_qty),
                        ],
                    });
                };
            });
            ui.horizontal(|ui| {
                ui.label("Filter:");

                ui.text_edit_singleline(&mut state.give_item_search_text);
            });

            crate::widgets::filterable_list(
                ui,
                &ITEM_SPECS,
                &state.give_item_search_text,
                &mut state.give_item_selected_idx,
            );
        });
}
fn draw_spawn_entities(ui: &mut Ui, state: &mut AdminCommandState, egui_actions: &mut EguiActions) {
    ui.spacing_mut().window_margin = Margin::same(10.0);
    Resize::default()
        .default_size([400.0, 200.0])
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    Slider::new(&mut state.spawn_entity_qty, 1..=49)
                        .logarithmic(true)
                        .clamp_to_range(true)
                        .text("Qty"),
                );
                if ui.button("Spawn Entities").clicked() {
                    egui_actions.actions.push(EguiAction::ChatCommand {
                        cmd: ServerChatCommand::MakeNpc,
                        args: vec![
                            format!(
                                "common.entity.{}",
                                ENTITY_CONFIGS[state.spawn_entity_selected_idx].clone()
                            ),
                            format!("{}", state.spawn_entity_qty),
                        ],
                    });
                };
            });
            ui.horizontal(|ui| {
                ui.label("Filter:");

                ui.text_edit_singleline(&mut state.spawn_entity_search_text);
            });

            crate::widgets::filterable_list(
                ui,
                &ENTITY_CONFIGS,
                &state.spawn_entity_search_text,
                &mut state.spawn_entity_selected_idx,
            );
        });
}

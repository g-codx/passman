use eframe::egui::{
    self, Color32, CornerRadius, Frame, Id, Key, Modifiers, Response, Sense, Stroke, TextEdit, Ui,
    Vec2, Visuals,
};

const TOOLBAR_ITEM_HEIGHT: f32 = 20.0;

fn toolbar_frame(visuals: &Visuals) -> Frame {
    Frame::new()
        .fill(visuals.extreme_bg_color)
        .stroke(Stroke::new(
            1.0,
            visuals.widgets.inactive.bg_stroke.color,
        ))
        .corner_radius(CornerRadius::same(7))
        .inner_margin(egui::Margin::symmetric(4, 2))
        .outer_margin(egui::Margin::symmetric(0, 5))
}

pub fn toolbar_item<R>(ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> R {
    let visuals = ui.style().visuals.clone();
    toolbar_frame(&visuals)
        .show(ui, |ui| {
            ui.set_height(TOOLBAR_ITEM_HEIGHT);
            content(ui)
        })
        .inner
}

pub fn search_bar(ui: &mut Ui, query: &mut String) -> Response {
    let search_id = Id::new("vault_search");
    let visuals = ui.style().visuals.clone();
    let text_color = visuals.strong_text_color();
    let hint_color = visuals.weak_text_color();

    if ui.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::F)) {
        ui.memory_mut(|mem| mem.request_focus(search_id));
    }

    let bar_width = ui.available_width().max(80.0);
    let edit_width = (bar_width - 52.0).max(40.0);

    toolbar_frame(&visuals).show(ui, |ui| {
            ui.set_width(bar_width);
            ui.set_height(TOOLBAR_ITEM_HEIGHT);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                ui.label(egui::RichText::new("🔍").color(hint_color));

                let edit = TextEdit::singleline(query)
                    .id(search_id)
                    .desired_width(edit_width)
                    .min_size(Vec2::new(edit_width, TOOLBAR_ITEM_HEIGHT - 8.0))
                    .frame(Frame::NONE)
                    .text_color(text_color)
                    .hint_text(egui::RichText::new("Search…").color(hint_color));

                let response = ui.add(edit);

                if !query.is_empty() {
                    if ui
                        .add(
                            egui::Label::new(egui::RichText::new("✖").color(hint_color))
                                .sense(Sense::click()),
                        )
                        .on_hover_text("Clear")
                        .clicked()
                    {
                        query.clear();
                        response.request_focus();
                    }
                }

                response
            })
            .inner
        })
        .inner
}

pub fn context_menu_action(ui: &mut Ui, icon: &str, label: &str, destructive: bool) -> bool {
    let visuals = ui.style().visuals.clone();
    let text_color = if destructive {
        Color32::from_rgb(200, 72, 72)
    } else {
        visuals.text_color()
    };

    let label = egui::RichText::new(format!("{icon}  {label}")).color(text_color);
    ui.selectable_label(false, label).clicked()
}

pub fn context_menu_separator(ui: &mut Ui) {
    ui.add_space(4.0);
    ui.separator();
    ui.add_space(4.0);
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntryMenuAction {
    Copy,
    Edit,
    Remove,
}

pub fn entry_context_menu(ui: &mut Ui, copy_label: &str) -> Option<EntryMenuAction> {
    ui.set_min_width(180.0);

    if context_menu_action(ui, "📋", copy_label, false) {
        ui.close();
        return Some(EntryMenuAction::Copy);
    }

    context_menu_separator(ui);

    if context_menu_action(ui, "✏", "Edit", false) {
        ui.close();
        return Some(EntryMenuAction::Edit);
    }

    if context_menu_action(ui, "🗑", "Remove", true) {
        ui.close();
        return Some(EntryMenuAction::Remove);
    }

    None
}

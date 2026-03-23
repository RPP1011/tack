use egui::{self, RichText, Color32, Ui, Vec2};

use crate::state::WidgetState;

/// Core widget functions that mirror Streamlit's API.
/// Each function takes an egui `Ui`, a `WidgetState` for persistence,
/// and the widget parameters, then returns the current value.

// ── Text Display ──────────────────────────────────────────────────

pub fn title(ui: &mut Ui, text: &str) {
    ui.heading(RichText::new(text).size(32.0).strong());
    ui.add_space(4.0);
}

pub fn header(ui: &mut Ui, text: &str) {
    ui.heading(RichText::new(text).size(24.0).strong());
    ui.add_space(2.0);
}

pub fn subheader(ui: &mut Ui, text: &str) {
    ui.heading(RichText::new(text).size(18.0));
    ui.add_space(2.0);
}

pub fn text(ui: &mut Ui, text: &str) {
    ui.label(text);
}

pub fn caption(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).small().weak());
}

pub fn code(ui: &mut Ui, code_text: &str) {
    egui::Frame::new()
        .fill(ui.visuals().extreme_bg_color)
        .corner_radius(4.0)
        .inner_margin(8.0)
        .show(ui, |ui| {
            ui.label(RichText::new(code_text).monospace());
        });
    ui.add_space(4.0);
}

pub fn markdown(ui: &mut Ui, _text: &str) {
    // Basic markdown: just render as text for now.
    // A full markdown parser could be added later.
    ui.label(_text);
}

pub fn divider(ui: &mut Ui) {
    ui.separator();
}

// ── Interactive Widgets ───────────────────────────────────────────

pub fn button(ui: &mut Ui, label: &str) -> bool {
    ui.button(label).clicked()
}

pub fn checkbox(ui: &mut Ui, state: &mut WidgetState, label: &str, default: bool) -> bool {
    let mut val = state.get_bool(label).unwrap_or(default);
    ui.checkbox(&mut val, label);
    state.set_bool(label, val);
    val
}

pub fn toggle(ui: &mut Ui, state: &mut WidgetState, label: &str, default: bool) -> bool {
    let mut val = state.get_bool(label).unwrap_or(default);
    ui.horizontal(|ui| {
        ui.toggle_value(&mut val, label);
    });
    state.set_bool(label, val);
    val
}

pub fn radio(ui: &mut Ui, state: &mut WidgetState, label: &str, options: &[&str], default: usize) -> usize {
    let mut selected = state.get_usize(label).unwrap_or(default);
    ui.label(RichText::new(label).strong());
    for (i, option) in options.iter().enumerate() {
        ui.radio_value(&mut selected, i, *option);
    }
    state.set_usize(label, selected);
    selected
}

pub fn text_input(ui: &mut Ui, state: &mut WidgetState, label: &str, default: &str) -> String {
    let mut val = state.get_string(label).cloned().unwrap_or_else(|| default.to_string());
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(&mut val);
    });
    state.set_string(label, val.clone());
    val
}

pub fn text_area(ui: &mut Ui, state: &mut WidgetState, label: &str, default: &str) -> String {
    let mut val = state.get_string(label).cloned().unwrap_or_else(|| default.to_string());
    ui.label(RichText::new(label).strong());
    ui.add(egui::TextEdit::multiline(&mut val).desired_width(f32::INFINITY));
    state.set_string(label, val.clone());
    val
}

pub fn number_input(
    ui: &mut Ui,
    state: &mut WidgetState,
    label: &str,
    default: f64,
    min: f64,
    max: f64,
    step: f64,
) -> f64 {
    let mut val = state.get_f64(label).unwrap_or(default);
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::DragValue::new(&mut val).range(min..=max).speed(step));
    });
    state.set_f64(label, val);
    val
}

pub fn slider_f64(
    ui: &mut Ui,
    state: &mut WidgetState,
    label: &str,
    min: f64,
    max: f64,
    default: f64,
) -> f64 {
    let mut val = state.get_f64(label).unwrap_or(default);
    ui.add(egui::Slider::new(&mut val, min..=max).text(label));
    state.set_f64(label, val);
    val
}

pub fn slider_i32(
    ui: &mut Ui,
    state: &mut WidgetState,
    label: &str,
    min: i32,
    max: i32,
    default: i32,
) -> i32 {
    let mut val = state.get_f64(label).unwrap_or(default as f64) as i32;
    ui.add(egui::Slider::new(&mut val, min..=max).text(label));
    state.set_f64(label, val as f64);
    val
}

pub fn selectbox(ui: &mut Ui, state: &mut WidgetState, label: &str, options: &[&str], default: usize) -> usize {
    let mut selected = state.get_usize(label).unwrap_or(default);
    ui.horizontal(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt(label)
            .selected_text(options.get(selected).copied().unwrap_or(""))
            .show_ui(ui, |ui| {
                for (i, option) in options.iter().enumerate() {
                    ui.selectable_value(&mut selected, i, *option);
                }
            });
    });
    state.set_usize(label, selected);
    selected
}

pub fn multiselect(
    ui: &mut Ui,
    state: &mut WidgetState,
    label: &str,
    options: &[&str],
) -> Vec<usize> {
    ui.label(RichText::new(label).strong());
    let mut result = Vec::new();
    for (i, option) in options.iter().enumerate() {
        let key = format!("{}__{}", label, option);
        let mut checked = state.get_bool(&key).unwrap_or(false);
        ui.checkbox(&mut checked, *option);
        state.set_bool(&key, checked);
        if checked {
            result.push(i);
        }
    }
    result
}

pub fn progress(ui: &mut Ui, value: f32) {
    let bar = egui::ProgressBar::new(value.clamp(0.0, 1.0)).show_percentage();
    ui.add(bar);
}

pub fn spinner(ui: &mut Ui) {
    ui.spinner();
}

pub fn color_picker(ui: &mut Ui, state: &mut WidgetState, label: &str) -> [u8; 3] {
    let key_r = format!("{}_r", label);
    let key_g = format!("{}_g", label);
    let key_b = format!("{}_b", label);

    let r = state.get_f64(&key_r).unwrap_or(128.0) as u8;
    let g = state.get_f64(&key_g).unwrap_or(128.0) as u8;
    let b = state.get_f64(&key_b).unwrap_or(128.0) as u8;

    let mut color = Color32::from_rgb(r, g, b);
    ui.horizontal(|ui| {
        ui.label(label);
        ui.color_edit_button_srgba(&mut color);
    });

    let [r, g, b, _] = color.to_array();
    state.set_f64(&key_r, r as f64);
    state.set_f64(&key_g, g as f64);
    state.set_f64(&key_b, b as f64);
    [r, g, b]
}

// ── Layout ────────────────────────────────────────────────────────

pub fn columns<F>(ui: &mut Ui, count: usize, f: F)
where
    F: FnOnce(&mut [Ui]),
{
    ui.columns(count, |cols| {
        f(cols);
    });
}

// ── Data Display ──────────────────────────────────────────────────

pub fn metric(ui: &mut Ui, label: &str, value: &str, delta: Option<&str>) {
    ui.vertical(|ui| {
        ui.label(RichText::new(label).small().weak());
        ui.label(RichText::new(value).size(28.0).strong());
        if let Some(d) = delta {
            let (color, prefix) = if d.starts_with('-') {
                (Color32::from_rgb(255, 75, 75), "")
            } else {
                (Color32::from_rgb(75, 200, 75), "")
            };
            ui.label(RichText::new(format!("{}{}", prefix, d)).color(color).small());
        }
    });
    ui.add_space(4.0);
}

pub fn json(ui: &mut Ui, text: &str) {
    egui::Frame::new()
        .fill(ui.visuals().extreme_bg_color)
        .corner_radius(4.0)
        .inner_margin(8.0)
        .show(ui, |ui| {
            ui.label(RichText::new(text).monospace().size(13.0));
        });
    ui.add_space(4.0);
}

pub fn table(ui: &mut Ui, headers: &[&str], rows: &[Vec<String>]) {
    use egui_extras::{TableBuilder, Column};
    let available_width = ui.available_width();
    let col_width = available_width / headers.len() as f32;

    TableBuilder::new(ui)
        .striped(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .columns(Column::initial(col_width).resizable(true), headers.len())
        .header(24.0, |mut header| {
            for h in headers {
                header.col(|ui| {
                    ui.strong(*h);
                });
            }
        })
        .body(|mut body| {
            for row_data in rows {
                body.row(20.0, |mut row| {
                    for cell in row_data {
                        row.col(|ui| {
                            ui.label(cell.as_str());
                        });
                    }
                });
            }
        });
    ui.add_space(4.0);
}

// ── Plotting ──────────────────────────────────────────────────────

pub fn line_chart(ui: &mut Ui, label: &str, points: &[(f64, f64)]) {
    let pts: Vec<[f64; 2]> = points.iter().map(|&(x, y)| [x, y]).collect();
    let line = egui_plot::Line::new(
        egui_plot::PlotPoints::new(pts),
    ).name(label);
    egui_plot::Plot::new(label)
        .height(200.0)
        .show(ui, |plot_ui| {
            plot_ui.line(line);
        });
    ui.add_space(4.0);
}

pub fn bar_chart(ui: &mut Ui, label: &str, bars: &[(f64, f64)]) {
    let bar_elems: Vec<egui_plot::Bar> = bars
        .iter()
        .map(|(x, h)| egui_plot::Bar::new(*x, *h))
        .collect();
    let chart = egui_plot::BarChart::new(bar_elems).name(label);
    egui_plot::Plot::new(label)
        .height(200.0)
        .show(ui, |plot_ui| {
            plot_ui.bar_chart(chart);
        });
    ui.add_space(4.0);
}

pub fn scatter(ui: &mut Ui, label: &str, points: &[(f64, f64)]) {
    let arr: Vec<[f64; 2]> = points.iter().map(|&(x, y)| [x, y]).collect();
    let pts = egui_plot::Points::new(
        arr,
    ).name(label).radius(4.0);
    egui_plot::Plot::new(label)
        .height(200.0)
        .show(ui, |plot_ui| {
            plot_ui.points(pts);
        });
    ui.add_space(4.0);
}

// ── Notifications ─────────────────────────────────────────────────

pub fn success(ui: &mut Ui, text: &str) {
    notification(ui, text, Color32::from_rgb(75, 200, 75));
}

pub fn info(ui: &mut Ui, text: &str) {
    notification(ui, text, Color32::from_rgb(75, 150, 255));
}

pub fn warning(ui: &mut Ui, text: &str) {
    notification(ui, text, Color32::from_rgb(255, 200, 50));
}

pub fn error(ui: &mut Ui, text: &str) {
    notification(ui, text, Color32::from_rgb(255, 75, 75));
}

fn notification(ui: &mut Ui, text: &str, color: Color32) {
    egui::Frame::new()
        .fill(color.gamma_multiply(0.15))
        .stroke(egui::Stroke::new(1.0, color))
        .corner_radius(4.0)
        .inner_margin(8.0)
        .show(ui, |ui| {
            ui.colored_label(color, text);
        });
    ui.add_space(4.0);
}

// ── Images ────────────────────────────────────────────────────────

pub fn image(ui: &mut Ui, uri: &str, size: Option<Vec2>) {
    let img = egui::Image::new(uri);
    let img = if let Some(s) = size {
        img.fit_to_exact_size(s)
    } else {
        img.shrink_to_fit()
    };
    ui.add(img);
    ui.add_space(4.0);
}

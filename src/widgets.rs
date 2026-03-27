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

// ── Latent Space / Embedding Visualization ────────────────────────

/// A scatter group: a named set of 2D points with a color.
pub struct ScatterGroup<'a> {
    pub label: &'a str,
    pub points: &'a [(f64, f64)],
    pub color: Color32,
}

pub fn scatter_colored(ui: &mut Ui, id: &str, groups: &[ScatterGroup], height: f32) {
    egui_plot::Plot::new(id)
        .height(height)
        .data_aspect(1.0)
        .show(ui, |plot_ui| {
            for group in groups {
                let arr: Vec<[f64; 2]> = group.points.iter().map(|&(x, y)| [x, y]).collect();
                let pts = egui_plot::Points::new(arr)
                    .name(group.label)
                    .color(group.color)
                    .radius(4.0);
                plot_ui.points(pts);
            }
        });
    ui.add_space(4.0);
}

/// Heatmap rendered as a grid of colored rectangles in an egui_plot.
/// `data` is row-major: data[row * cols + col]. Values are mapped to color
/// between `min_val` and `max_val`.
pub fn heatmap(
    ui: &mut Ui,
    _id: &str,
    data: &[f64],
    rows: usize,
    cols: usize,
    min_val: f64,
    max_val: f64,
    height: f32,
) {
    let range = (max_val - min_val).max(1e-10);

    // Draw as colored rectangles using the egui painter directly
    let (rect, _response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), height),
        egui::Sense::hover(),
    );

    if ui.is_rect_visible(rect) {
        let cell_w = rect.width() / cols as f32;
        let cell_h = rect.height() / rows as f32;
        let painter = ui.painter_at(rect);

        for row in 0..rows {
            for col in 0..cols {
                let idx = row * cols + col;
                let val = data.get(idx).copied().unwrap_or(0.0);
                let t = ((val - min_val) / range).clamp(0.0, 1.0) as f32;

                // Viridis-ish colormap: dark purple -> teal -> yellow
                let r = (t * 0.7 + 0.15).min(1.0);
                let g = (t * 0.85).min(1.0);
                let b = (1.0 - t * 0.8).max(0.0);
                let color = Color32::from_rgb(
                    (r * 255.0) as u8,
                    (g * 255.0) as u8,
                    (b * 255.0) as u8,
                );

                let cell_rect = egui::Rect::from_min_size(
                    egui::pos2(
                        rect.min.x + col as f32 * cell_w,
                        rect.min.y + row as f32 * cell_h,
                    ),
                    Vec2::new(cell_w, cell_h),
                );
                painter.rect_filled(cell_rect, 0.0, color);
            }
        }

        // Draw grid lines
        let stroke = egui::Stroke::new(0.5, Color32::from_gray(60));
        for row in 0..=rows {
            let y = rect.min.y + row as f32 * cell_h;
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                stroke,
            );
        }
        for col in 0..=cols {
            let x = rect.min.x + col as f32 * cell_w;
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                stroke,
            );
        }
    }
    ui.add_space(4.0);
}

/// A 2D draggable pad that returns the (x, y) position within [min, max] range.
/// Useful for navigating a 2D latent space or interpolation plane.
pub fn pad_2d(
    ui: &mut Ui,
    state: &mut WidgetState,
    id: &str,
    x_range: (f64, f64),
    y_range: (f64, f64),
    size: f32,
) -> (f64, f64) {
    let key_x = format!("{}_pad_x", id);
    let key_y = format!("{}_pad_y", id);
    let mid_x = (x_range.0 + x_range.1) / 2.0;
    let mid_y = (y_range.0 + y_range.1) / 2.0;
    let mut cur_x = state.get_f64(&key_x).unwrap_or(mid_x);
    let mut cur_y = state.get_f64(&key_y).unwrap_or(mid_y);

    let desired_size = Vec2::splat(size);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    if response.dragged() || response.clicked() {
        if let Some(pos) = response.interact_pointer_pos() {
            let t_x = ((pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0) as f64;
            let t_y = ((pos.y - rect.min.y) / rect.height()).clamp(0.0, 1.0) as f64;
            cur_x = x_range.0 + t_x * (x_range.1 - x_range.0);
            cur_y = y_range.1 - t_y * (y_range.1 - y_range.0); // flip Y: top = max
        }
    }

    state.set_f64(&key_x, cur_x);
    state.set_f64(&key_y, cur_y);

    if ui.is_rect_visible(rect) {
        let painter = ui.painter_at(rect);

        // Background
        painter.rect_filled(rect, 4.0, ui.visuals().extreme_bg_color);
        painter.rect_stroke(rect, 4.0, egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color), egui::StrokeKind::Outside);

        // Crosshair lines
        let t_x = ((cur_x - x_range.0) / (x_range.1 - x_range.0)).clamp(0.0, 1.0) as f32;
        let t_y = (1.0 - (cur_y - y_range.0) / (y_range.1 - y_range.0)).clamp(0.0, 1.0) as f32;
        let px = rect.min.x + t_x * rect.width();
        let py = rect.min.y + t_y * rect.height();
        let cross_color = Color32::from_gray(120);
        painter.line_segment(
            [egui::pos2(px, rect.min.y), egui::pos2(px, rect.max.y)],
            egui::Stroke::new(0.5, cross_color),
        );
        painter.line_segment(
            [egui::pos2(rect.min.x, py), egui::pos2(rect.max.x, py)],
            egui::Stroke::new(0.5, cross_color),
        );

        // Point indicator
        painter.circle_filled(egui::pos2(px, py), 6.0, Color32::from_rgb(75, 150, 255));
        painter.circle_stroke(egui::pos2(px, py), 6.0, egui::Stroke::new(1.5, Color32::WHITE));
    }

    ui.add_space(4.0);
    (cur_x, cur_y)
}

/// Clickable image grid. Returns the index of the clicked cell (if any).
/// `cell_render` draws each cell's content given its index and a `Ui`.
pub fn image_grid<F>(
    ui: &mut Ui,
    id: &str,
    cols: usize,
    count: usize,
    cell_size: Vec2,
    mut cell_render: F,
) -> Option<usize>
where
    F: FnMut(usize, &mut Ui),
{
    let mut clicked = None;
    let rows = (count + cols - 1) / cols;

    egui::Grid::new(id)
        .num_columns(cols)
        .spacing([4.0, 4.0])
        .show(ui, |ui| {
            for row in 0..rows {
                for col in 0..cols {
                    let idx = row * cols + col;
                    if idx >= count {
                        break;
                    }
                    let (rect, response) = ui.allocate_exact_size(cell_size, egui::Sense::click());
                    if response.clicked() {
                        clicked = Some(idx);
                    }

                    // Highlight on hover/click
                    if ui.is_rect_visible(rect) {
                        let painter = ui.painter_at(rect);
                        let bg = if response.hovered() {
                            ui.visuals().widgets.hovered.bg_fill
                        } else {
                            ui.visuals().extreme_bg_color
                        };
                        painter.rect_filled(rect, 2.0, bg);
                    }

                    // Render cell content in a child ui
                    let mut child_ui = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(rect)
                    );
                    cell_render(idx, &mut child_ui);
                }
                ui.end_row();
            }
        });

    ui.add_space(4.0);
    clicked
}

/// Multi-slider panel for controlling N latent dimensions.
/// Returns a Vec of current values.
pub fn latent_sliders(
    ui: &mut Ui,
    state: &mut WidgetState,
    id: &str,
    count: usize,
    range: (f64, f64),
    labels: Option<&[&str]>,
) -> Vec<f64> {
    let mut values = Vec::with_capacity(count);
    let mid = (range.0 + range.1) / 2.0;

    for i in 0..count {
        let key = format!("{}__dim_{}", id, i);
        let mut val = state.get_f64(&key).unwrap_or(mid);
        let label = labels
            .and_then(|l| l.get(i).copied())
            .unwrap_or("");
        let slider_label = if label.is_empty() {
            format!("z[{}]", i)
        } else {
            label.to_string()
        };
        ui.add(egui::Slider::new(&mut val, range.0..=range.1).text(slider_label));
        state.set_f64(&key, val);
        values.push(val);
    }
    values
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

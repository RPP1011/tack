use egui::{self, Ui, Vec2};

use crate::state::WidgetState;
use crate::widgets;

/// The main UI context passed to user code. Wraps egui with a Streamlit-like API.
///
/// All widget methods return their current value directly, and state is persisted
/// automatically between frames. This means you write linear, top-to-bottom code
/// just like Streamlit — no callbacks, no message passing.
pub struct TackUi<'a> {
    pub(crate) ui: &'a mut Ui,
    pub(crate) state: &'a mut WidgetState,
}

impl<'a> TackUi<'a> {
    pub fn new(ui: &'a mut Ui, state: &'a mut WidgetState) -> Self {
        Self { ui, state }
    }

    // ── Text Display ──────────────────────────────────────────

    /// Large page title.
    pub fn title(&mut self, text: &str) {
        widgets::title(self.ui, text);
    }

    /// Section header.
    pub fn header(&mut self, text: &str) {
        widgets::header(self.ui, text);
    }

    /// Subsection header.
    pub fn subheader(&mut self, text: &str) {
        widgets::subheader(self.ui, text);
    }

    /// Plain text.
    pub fn text(&mut self, text: &str) {
        widgets::text(self.ui, text);
    }

    /// Small, muted caption text.
    pub fn caption(&mut self, text: &str) {
        widgets::caption(self.ui, text);
    }

    /// Code block with monospace font and background.
    pub fn code(&mut self, code_text: &str) {
        widgets::code(self.ui, code_text);
    }

    /// Render text (basic rendering, not full markdown yet).
    pub fn markdown(&mut self, text: &str) {
        widgets::markdown(self.ui, text);
    }

    /// Horizontal divider line.
    pub fn divider(&mut self) {
        widgets::divider(self.ui);
    }

    /// Add vertical spacing.
    pub fn space(&mut self, amount: f32) {
        self.ui.add_space(amount);
    }

    // ── Interactive Widgets ───────────────────────────────────

    /// Button. Returns `true` on the frame it was clicked.
    pub fn button(&mut self, label: &str) -> bool {
        widgets::button(self.ui, label)
    }

    /// Checkbox. Returns current checked state.
    pub fn checkbox(&mut self, label: &str, default: bool) -> bool {
        widgets::checkbox(self.ui, self.state, label, default)
    }

    /// Toggle button. Returns current toggled state.
    pub fn toggle(&mut self, label: &str, default: bool) -> bool {
        widgets::toggle(self.ui, self.state, label, default)
    }

    /// Radio button group. Returns index of selected option.
    pub fn radio(&mut self, label: &str, options: &[&str], default: usize) -> usize {
        widgets::radio(self.ui, self.state, label, options, default)
    }

    /// Single-line text input. Returns current text.
    pub fn text_input(&mut self, label: &str, default: &str) -> String {
        widgets::text_input(self.ui, self.state, label, default)
    }

    /// Multi-line text area. Returns current text.
    pub fn text_area(&mut self, label: &str, default: &str) -> String {
        widgets::text_area(self.ui, self.state, label, default)
    }

    /// Numeric input with drag. Returns current value.
    pub fn number_input(&mut self, label: &str, default: f64, min: f64, max: f64, step: f64) -> f64 {
        widgets::number_input(self.ui, self.state, label, default, min, max, step)
    }

    /// Float slider. Returns current value.
    pub fn slider_f64(&mut self, label: &str, min: f64, max: f64, default: f64) -> f64 {
        widgets::slider_f64(self.ui, self.state, label, min, max, default)
    }

    /// Convenience alias: float slider (same as `slider_f64`).
    pub fn slider(&mut self, label: &str, min: f64, max: f64) -> f64 {
        widgets::slider_f64(self.ui, self.state, label, min, max, min)
    }

    /// Integer slider. Returns current value.
    pub fn slider_i32(&mut self, label: &str, min: i32, max: i32, default: i32) -> i32 {
        widgets::slider_i32(self.ui, self.state, label, min, max, default)
    }

    /// Dropdown select box. Returns index of selected option.
    pub fn selectbox(&mut self, label: &str, options: &[&str]) -> usize {
        widgets::selectbox(self.ui, self.state, label, options, 0)
    }

    /// Dropdown select box with a default index.
    pub fn selectbox_with_default(&mut self, label: &str, options: &[&str], default: usize) -> usize {
        widgets::selectbox(self.ui, self.state, label, options, default)
    }

    /// Multi-select checkboxes. Returns indices of selected options.
    pub fn multiselect(&mut self, label: &str, options: &[&str]) -> Vec<usize> {
        widgets::multiselect(self.ui, self.state, label, options)
    }

    /// Progress bar (0.0 to 1.0).
    pub fn progress(&mut self, value: f32) {
        widgets::progress(self.ui, value);
    }

    /// Spinning loading indicator.
    pub fn spinner(&mut self) {
        widgets::spinner(self.ui);
    }

    /// Color picker. Returns `[r, g, b]`.
    pub fn color_picker(&mut self, label: &str) -> [u8; 3] {
        widgets::color_picker(self.ui, self.state, label)
    }

    // ── Data Display ──────────────────────────────────────────

    /// Display a large metric value with optional delta.
    pub fn metric(&mut self, label: &str, value: &str, delta: Option<&str>) {
        widgets::metric(self.ui, label, value, delta);
    }

    /// Display JSON text in a code block.
    pub fn json(&mut self, text: &str) {
        widgets::json(self.ui, text);
    }

    /// Display a table with headers and rows.
    pub fn table(&mut self, headers: &[&str], rows: &[Vec<String>]) {
        widgets::table(self.ui, headers, rows);
    }

    // ── Charts ────────────────────────────────────────────────

    /// Line chart from (x, y) points.
    pub fn line_chart(&mut self, label: &str, points: &[(f64, f64)]) {
        widgets::line_chart(self.ui, label, points);
    }

    /// Bar chart from (x, height) pairs.
    pub fn bar_chart(&mut self, label: &str, bars: &[(f64, f64)]) {
        widgets::bar_chart(self.ui, label, bars);
    }

    /// Scatter plot from (x, y) points.
    pub fn scatter(&mut self, label: &str, points: &[(f64, f64)]) {
        widgets::scatter(self.ui, label, points);
    }

    // ── Latent Space / Embedding Visualization ─────────────────

    /// Colored scatter plot with multiple labeled groups.
    /// Ideal for t-SNE/UMAP projections with cluster labels.
    pub fn scatter_colored(&mut self, id: &str, groups: &[widgets::ScatterGroup], height: f32) {
        widgets::scatter_colored(self.ui, id, groups, height);
    }

    /// Heatmap grid. `data` is row-major (length = rows * cols).
    /// Values are color-mapped between `min_val` and `max_val`.
    /// Use for QD archives, similarity matrices, confusion matrices.
    pub fn heatmap(&mut self, id: &str, data: &[f64], rows: usize, cols: usize, min_val: f64, max_val: f64, height: f32) {
        widgets::heatmap(self.ui, id, data, rows, cols, min_val, max_val, height);
    }

    /// 2D draggable pad. Returns `(x, y)` in the given ranges.
    /// Use for navigating a 2D latent space, interpolation planes, etc.
    pub fn pad_2d(&mut self, id: &str, x_range: (f64, f64), y_range: (f64, f64), size: f32) -> (f64, f64) {
        widgets::pad_2d(self.ui, self.state, id, x_range, y_range, size)
    }

    /// Clickable image grid. Returns index of clicked cell (if any).
    /// `cell_render` draws each cell given its index and an `egui::Ui`.
    pub fn image_grid<F>(&mut self, id: &str, cols: usize, count: usize, cell_size: egui::Vec2, cell_render: F) -> Option<usize>
    where
        F: FnMut(usize, &mut egui::Ui),
    {
        widgets::image_grid(self.ui, id, cols, count, cell_size, cell_render)
    }

    /// Panel of N sliders for controlling latent dimensions.
    /// Returns a Vec of current values. Optionally provide semantic labels.
    pub fn latent_sliders(&mut self, id: &str, count: usize, range: (f64, f64), labels: Option<&[&str]>) -> Vec<f64> {
        widgets::latent_sliders(self.ui, self.state, id, count, range, labels)
    }

    // ── Notifications ─────────────────────────────────────────

    /// Green success banner.
    pub fn success(&mut self, text: &str) {
        widgets::success(self.ui, text);
    }

    /// Blue info banner.
    pub fn info(&mut self, text: &str) {
        widgets::info(self.ui, text);
    }

    /// Yellow warning banner.
    pub fn warning(&mut self, text: &str) {
        widgets::warning(self.ui, text);
    }

    /// Red error banner.
    pub fn error(&mut self, text: &str) {
        widgets::error(self.ui, text);
    }

    // ── Layout ────────────────────────────────────────────────

    /// Multi-column layout. The closure receives a mutable slice of raw egui `Ui`s.
    pub fn columns<F>(&mut self, count: usize, f: F)
    where
        F: FnOnce(&mut [Ui]),
    {
        widgets::columns(self.ui, count, f);
    }

    /// Collapsible/expandable section.
    pub fn expander<F>(&mut self, label: &str, default_open: bool, f: F)
    where
        F: FnOnce(&mut TackUi),
    {
        let state = &mut *self.state;
        egui::CollapsingHeader::new(label)
            .default_open(default_open)
            .show(self.ui, |ui| {
                let mut inner = TackUi::new(ui, state);
                f(&mut inner);
            });
    }

    /// Horizontal layout group.
    pub fn horizontal<F>(&mut self, f: F)
    where
        F: FnOnce(&mut TackUi),
    {
        let state = &mut *self.state;
        self.ui.horizontal(|ui| {
            let mut inner = TackUi::new(ui, state);
            f(&mut inner);
        });
    }

    // ── Images ────────────────────────────────────────────────

    /// Display an image from a URI. Supports `file://`, `https://`, and `bytes://` schemes.
    pub fn image(&mut self, uri: &str) {
        widgets::image(self.ui, uri, None);
    }

    /// Display an image with a specific size.
    pub fn image_with_size(&mut self, uri: &str, width: f32, height: f32) {
        widgets::image(self.ui, uri, Some(Vec2::new(width, height)));
    }

    // ── Raw egui access ───────────────────────────────────────

    /// Escape hatch: access the underlying `egui::Ui` directly.
    pub fn raw_ui(&mut self) -> &mut Ui {
        self.ui
    }
}

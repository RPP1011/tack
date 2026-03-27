//! # Tack
//!
//! A Streamlit-like UI framework for Rust, built on egui.
//!
//! Write native desktop UIs with a simple, linear API — no callbacks, no
//! message-passing, no state wrangling. Just call widget functions top-to-bottom
//! and they return their current values.
//!
//! ## Quick Start
//!
//! ```no_run
//! fn main() {
//!     tack::run("My App", |ui| {
//!         ui.title("Hello, Tack!");
//!         ui.text("Welcome to your first app.");
//!
//!         let name = ui.text_input("Name", "world");
//!         ui.text(&format!("Hello, {}!", name));
//!
//!         if ui.button("Click me") {
//!             ui.success("Button clicked!");
//!         }
//!
//!         let val = ui.slider("Value", 0.0, 100.0);
//!         ui.text(&format!("Slider: {:.1}", val));
//!     });
//! }
//! ```

mod app;
mod state;
mod ui;
mod widgets;

pub use app::TackConfig;
pub use ui::TackUi;
pub use widgets::ScatterGroup;

// Re-export egui so users can access it for advanced widgets.
pub use egui;
pub use egui::Color32;
pub use egui::Vec2;

/// Run a Tack app with the given title and render function.
///
/// This is the simplest way to create a Tack app. For more control
/// (sidebar, window size, theme), use [`TackConfig`].
///
/// ```no_run
/// tack::run("My App", |ui| {
///     ui.title("Hello!");
/// });
/// ```
pub fn run<F>(title: impl Into<String>, f: F)
where
    F: FnMut(&mut TackUi) + 'static,
{
    TackConfig::new(title, f).run().unwrap();
}

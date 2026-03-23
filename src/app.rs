use eframe::{self, egui};

use crate::state::WidgetState;
use crate::ui::TackUi;

/// Internal eframe app that drives the render loop.
pub(crate) struct TackApp<F>
where
    F: FnMut(&mut TackUi),
{
    render_fn: F,
    state: WidgetState,
    sidebar_fn: Option<Box<dyn FnMut(&mut TackUi)>>,
}

impl<F> TackApp<F>
where
    F: FnMut(&mut TackUi),
{
    pub fn new(render_fn: F) -> Self {
        Self {
            render_fn,
            state: WidgetState::default(),
            sidebar_fn: None,
        }
    }

    pub fn with_sidebar<S: FnMut(&mut TackUi) + 'static>(mut self, sidebar: S) -> Self {
        self.sidebar_fn = Some(Box::new(sidebar));
        self
    }
}

impl<F> eframe::App for TackApp<F>
where
    F: FnMut(&mut TackUi),
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Sidebar
        if let Some(sidebar_fn) = &mut self.sidebar_fn {
            egui::SidePanel::left("tack_sidebar")
                .default_width(250.0)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let mut tui = TackUi::new(ui, &mut self.state);
                        sidebar_fn(&mut tui);
                    });
                });
        }

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut tui = TackUi::new(ui, &mut self.state);
                (self.render_fn)(&mut tui);
            });
        });
    }
}

/// Configuration for running a Tack app.
pub struct TackConfig<F>
where
    F: FnMut(&mut TackUi),
{
    title: String,
    render_fn: F,
    sidebar_fn: Option<Box<dyn FnMut(&mut TackUi)>>,
    width: f32,
    height: f32,
    dark_mode: bool,
}

impl<F> TackConfig<F>
where
    F: FnMut(&mut TackUi) + 'static,
{
    /// Create a new app configuration.
    pub fn new(title: impl Into<String>, render_fn: F) -> Self {
        Self {
            title: title.into(),
            render_fn,
            sidebar_fn: None,
            width: 1024.0,
            height: 768.0,
            dark_mode: true,
        }
    }

    /// Add a sidebar.
    pub fn sidebar<S: FnMut(&mut TackUi) + 'static>(mut self, f: S) -> Self {
        self.sidebar_fn = Some(Box::new(f));
        self
    }

    /// Set the initial window size.
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Use light mode instead of dark mode.
    pub fn light_mode(mut self) -> Self {
        self.dark_mode = false;
        self
    }

    /// Run the app (blocks until window is closed).
    pub fn run(self) -> eframe::Result {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([self.width, self.height]),
            ..Default::default()
        };

        let mut app = TackApp::new(self.render_fn);
        if let Some(sidebar) = self.sidebar_fn {
            app = app.with_sidebar(sidebar);
        }
        let dark_mode = self.dark_mode;

        eframe::run_native(
            &self.title,
            options,
            Box::new(move |cc| {
                let style = if dark_mode {
                    egui::Visuals::dark()
                } else {
                    egui::Visuals::light()
                };
                cc.egui_ctx.set_visuals(style);
                Ok(Box::new(app))
            }),
        )
    }
}

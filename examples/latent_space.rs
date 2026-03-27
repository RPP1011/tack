//! Latent Space Explorer demo.
//!
//! Demonstrates the embedding/latent-space widgets:
//! - Colored scatter (t-SNE/UMAP projection)
//! - Heatmap (QD archive / similarity matrix)
//! - 2D navigation pad
//! - Latent dimension sliders
//! - Clickable image grid

use tack::{Color32, ScatterGroup, TackConfig, TackUi, Vec2};

fn main() {
    TackConfig::new("Latent Space Explorer", main_ui)
        .sidebar(sidebar_ui)
        .size(1300.0, 900.0)
        .run()
        .unwrap();
}

fn sidebar_ui(ui: &mut TackUi) {
    ui.title("Controls");
    ui.divider();

    ui.subheader("Latent Vector");
    let z = ui.latent_sliders("z", 8, (-3.0, 3.0), Some(&[
        "style", "complexity", "density", "symmetry",
        "height", "enemies", "rewards", "difficulty",
    ]));
    ui.divider();
    ui.caption(&format!("z = [{:.2}, {:.2}, {:.2}, ...]", z[0], z[1], z[2]));

    ui.divider();

    ui.subheader("2D Navigation");
    let (nx, ny) = ui.pad_2d("nav", (-3.0, 3.0), (-3.0, 3.0), 200.0);
    ui.caption(&format!("Position: ({:.2}, {:.2})", nx, ny));
}

fn main_ui(ui: &mut TackUi) {
    ui.title("Latent Space Explorer");
    ui.text("Interactive exploration of a simulated embedding space.");
    ui.divider();

    // ── Colored Scatter (simulated t-SNE projection) ─────────
    ui.header("Embedding Projection (t-SNE)");

    // Generate fake cluster data
    let cluster_a: Vec<(f64, f64)> = (0..40)
        .map(|i| {
            let angle = i as f64 * 0.15;
            (-2.0 + angle.cos() * 0.8 + (i as f64 * 0.1).sin() * 0.3,
             1.0 + angle.sin() * 0.8 + (i as f64 * 0.2).cos() * 0.3)
        })
        .collect();
    let cluster_b: Vec<(f64, f64)> = (0..35)
        .map(|i| {
            let angle = i as f64 * 0.18;
            (1.5 + angle.cos() * 0.6 + (i as f64 * 0.15).sin() * 0.4,
             -0.5 + angle.sin() * 0.6 + (i as f64 * 0.25).cos() * 0.2)
        })
        .collect();
    let cluster_c: Vec<(f64, f64)> = (0..30)
        .map(|i| {
            let angle = i as f64 * 0.2;
            (-0.5 + angle.cos() * 0.5 + (i as f64 * 0.12).sin() * 0.3,
             -2.0 + angle.sin() * 0.5 + (i as f64 * 0.18).cos() * 0.3)
        })
        .collect();

    let groups = [
        ScatterGroup { label: "Platformer", points: &cluster_a, color: Color32::from_rgb(255, 100, 100) },
        ScatterGroup { label: "Puzzle",     points: &cluster_b, color: Color32::from_rgb(100, 200, 255) },
        ScatterGroup { label: "Combat",     points: &cluster_c, color: Color32::from_rgb(100, 255, 150) },
    ];
    ui.scatter_colored("tsne", &groups, 300.0);

    ui.divider();

    // ── QD Archive Heatmap ───────────────────────────────────
    ui.header("Quality-Diversity Archive");
    ui.text("Fitness heatmap over (enemy count × platform density) behavior space");

    let archive_rows = 12;
    let archive_cols = 16;
    let archive_data: Vec<f64> = (0..archive_rows * archive_cols)
        .map(|i| {
            let r = (i / archive_cols) as f64;
            let c = (i % archive_cols) as f64;
            // Simulate fitness landscape with a peak
            let dx = c - 8.0;
            let dy = r - 6.0;
            let fitness = 1.0 - (dx * dx + dy * dy) / 100.0;
            // Some cells are empty (no solution found)
            if ((i * 7 + 3) % 11) < 3 { 0.0 } else { fitness.max(0.0) }
        })
        .collect();

    ui.heatmap("qd_archive", &archive_data, archive_rows, archive_cols, 0.0, 1.0, 200.0);

    ui.divider();

    // ── Image Grid (generated content browser) ───────────────
    ui.header("Generated Content Browser");
    ui.text("Click a cell to select it for editing.");

    let selected = ui.image_grid("content_grid", 5, 15, Vec2::new(80.0, 60.0), |idx, cell_ui| {
        // In a real app, each cell would show a decoded latent vector as an image.
        // Here we simulate with colored rectangles + labels.
        let hue = (idx as f32 * 25.0) % 360.0;
        let color = egui::ecolor::Hsva::new(hue / 360.0, 0.6, 0.8, 1.0);
        let rect = cell_ui.max_rect();
        cell_ui.painter().rect_filled(rect.shrink(2.0), 4.0, color);
        cell_ui.put(rect, egui::Label::new(
            egui::RichText::new(format!("#{}", idx)).small().color(Color32::WHITE),
        ));
    });

    if let Some(idx) = selected {
        ui.info(&format!("Selected content #{}", idx));
    }

    ui.divider();

    // ── Similarity Matrix ────────────────────────────────────
    ui.header("Embedding Similarity Matrix");

    let sim_size = 8;
    let sim_data: Vec<f64> = (0..sim_size * sim_size)
        .map(|i| {
            let r = i / sim_size;
            let c = i % sim_size;
            if r == c {
                1.0
            } else {
                let diff = (r as f64 - c as f64).abs();
                (1.0 - diff / sim_size as f64).max(0.0).powi(2)
            }
        })
        .collect();

    ui.heatmap("similarity", &sim_data, sim_size, sim_size, 0.0, 1.0, 200.0);
}

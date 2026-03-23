use tack::{TackConfig, TackUi};

fn main() {
    TackConfig::new("Tack Kitchen Sink", main_ui)
        .sidebar(sidebar_ui)
        .size(1200.0, 800.0)
        .run()
        .unwrap();
}

fn sidebar_ui(ui: &mut TackUi) {
    ui.title("Sidebar");
    ui.divider();

    let _name = ui.text_input("Your Name", "User");
    let _theme = ui.selectbox("Theme", &["Dark", "Light", "Auto"]);
    let _verbose = ui.checkbox("Verbose mode", false);

    ui.divider();
    ui.caption("Built with Tack");
}

fn main_ui(ui: &mut TackUi) {
    ui.title("Tack Kitchen Sink");
    ui.text("A demo of every widget in the Tack library.");
    ui.divider();

    // ── Text ─────────────────────────────────────────────
    ui.header("Text Display");
    ui.subheader("Various text styles");
    ui.text("Plain body text.");
    ui.caption("A small caption.");
    ui.code("fn main() {\n    println!(\"Hello, Tack!\");\n}");

    ui.divider();

    // ── Notifications ────────────────────────────────────
    ui.header("Notifications");
    ui.success("Operation completed successfully.");
    ui.info("Here's some useful information.");
    ui.warning("Be careful with this action.");
    ui.error("Something went wrong!");

    ui.divider();

    // ── Input Widgets ────────────────────────────────────
    ui.header("Input Widgets");

    let name = ui.text_input("Name", "World");
    ui.text(&format!("Hello, {}!", name));

    let bio = ui.text_area("Bio", "Tell us about yourself...");
    ui.caption(&format!("{} chars", bio.len()));

    let age = ui.slider_i32("Age", 0, 120, 25);
    ui.text(&format!("Age: {}", age));

    let temp = ui.slider_f64("Temperature", -20.0, 50.0, 22.0);
    ui.text(&format!("Temperature: {:.1}°C", temp));

    let count = ui.number_input("Count", 10.0, 0.0, 100.0, 1.0);
    ui.text(&format!("Count: {:.0}", count));

    ui.divider();

    // ── Selection Widgets ────────────────────────────────
    ui.header("Selection Widgets");

    let fruit = ui.selectbox("Favorite Fruit", &["Apple", "Banana", "Cherry", "Date"]);
    let fruits = ["Apple", "Banana", "Cherry", "Date"];
    ui.text(&format!("You picked: {}", fruits[fruit]));

    let _agree = ui.checkbox("I agree to the terms", false);
    let _dark = ui.toggle("Dark mode", true);

    let choice = ui.radio("Pick one", &["Small", "Medium", "Large"], 1);
    let sizes = ["Small", "Medium", "Large"];
    ui.text(&format!("Size: {}", sizes[choice]));

    let selected = ui.multiselect("Toppings", &["Cheese", "Pepperoni", "Mushrooms", "Olives"]);
    let toppings = ["Cheese", "Pepperoni", "Mushrooms", "Olives"];
    let chosen: Vec<&str> = selected.iter().map(|&i| toppings[i]).collect();
    ui.text(&format!("Toppings: {}", chosen.join(", ")));

    let color = ui.color_picker("Pick a color");
    ui.text(&format!("Color: rgb({}, {}, {})", color[0], color[1], color[2]));

    ui.divider();

    // ── Data Display ─────────────────────────────────────
    ui.header("Data Display");

    ui.horizontal(|ui| {
        ui.metric("Users", "1,234", Some("+12%"));
        ui.metric("Revenue", "$56.7k", Some("+8.3%"));
        ui.metric("Errors", "3", Some("-45%"));
    });

    ui.json("{\n  \"name\": \"tack\",\n  \"version\": \"0.1.0\",\n  \"awesome\": true\n}");

    ui.table(
        &["Name", "Language", "Stars"],
        &[
            vec!["tack".into(), "Rust".into(), "42".into()],
            vec!["streamlit".into(), "Python".into(), "30k".into()],
            vec!["egui".into(), "Rust".into(), "20k".into()],
        ],
    );

    ui.divider();

    // ── Charts ───────────────────────────────────────────
    ui.header("Charts");

    let sin_points: Vec<(f64, f64)> = (0..200)
        .map(|i| {
            let x = i as f64 * 0.05;
            (x, x.sin())
        })
        .collect();
    ui.line_chart("sin(x)", &sin_points);

    let bar_data: Vec<(f64, f64)> = (0..10)
        .map(|i| (i as f64, ((i * 7 + 3) % 13) as f64))
        .collect();
    ui.bar_chart("Random bars", &bar_data);

    let scatter_data: Vec<(f64, f64)> = (0..50)
        .map(|i| {
            let x = (i as f64) * 0.2;
            let y = x.sin() * 3.0 + (i as f64 * 1.7).cos();
            (x, y)
        })
        .collect();
    ui.scatter("Scatter", &scatter_data);

    ui.divider();

    // ── Layout ───────────────────────────────────────────
    ui.header("Layout");

    ui.expander("Click to expand", false, |ui| {
        ui.text("Hidden content revealed!");
        ui.text("You can put anything in here.");
    });

    // ── Progress ─────────────────────────────────────────
    ui.header("Progress");
    let pval = ui.slider_f64("Progress value", 0.0, 1.0, 0.65);
    ui.progress(pval as f32);
}

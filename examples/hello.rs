fn main() {
    tack::run("Hello Tack", |ui| {
        ui.title("Hello, Tack!");
        ui.text("This is the simplest possible Tack app.");
        ui.divider();

        let name = ui.text_input("Your name", "World");
        ui.text(&format!("Hello, {}!", name));

        if ui.button("Click me") {
            ui.success("You clicked the button!");
        }

        let val = ui.slider("Pick a number", 0.0, 100.0);
        ui.text(&format!("Value: {:.1}", val));
    });
}

use tack::{TackConfig, TackUi};

fn main() {
    TackConfig::new("Dashboard", |ui: &mut TackUi| {
        ui.title("Sales Dashboard");
        ui.divider();

        // Metrics row
        ui.horizontal(|ui| {
            ui.metric("Total Sales", "$124,500", Some("+15.3%"));
            ui.metric("Orders", "1,847", Some("+8.1%"));
            ui.metric("Customers", "562", Some("+22.4%"));
            ui.metric("Avg Order", "$67.40", Some("-2.1%"));
        });

        ui.divider();

        // Revenue chart
        ui.header("Monthly Revenue");
        let revenue: Vec<(f64, f64)> = vec![
            (1.0, 8200.0), (2.0, 9100.0), (3.0, 8800.0),
            (4.0, 10200.0), (5.0, 11500.0), (6.0, 10800.0),
            (7.0, 12400.0), (8.0, 13100.0), (9.0, 11900.0),
            (10.0, 14200.0), (11.0, 15100.0), (12.0, 16800.0),
        ];
        ui.line_chart("Revenue ($)", &revenue);

        // Sales by category
        ui.header("Sales by Category");
        let categories: Vec<(f64, f64)> = vec![
            (1.0, 4200.0), (2.0, 3800.0), (3.0, 2900.0),
            (4.0, 2100.0), (5.0, 1500.0),
        ];
        ui.bar_chart("Sales ($)", &categories);

        // Recent orders table
        ui.header("Recent Orders");
        ui.table(
            &["Order ID", "Customer", "Amount", "Status"],
            &[
                vec!["#1847".into(), "Alice Johnson".into(), "$234.00".into(), "Shipped".into()],
                vec!["#1846".into(), "Bob Smith".into(), "$89.50".into(), "Processing".into()],
                vec!["#1845".into(), "Carol White".into(), "$156.75".into(), "Delivered".into()],
                vec!["#1844".into(), "Dave Brown".into(), "$412.00".into(), "Shipped".into()],
                vec!["#1843".into(), "Eve Davis".into(), "$67.25".into(), "Delivered".into()],
            ],
        );
    })
    .size(1100.0, 850.0)
    .run()
    .unwrap();
}

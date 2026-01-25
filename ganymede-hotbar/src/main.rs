//! Hotbar GTK (barre du haut, MVP)
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, HeaderBar, Label, Button};

fn main() {
    let app = Application::builder()
        .application_id("dev.ganymede.hotbar")
        .build();
    app.connect_activate(|app| {
        let bar = HeaderBar::builder()
            .title(Some("Ganymede Hotbar"))
            .show_close_button(false)
            .build();
        let time_label = Label::new(Some("12:34"));
        let dashboard_btn = Button::with_label("Dashboard");
        bar.pack_start(&time_label);
        bar.pack_end(&dashboard_btn);
        let win = ApplicationWindow::builder()
            .application(app)
            .title("Ganymede Hotbar")
            .decorated(false)
            .opacity(0.5)
            .child(&bar)
            .build();
        win.show();
    });
    app.run();
}

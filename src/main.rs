//
//  Requirements for building: https://gtk-rs.org/docs-src/requirements.html
//  Dependencies needed in Cargo.toml: gtk = "0.5.0"
//

extern crate gtk;

use gtk::prelude::*;
use gtk::{Label, Window, WindowType};

fn main() {
    if gtk::init().is_err() {
        println!("GTK init failed.");
        return;
    }

    let hw_window = Window::new(WindowType::Toplevel);
    hw_window.set_title("helloRube");
    hw_window.set_default_size(800, 400);

    let hw_label = Label::new("Hello World!");

    hw_window.add(&hw_label);
    hw_window.show_all();

    hw_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

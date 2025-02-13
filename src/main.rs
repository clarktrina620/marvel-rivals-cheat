use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};
use std::process::Command;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Marvel Rivals Cheat");
    window.set_default_size(300, 200);

    let label = Label::new(Some("Welcome to Marvel Rivals Cheat"));
    let button = Button::new_with_label("Activate Cheat");

    button.connect_clicked(move |_| {
        activate_cheat();
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.pack_start(&label, true, true, 0);
    vbox.pack_start(&button, true, true, 0);
    window.add(&vbox);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}

fn activate_cheat() {
    Command::new("cmd")
        .args(&["/C", "echo Cheat Activated"])
        .output()
        .expect("Failed to execute command");
}
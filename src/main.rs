/*
 * gtk4
 *
 * Memory Management
 *
 * Got my Working Example from here:
 * https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_memory_management.html
 */

use std::rc::Rc;
use std::cell::Cell;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, Button, ApplicationWindow, Orientation};
use glib;
use glib_macros::clone;

struct Con {
    num: i16,
    name: char,
}

impl Con {
    fn set(&mut self, value: i16) {
        self.num = value;
    }
    fn get(&self) -> i16 {
        return self.num;
    }
}

fn main() {
    // Create a new application
    let app = Application::builder().application_id("str").build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_show = Button::builder()
        .label("None")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // A mutable integer
    //let mut number = 0;
    let mut con = Con {num: 1, name: 'c'};
    let number = Rc::new(Cell::new(0));

    // Connect callbacks
    // When a button is clicked, `number` should be changed
    button_increase.connect_clicked(clone!(@weak number, @weak button_increase, @weak button_show =>
        move |_| {
                number.set(number.get() + 1);
                button_show.set_label(&number.get().to_string());

        }));

    button_decrease.connect_clicked(clone!(@weak button_decrease, @weak button_show =>
        move |_| {
                number.set(number.get() - 1);
                button_show.set_label(&number.get().to_string());
        }));

    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    gtk_box.append(&button_show);
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}

/* User Interface */

use std::rc::Rc;
use std::cell::{Cell, RefCell};

use gtk::{Application, ApplicationWindow, Box, Orientation, GestureClick,
Popover, Grid, HeaderBar, AboutDialog, MenuButton, Label};
use gtk::prelude::*;

use glib_macros::clone;

use crate::operator_symbols::*;
use crate::widgets;
use crate::widgets::{NumberButtons, OperatorButtons, SpecialButtons};
use crate::calculator;
use crate::calculator::{Values, Operators};
use crate::grid;

// Get package version from Cargo
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn build_ui(application: &Application) {
    /* build ui of the application */
    // Creates file of the image
    let logo_file = gio::File::for_path("src/resources/example.png");
    
    // Creates picture
    let app_logo = gtk::Picture::for_file(&logo_file);

    // Header bar
    let header_bar = HeaderBar::new();

    // Menu Button
    let menu_button = MenuButton::new();
    menu_button.set_icon_name("view-list"); // set menu button icon

    // Vertical Box
    let vbox = Box::new(Orientation::Vertical, 0);

    // Popover for labels
    let popover = Popover::new();

    // About label
    let about_label = Label::new(Some("About"));

    // About gesture
    let about_gesture = GestureClick::new();
    
    // Grid
    let grid = Grid::new();
   
    // entry
    let entry = widgets::create_entry();

    // number buttons
    let number_buttons = NumberButtons::new();

    // operator buttons
    let operator_buttons = OperatorButtons::new();

    // special buttons
    let special_buttons = SpecialButtons::new();

    // add css class for the special button
    special_buttons.clear.add_css_class("clear");
    special_buttons.equals.add_css_class("equals");

    /* Mutable values */
    // values
    let vals = Rc::new(RefCell::new(Values::new()));
    // operators
    let ops = Rc::new(RefCell::new(Operators::new()));

    let num_counter: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let divide_zero: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let initiate_equals: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    /* Connect callbacks */
    about_gesture.connect_pressed(move |about_gesture, _, _, _| {
        about_gesture.set_state(gtk::EventSequenceState::Claimed);

        // create about dialog here
        // About Dialog 
        let about_dialog = AboutDialog::builder()
            .logo(&app_logo.paintable().unwrap())
            .version(VERSION)
            .comments("GTK4 Calculator App written in Rust")
            .copyright("© 2022 Salmaan Saeed")
            .authors(vec![String::from("Salmaan Saeed")])
            .license("The 3-Clause BSD License")
            .build();

        about_dialog.show();
    });

    number_buttons.num0.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(num_counter.get(), &vals, 0.0);
            entry.insert_text("0", &mut -1);
        }));
    number_buttons.num1.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(num_counter.get(), &vals, 1.0);
            entry.insert_text("1", &mut -1);
        }));
    number_buttons.num2.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(num_counter.get(), &vals, 2.0);
            entry.insert_text("2", &mut -1);
        }));
    number_buttons.num3.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(num_counter.get(), &vals, 3.0);
            entry.insert_text("3", &mut -1);
        }));
    number_buttons.num4.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(num_counter.get(), &vals, 4.0);
            entry.insert_text("4", &mut -1);
        }));
    number_buttons.num5.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(num_counter.get(), &vals, 5.0);
            entry.insert_text("5", &mut -1);
        }));
    number_buttons.num6.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(num_counter.get(), &vals, 6.0);
            entry.insert_text("6", &mut -1);
        }));
    number_buttons.num7.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(num_counter.get(), &vals, 7.0);
            entry.insert_text("7", &mut -1);
        }));
    number_buttons.num8.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(num_counter.get(), &vals, 8.0);
            entry.insert_text("8", &mut -1);
        }));
    number_buttons.num9.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong initiate_equals, @strong entry =>
        move |_| {
            calculator::clear_entry_before_calculation(&initiate_equals, &entry);
            calculator::set_value(num_counter.get(), &vals, 9.0);
            entry.insert_text("9", &mut -1);
        }));
    
    operator_buttons.plus.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong entry =>
        move |_| {
            if entry.text().chars().last().unwrap() != '+' {
                // Increase the counter
                num_counter.set(num_counter.get() + 1);

                // Do the operation
                calculator::operation(ADD, &num_counter, &ops, &vals, &divide_zero);

                // Insert the addition symbol to the entry
                entry.insert_text("+", &mut -1);
            }
        }));

    operator_buttons.minus.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong entry =>
        move |_| {
            if entry.text().chars().last().unwrap() != '-' {
                // Increase the counter
                num_counter.set(num_counter.get() + 1);

                // Do the operation
                calculator::operation(SUBTRACT, &num_counter, &ops, &vals, &divide_zero);

                // Insert the subtraction symbol to the entry
                entry.insert_text("-", &mut -1);
            }
        }));

    operator_buttons.multiply.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong entry =>
        move |_| {
            if entry.text().chars().last().unwrap() != '\u{00D7}' {
                // Increase the counter
                num_counter.set(num_counter.get() + 1);

                // Do the operation
                calculator::operation(MULTIPLY, &num_counter, &ops, &vals, &divide_zero);
    
                // Insert the multiplication symbol to the entry
                entry.insert_text("\u{00D7}", &mut -1);
            }
        }));

    operator_buttons.divide.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong entry =>
        move |_| {
            if entry.text().chars().last().unwrap() != '\u{00F7}' {
                // Increase the counter
                num_counter.set(num_counter.get() + 1);

                // Do the operation
                calculator::operation(DIVIDE, &num_counter, &ops, &vals, &divide_zero);
        
                // Insert the division symbol to the entry
                entry.insert_text("\u{00F7}", &mut -1);
            }
        }));
    
    special_buttons.equals.connect_clicked(clone!(@strong vals, @strong num_counter, @strong ops, 
        @strong divide_zero, @strong entry =>
        move |_| {
            let last_entry_char = entry.text().chars().last();

            match last_entry_char {
                Some(_) => {
                    if last_entry_char.unwrap().is_numeric() {
                        // Increase the counter
                        num_counter.set(num_counter.get() + 1);
                        
                        // Equality
                        calculator::equality(&num_counter, &ops, &vals, &divide_zero, &entry, &initiate_equals);
                    }
                },
                None => {}
            }
        }));

    special_buttons.clear.connect_clicked(clone!(@strong entry =>
        move |_| {
            // reset variables
            calculator::reset_variables(&vals, &ops, &num_counter, &divide_zero);

            // Clear entry text
            entry.set_text("");
        }));

    // Add label to box
    about_label.add_controller(&about_gesture);

    // Add about label to vertical box
    vbox.append(&about_label);
    
    // Set vertical box as the child of popover
    popover.set_child(Some(&vbox));

    // Set popover for menu button
    menu_button.set_popover(Some(&popover));

    // Add about button to the header bar
    header_bar.pack_end(&menu_button);

    /* Attach widgets to the Grid */
    grid::set_grid(&grid, &entry, &special_buttons, &operator_buttons, &number_buttons);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Calculator")
        .default_width(250)
        .default_height(70)
        .build();

    // Set the window title bar
    window.set_titlebar(Some(&header_bar));

    // set grid as a child of window
    window.set_child(Some(&grid));

    // Present the window
    window.present();
}


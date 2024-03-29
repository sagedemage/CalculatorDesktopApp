/* Calculator Desktop App */

use gtk::Application;
use gtk::prelude::*;

mod operator_symbols;
mod widgets;
mod grid;
mod calculator;
mod ui;
mod css;
mod editable_entry_text;

const APP_ID: &str = "org.gtk_rs.GObjectSubclassing1";
const CSS_FILE_PATH: &str = "src/resources/css/styles.css";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    
    /* Connect to signals */
    // Load css file
    app.connect_startup(|_| css::load_css_file(CSS_FILE_PATH));
    
    // Activate UI
    app.connect_activate(ui::build_ui);

    // Run the application
    app.run();
}


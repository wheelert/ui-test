use std::{path::Path};
use gtk::glib::signal::Inhibit;
use gtk4 as gtk;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Builder, Button, MessageDialog, ResponseType};


fn main() {
    gtk::init() // This function will initialize the gtk
    .expect("Could not init the GTK"); 

    let application = gtk::Application::builder()
        .application_id("com.wheelerwire.ui-test")
        .build();

    application.connect_activate(build_ui);
    application.run(); 
}

fn build_ui(application: &gtk::Application) {
    let ui_src = include_str!("ui2.ui");
    let builder = Builder::from_string(ui_src);

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let bigbutton: Button = builder.object("button").expect("Couldn't get button");
    let dialog: MessageDialog = builder
        .object("messagedialog")
        .expect("Couldn't get messagedialog");

    dialog.connect_response(move |d: &MessageDialog, _: ResponseType| {
        d.hide();
    });

    bigbutton.connect_clicked(move |_| {
        dialog.show();
    });

    window.show();

}

   

    




#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use crate::conversion;
use crate::interface;
use clipboard::{ClipboardContext, ClipboardProvider};
#[cfg(not(target_os = "android"))] // graphics disabled on android
use gtk::prelude::*;
use gtk4 as gtk;

#[cfg(not(target_os = "android"))]
pub fn run(ui: interface::Ui, digitsarg: interface::Digits) {
    let app = gtk::Application::builder()
        .application_id("com.tdb.numstring")
        .build();
    app.connect_activate(move |app| {
        // widgets
        let convert_button = gtk::Button::with_label(&ui.convert_btn);
        let clear_button = gtk::Button::with_label(&ui.clear_btn);
        let copy_button = gtk::Button::with_label(&ui.copy_btn);
        let inputbox = gtk::Entry::new();
        let outputbox = gtk::TextView::new();

        // scrolled view for the output
        let scrolled_window = gtk::ScrolledWindow::new();
        scrolled_window.set_child(Some(&outputbox));
        scrolled_window.set_kinetic_scrolling(true);
        scrolled_window.set_height_request(500);

        // window structure
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .default_width(400)
            .default_height(600)
            .resizable(false)
            .title("Num String")
            .build();

        // box to store all the widgets
        let gtk_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        gtk_box.append(&inputbox);
        gtk_box.append(&convert_button);
        gtk_box.append(&copy_button);
        gtk_box.append(&clear_button);
        gtk_box.append(&scrolled_window);

        window.set_child(Some(&gtk_box));
        window.show();

        // workaround to have shared memory between the two scopes
        let inputbox_buf = inputbox.buffer();
        let outputbox_buf = outputbox.buffer();
        let digits = digitsarg.clone();

        // convert if enter is pressed
        inputbox.connect_activate(move |_| {
            let mut input = inputbox_buf.text().to_string();
            outputbox_buf.set_text(
                &conversion::convert(&digits, conversion::separate_nums(&mut input)).to_string(),
            );
        });

        // disable output editing
        outputbox.set_editable(false);
        outputbox.set_wrap_mode(gtk::WrapMode::Word);
        // outputbox.set_can_focus(false);
        outputbox.set_cursor_visible(false);
        // outputbox.connect("button-press-event", true, |_| {
        // 	Some(0.to_value())
        // });

        // continuing workaround
        let inputbox_buf = inputbox.buffer();
        let outputbox_buf = outputbox.buffer();
        let digits = digitsarg.clone();

        // convert on click
        convert_button.connect_clicked(move |_| {
            let mut input = inputbox_buf.text().to_string();
            outputbox_buf.set_text(
                &conversion::convert(&digits, conversion::separate_nums(&mut input)).to_string(),
            );
        });

        // continuing workaround
        let inputbox_buf = inputbox.buffer();
        let outputbox_buf = outputbox.buffer();

        // clear on click
        clear_button.connect_clicked(move |_| {
            inputbox_buf.set_text("");
            outputbox_buf.set_text("");
        });

        // copy on click
        let outputbox_buf = outputbox.buffer();
        copy_button.connect_clicked(move |_| {
            let mut clipb: ClipboardContext = ClipboardProvider::new().unwrap();
            let text_start = outputbox_buf.start_iter();
            let text_end = outputbox_buf.end_iter();
            clipb
                .set_contents(outputbox_buf.text(&text_start, &text_end, true).to_string())
                .unwrap();
        });
    });
    app.run_with_args(&[""]);
}

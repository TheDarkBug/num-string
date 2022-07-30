#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)] // hide console window on Windows in release
use tauri::State;
use crate::conversion;
use crate::interface;
// use clipboard::{ClipboardContext, ClipboardProvider};

#[tauri::command]
fn get_ui_btns(ui_vec: State<Vec<String>>) -> Vec<String> {
    return (*ui_vec).clone();
}

#[tauri::command]
fn convert(input: &str, digits: State<interface::Digits>) -> String {
    conversion::convert(&digits.clone(), conversion::separate_nums(String::from(input).as_str())).to_string()
}

#[cfg(not(target_os = "android"))]
pub fn run(ui: interface::Ui, digits: interface::Digits) {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .manage(vec![ui.insert_num, ui.convert_btn, ui.clear_btn, ui.copy_btn])
        .manage(digits)
        // .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![get_ui_btns, convert])
        .run(context)
        .expect("error while running tauri application");
   
        // convert if enter is pressed
 
    //     // convert on click
    //     convert_button.connect_clicked(move |_| {
    //         let mut input = inputbox_buf.text().to_string();
    //         outputbox_buf.set_text(
    //             &conversion::convert(&digits, conversion::separate_nums(&mut input)).to_string(),
    //         );
    //     });

    //     // copy on click
    //     let outputbox_buf = outputbox.buffer();
    //     copy_button.connect_clicked(move |_| {
    //         let mut clipb: ClipboardContext = ClipboardProvider::new().unwrap();
    //         let text_start = outputbox_buf.start_iter();
    //         let text_end = outputbox_buf.end_iter();
    //         clipb
    //             .set_contents(outputbox_buf.text(&text_start, &text_end, true).to_string())
    //             .unwrap();
    //     });
    // });
}

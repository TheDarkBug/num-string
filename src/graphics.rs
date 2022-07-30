#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)] // hide console window on Windows in release
use crate::conversion;
use crate::interface;
use tauri::State;
// use clipboard::{ClipboardContext, ClipboardProvider};

#[tauri::command]
fn get_ui_btns(ui_vec: State<Vec<String>>) -> Vec<String> {
    return (*ui_vec).clone();
}

#[tauri::command]
fn convert(input: &str, digits: State<interface::Digits>) -> String {
    conversion::convert(
        &digits.clone(),
        conversion::separate_nums(String::from(input).as_str()),
    )
    .to_string()
}

#[cfg(not(target_os = "android"))]
pub fn run(ui: interface::Ui, digits: interface::Digits) {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .manage(vec![
            ui.insert_num,
            ui.convert_btn,
            ui.clear_btn,
            ui.copy_btn,
        ])
        .manage(digits)
        // .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![get_ui_btns, convert])
        .run(context)
        .expect("error while running tauri application");
}

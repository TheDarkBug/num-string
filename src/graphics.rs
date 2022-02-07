#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use crate::conversion;
use crate::interface;
extern crate clipboard;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
#[cfg(not(target_os = "android"))] // graphics disabled on android
use eframe::{egui, epi};

struct Gui {
	ui: interface::Ui,
	digits: interface::Digits,
	input: String,
	output: String,
	// clipboard handle to copy the output
	clipb: ClipboardContext,
}

impl Gui {
	fn new(uip: interface::Ui, digitsp: interface::Digits) -> Gui {
		// initialize the gui struct
		Gui {
			ui: uip,
			digits: digitsp,
			input: String::new(),
			output: String::new(),
			clipb: ClipboardProvider::new().unwrap(),
		}
	}
}

#[cfg(not(target_os = "android"))]
impl epi::App for Gui {
	fn name(&self) -> &str {
		"Num String"
	}

	fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.text_edit_multiline(&mut self.input);
			// if input exists
			let input_len = self.input.len();
			if input_len > 0 {
				// get the last char
				let last_chr = self.input.chars().nth(input_len - 1);
				if last_chr != None {
					// if it is a \n (if return was pressed)
					if last_chr.unwrap() == '\n' {
						// convert the input
						self.output = conversion::convert(
							&self.digits,
							conversion::separate_nums(&mut self.input),
						);
						// remove the \n
						self.input.remove(input_len - 1);
					}
				}
			}
			// convert button
			if ui.button(&self.ui.convert_btn).clicked() {
				self.output =
					conversion::convert(&self.digits, conversion::separate_nums(&mut self.input));
			}
			// clear button
			if ui.button(&self.ui.clear_btn).clicked() {
				self.input = String::new();
				self.output = String::new();
			}
			// scrollable output (if the output is too big to be shown in one window)
			egui::ScrollArea::vertical().show(ui, |ui| {
				// copy the output to clipboard if the output widget is clicked
				if ui.selectable_label(false, &self.output).clicked() {
					self.clipb.set_contents(self.output.clone()).unwrap();
				}
			});
		});
	}
}

#[cfg(not(target_os = "android"))]
pub fn run(ui: interface::Ui, digits: interface::Digits) {
	eframe::run_native(
		Box::new(Gui::new(ui, digits)),
		eframe::NativeOptions {
			// window size
			initial_window_size: Some(egui::Vec2::new(400., 600.)),
			resizable: false,
			// icon, for windows
			icon_data: Some(eframe::epi::IconData {
				rgba: vec![0, 1, 2, 3],
				width: 128,
				height: 128,
			}),
			..eframe::NativeOptions::default()
		},
	);
}

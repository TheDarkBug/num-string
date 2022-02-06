#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

extern crate clipboard;
use crate::conversion;
use crate::interface;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use eframe::{egui, epi};

struct Gui {
	ui: interface::Ui,
	digits: interface::Digits,
	input: String,
	output: String,
	checked: bool,
	clipb: ClipboardContext,
}

impl Gui {
	fn new(uip: interface::Ui, digitsp: interface::Digits) -> Gui {
		Gui {
			ui: uip,
			digits: digitsp,
			input: String::new(),
			output: String::new(),
			checked: false,
			clipb: ClipboardProvider::new().unwrap(),
		}
	}
}

impl epi::App for Gui {
	fn name(&self) -> &str {
		"Num String"
	}

	fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.text_edit_singleline(&mut self.input);
			if ui.button("Convert").clicked() {
				self.output = conversion::convert(
					&self.digits,
					&self.ui,
					conversion::separate_nums(&self.input, &self.ui),
				);
			}
			if ui.button("Clear").clicked() {
				self.input = String::new();
			}
			egui::ScrollArea::vertical().show(ui, |ui| {
				if ui.selectable_label(self.checked, &self.output).clicked() {
					self.clipb.set_contents(self.output.clone()).unwrap();
				}
			});
		});
	}
}

pub fn run(ui: interface::Ui, digits: interface::Digits) {
	eframe::run_native(
		Box::new(Gui::new(ui, digits)),
		eframe::NativeOptions::default(),
	);
}

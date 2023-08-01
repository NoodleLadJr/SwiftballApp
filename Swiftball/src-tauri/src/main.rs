//use test_py::types::IntoPyDict

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PyModule;
use pyo3::Python;
use std::env;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn test_py() {
    println!("Hello Rust World!");
    dbg!(env::current_dir());
    let code = std::fs::read_to_string("../../testmod.py").unwrap();
    Python::with_gil(|py| {
        let testmod = PyModule::from_code(py, &code, "testmod.py", "testmod")
            .expect("Error importing testmod");
        testmod.call_method0("print_info");
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, test_py])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::env;
use std::time::Instant;

mod prime_finder;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {} hihihi", name)
}

#[tauri::command]
fn prime(number: &str) -> Result<(String, String, String), String> {
    if number.is_empty() {
        return Err("Input cannot be empty".to_string());
    }
    
    let number: u32 = match number.parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid input".to_string()),
    };
    
    let start = Instant::now();
    let result = prime_finder::eratos(number);
    let duration = start.elapsed();
    
    let total_primes_string = format!("Total primes: {}", result.len());
    let duration_string = format!("Time elapsed: {:?}", duration);
    let largest_prime_string = format!("Largest prime: {}", result.last().unwrap());
    Ok((total_primes_string, duration_string, largest_prime_string))
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![prime, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

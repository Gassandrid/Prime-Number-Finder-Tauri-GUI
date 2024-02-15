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
fn prime(number: u32) -> (String, String, String) {
    let start_time = Instant::now();
    
    let primes = prime_finder::eratos(number);
    let end_time = start_time.elapsed();
    let time_taken = end_time.as_millis();

    let time_taken_string = format!("Time taken: {:?}", time_taken);
    let num_primes_string = format!("Number of primes: {}", primes.len());
    let largest_prime_string = format!("Largest prime: {}", primes.last().unwrap());

    (time_taken_string, num_primes_string, largest_prime_string)
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![prime, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

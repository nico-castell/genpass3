// MIT License - Copyright (c) 2022 Nicolás Castellán
// SPDX License identifier: MIT
// THE SOFTWARE IS PROVIDED "AS IS"
// Read the included LICENSE file for more information

use std::{env, process};

use genpass3::Config;

fn main() {
    // Handle `--help` and `-h`
    if env::args().any(|arg| arg == "--help" || arg == "-h") {
        Config::print_config();
        process::exit(0);
    }

    // Build config
    let config = match Config::build(env::args()) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("\x1B[01;31mConfiguration error\x1B[00m: {}", error);
            Config::print_config();
            process::exit(1);
        }
    };

    // Run the program
    match genpass3::run(&config) {
        Ok(password) => println!("{}", password),
        Err(error) => {
            eprintln!("\x1B[01;31mApplication error\x1B[00m: {}", error);
            process::exit(2);
        }
    }
}

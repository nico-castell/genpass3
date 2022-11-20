// MIT License - Copyright (c) 2022 Nicolás Castellán
// SPDX License identifier: MIT
// THE SOFTWARE IS PROVIDED "AS IS"
// Read the included LICENSE file for more information

//! # genpass3
//!
//! `genpass3` is a library and binary crate that allows Linux users to generate very long & secure
//! passwords at incredible speeds using `/dev/urandom`.

use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

/// The config for the generation of passwords. Taken as an argument by the [`run`](run) function.
///
/// You can create an instance of this struct using [`Config::build`](Config::build).
pub struct Config {
    length: usize,
}

impl Config {
    /// Creates a [`Config`](Config) type. **Assumes** the first iteration of `args` is the program
    /// name, so it's ignores.
    ///
    /// Parameter:
    /// - `args` - An iterator, meant to iterate over the binary's arguments and flags.
    ///
    /// # Errors
    /// This function can result in an error if the length parameter could not be parsed into a
    /// [`usize`](usize).
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
        // Ignore the first argument
        args.next();

        let length = match args.next() {
            Some(length) => length.parse()?,
            None => 16,
        };

        Ok(Config { length })
    }

    /// Prints the configuration options to stderr.
    ///
    /// # Example
    ///
    /// ```
    /// # use genpass3::*;
    /// Config::print_config();
    /// ```
    pub fn print_config() {
        eprint!(
            "\
Usage:
    \x1B[01m{} <LENGTH>\x1B[00m\n
The LENGTH is an optional parameter specifying the desired length of the password.\n
Version: {}, {} License
",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_LICENSE")
        );
    }
}

/// Runs the configured password generation
///
/// Parameter:
/// - `config` - A [`Config`](Config) that contains the configuration for the function.
///
/// # Errors
///
/// This function can return errors if `/dev/urandom` does not exist.
///
/// # Example
///
/// ```
/// # use genpass3::*;
/// # let args = vec![
/// #     String::from("program_name"),
/// #     String::from("20"),
/// # ];
/// # let args = args.into_iter();
/// let config = match Config::build(args) {
///     Ok(config) => config,
///     Err(error) => panic!("Failed to build `Config`: {}", error),
/// };
///
/// genpass3::run(&config);
/// ```
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut password = vec![0u8; config.length];

    {
        // This is a special file available in Linux as a way to get random data.
        // It's a device, it generates characters as we request it.
        let dev_chars = File::open("/dev/urandom")?;
        let mut reader = BufReader::with_capacity(config.length, dev_chars);

        // Read all the data at once.
        reader.read_exact(&mut password)?;
    }

    {
        // We're bounding our characters within 32 and 127 in ASCII.
        let min_ascii = 32;
        let diff_ascii = 95;

        // We need to iterate over a range of numbers because if we were to iterate over the
        // elements of the Vec we would have mutable reference problems.
        #[allow(clippy::needless_range_loop)]
        for i in 0..config.length {
            password[i] = password[i] % diff_ascii + min_ascii;
        }
    }

    // Effectively transforms `Vec<u8>` into `Vec<char>` into `String`.
    let password: String = password.into_iter().map(|c| c as char).collect();

    println!("{}", password);

    Ok(())
}

# CLI Password Generator 3
[![Commits since last release](https://img.shields.io/github/commits-since/nico-castell/genpass3/latest?label=Commits%20since%20last%20release&color=informational&logo=Git&logoColor=white&style=flat-square)](https://github.com/nico-castell/genpass3/commits)
[![Crates version](https://img.shields.io/crates/v/genpass3?color=informational&label=Crate%20version&logo=Rust&logoColor=white&style=flat-square)](https://crates.io/crates/genpass3/versions)
[![License](https://img.shields.io/github/license/nico-castell/genpass3?label=License&color=informational&logo=Open%20Source%20Initiative&logoColor=white&style=flat-square)](LICENSE)
[![Tests](https://img.shields.io/github/workflow/status/nico-castell/genpass3/tests?label=tests&logo=GitHub%20Actions&logoColor=white&style=flat-square)](https://github.com/nico-castell/genpass3/actions/workflows/rust-tests.yml)

This program can read `/dev/urandom` to quickly generate random passwords in Linux.

This is a continuation of my saga of password generators. Some of which include
[genpass2](https://github.com/nico-castell/genpass2),
[Genpass4Win](https://github.com/nico-castell/Genpass4Win), and
[Password-Magician](https://github.com/nico-castell/Password-Magician). (Not rust projects)

## Installation
To install this application, you will need to have **cargo** from the Rust language. If you don't
have it, you can refer to the installation instructions
[here](https://www.rust-lang.org/learn/get-started).

Then you simply run the following command in your terminal:

```
$ cargo install genpass3
```

## Usage

```
$ genpass3
*rGV<QPYe>5"Eag&
```

You can also specify the length of the password you desire:

```
$ genpass3 50
hr#7dE_DpMM)K0(y>) 5'`0 BI|3.?*6;+5OF/SHhJX.rip\W_
```

There's also a `--help` menu:

```
$ genpass3 --help
Usage:
    genpass3 <LENGTH>

The LENGTH is an optional parameter specifying the desired length of the password.

Version: 1.1.0, MIT License
```

## About
This program and this repository are available under an [MIT License](LICENSE).

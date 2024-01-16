# root-rs
 Rust bindings for the C++ ROOT library.

This WIP project aims to provide C and Rust interfaces for the C++ ROOT library.

The C interfaces are manually curated and can be found in the [`root-rs-c-bindings`](./root-rs-c-bindings/) directory. Please note that covering the entire ROOT C API is not the primary objective of this project. If you wish to request a new feature or contribute to the development, feel free to [open an issue](https://github.com/LucaCiucci/root-rs/issues) or submit a [pull request](https://github.com/LucaCiucci/root-rs/pulls).

## Usage

See the [examples](./examples/) directory for some examples of how to use the library.

## Requirements

Root 6.22 or later is required. See <https://root.cern/install/> for installation instructions.

## Status of the project

The project is in an very early stage of development and the current codebase is a proof of concept.

### OS support

| OS      | Status |
| ------- | ------ |
| Almalinux | ❌ |
| Centosstream | ❌ |
| Fedora | ❌ |
| Ubuntu | 🚧 |
| macOS | ❌ |
| Windows | ✅ |
# root-rs
 Rust bindings for the C++ ROOT library.

This is a WIP project to provide C and Rust bindings for the C++ ROOT library.

C bindings are manually maintained in the [`root-rs-c-bindings`](./root-rs-c-bindings/) directory, supporting 100% of the ROOT C API is a **non-goal** of this project. You can [open an issue](https://github.com/LucaCiucci/root-rs/issues) or a [pull request](https://github.com/LucaCiucci/root-rs/pulls) to request a new feature or help with the development.

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
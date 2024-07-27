# Typst Library Swift

Swift Package for using Typst Library in iOS, iPadOS, and macOS applications developed in Swift

## Requirements

- macOS Monterey (12.0) or later
- Xcode 13.0 or later
- rustup
- Rust 1.80.0 or later, **installed via rustup**
- [cargo-swift](https://crates.io/crates/cargo-swift), **installed via `cargo install cargo-swift`**

## Build Swift Package

1. Open the project in Terminal
2. Run `cargo swift package`
3. Keep the default package name `TypstLibrarySwift` and build target **macOS** and **iOS**
4. The Swift package will be built in the `TypstLibrarySwift` directory

## Acknowledgments

This project relies on the following open-source projects:

- [cargo-swift](https://github.com/antoniusnaumann/cargo-swift) for building Swift packages from Rust code
- [Typst Libraries](https://crates.io/crates/typst) for [Typst](https://typst.app) related functionality such as Typst document rendering

This project heavily relies on the implementation of [tfachmann/typst-as-library](https://github.com/tfachmann/typst-as-library)
for the Typst Wrapper in Rust, and other implementations to interact with the Typst Library.

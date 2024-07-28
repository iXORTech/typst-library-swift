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

### Embedded Fonts

To make sure that the Typst Library can render the text whenever the library is used, some fonts are embedded in the Swift package.

They are **all** open-source fonts, licensed under the [SIL Open Font License](https://openfontlicense.org/).

The fonts are:

- [CMU* Concrete](https://fontlibrary.org/en/font/cmu-concrete)
- [CMU* Sans Serif](https://fontlibrary.org/en/font/cmu-sans-serif)
- [CMU* Serif](https://fontlibrary.org/en/font/cmu-serif)
- [CMU* Typewriter](https://fontlibrary.org/en/font/cmu-typewriter)
- [IBM Plex Mono](https://www.ibm.com/plex/)
- [IBM Plex Sans](https://www.ibm.com/plex/)
- [IBM Plex Serif](https://www.ibm.com/plex/)
- [STIX Two Math](https://stixfonts.org/)
- [LXGW WenKai Mono Lite](https://github.com/lxgw/LxgwWenKai-Lite)

* CMU stands for Computer Modern Unicode, which is a [derivative of the Computer Modern font family](https://en.wikipedia.org/wiki/Computer_Modern).

By default, the fonts always falls backs to:

- STIX Two Math for mathematical formulas
- IBM Plex Mono for raw text (e.g. code)
- IBM Plex Sans and LXGW WenKai Mono Lite for normal text (LXGW WenKai Mono Lite is used for characters that are not supported by IBM Plex Sans, such as Chinese characters)

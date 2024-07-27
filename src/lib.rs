uniffi::include_scaffolding!("lib");

mod typst_wrapper_world;

// Bindings have to be exposed in a .udl file with the same name as the corresponding .rs file, i.e. lib.udl
// You can expose top-level functions...
pub fn add(a: u64, b: u64) -> u64 {
    a + b
}

// ... data structs without methods ...
pub struct Example {
    pub items: Vec<String>,
    pub value: Option<f64>,
}

// ... classes with methods ...
pub struct Greeter {
    name: String,
}

impl Greeter {
    // By convention, a method called new is exposed as a constructor
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn greet(&self) -> String {
        format!("Hello, {}!\nThis is from a Rust Library!", self.name)
    }
}

// ... and much more! For more information about bindings, read the UniFFI book: https://mozilla.github.io/uniffi-rs/udl_file_spec.html

mod tests {
    use std::fs;
    use typst_library_swift::get_rendered_document;

    #[test]
    fn test_get_rendered_document() {
        let source = r#"
#import "@preview/polylux:0.3.1": *
#import themes.simple: *

#set page(paper: "presentation-16-9")

#show: simple-theme.with()

#title-slide[
= Hello, World!
A document (+ `polylux` library) rendered with `Typst`!
]

#polylux-slide[
  == This slide contains some Math Formulas

  $
  integral_a^b f(x) d x = lim_(n->infinity) Delta x sum_(i=1)^n f(x_i) space space "where"
  space space Delta x = (b-a)/n space space "and" space space x_i = a + i Delta x
  $
]

#polylux-slide[
  == This slide contains some code!

  ```rust
  // This is a comment, and is ignored by the compiler.
  // You can test this code by clicking the "Run" button over there ->
  // or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
  // shortcut.

  // This code is editable, feel free to hack it!
  // You can always return to the original code by clicking the "Reset" button ->

  // This is the main function. 0123 oOiIlL
  fn main() {
      // Statements here are executed when the compiled binary is called.

      // Print text to the console.
      println!("Hello World!");
  }
  ```
]

#centered-slide[
    #emph(text(size: 48pt)[
        落霞与孤鹜齐飞\
        秋水共长天一色
    ])

    #emph(text(size: 24pt)[
        ﹝唐﹞王勃《滕王阁序》
    ])
]
"#.to_owned();

        let document = get_rendered_document(source);

        fs::write("./output.svg", document).expect("Error writing SVG.");
        println!("Created svg: `./output.svg`");
    }
}
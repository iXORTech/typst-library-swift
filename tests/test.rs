mod tests {
    use std::fs;
    use typst_library_swift::{get_rendered_document_pdf, get_rendered_document_svg};

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
  == This slide contains some text

  The quick brown fox jumps over the lazy dog

  *The quick brown fox jumps over the lazy dog*

  _The quick brown fox jumps over the lazy dog_

  *_The quick brown fox jumps over the lazy dog_*
]

#polylux-slide[
  1. Item 1
  2. Item 2
    2.1. Subitem 2.1

  - Item 1
  - Item 2
    - Subitem 2.1
]

#polylux-slide[
  == Symbols

  $
    [| space |] space || space * space := space ::= space ... \
    ' space - space =: space != space >> space >= space >>> \
    << space <= space <<< space -> space |-> space => space |=> \
    ==> space --> space ~~> space ~> space >-> space ->> space <- \
    <== space <-- space <~~ space <~ space <-< space <<- space <-> \
    <=> space <==> space <-->
  $
]

#polylux-slide[
  #text(font: "CMU Concrete")[
    == CMU Concrete

    The quick brown fox jumps over the lazy dog

    *The quick brown fox jumps over the lazy dog*

    _The quick brown fox jumps over the lazy dog_

    *_The quick brown fox jumps over the lazy dog_*
  ]
]

#polylux-slide[
  #text(font: "CMU Sans Serif")[
    == CMU Sans Serif

    The quick brown fox jumps over the lazy dog

    *The quick brown fox jumps over the lazy dog*

    _The quick brown fox jumps over the lazy dog_

    *_The quick brown fox jumps over the lazy dog_*
  ]
]

#polylux-slide[
  #text(font: "CMU Serif")[
    == CMU Serif

    The quick brown fox jumps over the lazy dog

    *The quick brown fox jumps over the lazy dog*

    _The quick brown fox jumps over the lazy dog_

    *_The quick brown fox jumps over the lazy dog_*
  ]
]

#polylux-slide[
  #text(font: "CMU Typewriter Text")[
    == CMU Typewriter

    The quick brown fox jumps over the lazy dog

    *The quick brown fox jumps over the lazy dog*

    _The quick brown fox jumps over the lazy dog_

    *_The quick brown fox jumps over the lazy dog_*
  ]
]

#polylux-slide[
  #text(font: "IBM Plex Mono")[
    == IBM Plex Mono

    The quick brown fox jumps over the lazy dog

    *The quick brown fox jumps over the lazy dog*

    _The quick brown fox jumps over the lazy dog_

    *_The quick brown fox jumps over the lazy dog_*
  ]
]

#polylux-slide[
  #text(font: "IBM Plex Sans")[
    == IBM Plex Sans

    The quick brown fox jumps over the lazy dog

    *The quick brown fox jumps over the lazy dog*

    _The quick brown fox jumps over the lazy dog_

    *_The quick brown fox jumps over the lazy dog_*
  ]
]

#polylux-slide[
  #text(font: "IBM Plex Serif")[
    == IBM Plex Serif

    The quick brown fox jumps over the lazy dog

    *The quick brown fox jumps over the lazy dog*

    _The quick brown fox jumps over the lazy dog_

    *_The quick brown fox jumps over the lazy dog_*
  ]
]

#polylux-slide[
  == This slide contains some Math Formulas

  $
  B(P) = (mu_0)/(4pi) integral (I times hat(r)')/(r^('2)) d l = (mu_0)/(4pi) I integral (d l times hat(r)')/(r^('2))
  $

  $
  integral_a^b f(x) d x = lim_(n->infinity) Delta x sum_(i=1)^n f(x_i) space "where"
  space Delta x = (b-a)/n , x_i = a + i Delta x
  $
]

#polylux-slide[
  == This slide contains some code!

  ```rust
  // This is the main function. 0123 oOiIlL
  fn main() {
      // Statements here are executed when the compiled binary is called.

      // Print text to the console.
      println!("Hello World!");
  }
  ```
]

#centered-slide[
    #text(size: 48pt)[
        落霞与孤鹜齐飞\
        秋水共长天一色
    ]

    #text(size: 24pt)[
        ﹝唐﹞王勃《滕王阁序》
    ]
]
"#.to_owned();

        let svg = get_rendered_document_svg(source.clone());
        fs::write("./output.svg", svg).expect("Error writing SVG.");
        println!("Created svg: `./output.svg`");

        let pdf = get_rendered_document_pdf(source.clone());
        fs::write("./output.pdf", pdf).expect("Error writing PDF.");
        println!("Created pdf: `./output.pdf`");
    }
}

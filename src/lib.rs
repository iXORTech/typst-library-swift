use typst::{eval::Tracer, layout::Abs};

mod typst_wrapper_world;

uniffi::include_scaffolding!("lib");


fn add_fallback_font(source: String) -> String {
    format!(
        "#show math.equation: set text(font: \"STIX Two Math\")\
        \n#show raw: set text(font: \"JetBrains Mono\")\
        \n#set text(font: (\"LXGW WenKai Mono Lite\"))\
        \n{}",
        source
    )
}

pub fn get_rendered_document(source: String) -> String {
    let source = add_fallback_font(source);

    let world = typst_wrapper_world::TypstWrapperWorld::new(
        "./".to_owned(), source
    );

    // Render document
    let mut tracer = Tracer::default();
    let document = typst::compile(&world, &mut tracer).expect("Error compiling typst.");

    // Render SVG and return the SVG string
    let svg = typst_svg::svg_merged(&document, Abs::pt(2.0));
    svg
}

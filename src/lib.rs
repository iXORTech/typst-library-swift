use typst::foundations::Smart;
use typst::{eval::Tracer, layout::Abs};

mod typst_wrapper_world;

uniffi::include_scaffolding!("lib");


pub fn get_rendered_document(source: String) -> String {
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

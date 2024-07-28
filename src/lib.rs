use typst::{eval::Tracer, layout::Abs};
use typst::foundations::Smart;
use typst::model::Document;

mod typst_wrapper_world;

uniffi::include_scaffolding!("lib");


fn add_fallback_font(source: String) -> String {
    format!(
        "#show math.equation: set text(font: \"STIX Two Math\")\
        \n#show raw: set text(font: \"IBM Plex Mono\")\
        \n#set text(font: (\"IBM Plex Sans\", \"LXGW WenKai Mono Lite\"))\
        \n{}",
        source
    )
}

fn get_rendered_document(source: String) -> Document {
    let source = add_fallback_font(source);

    let world = typst_wrapper_world::TypstWrapperWorld::new(
        "./".to_owned(), source
    );

    // Render document
    let mut tracer = Tracer::default();
    typst::compile(&world, &mut tracer).expect("Error compiling typst.")
}

pub fn get_rendered_document_svg(source: String) -> String {
    // Render SVG and return the SVG string
    typst_svg::svg_merged(&get_rendered_document(source), Abs::pt(2.0))
}

pub fn get_rendered_document_pdf(source: String) -> Vec<u8> {
    // Render PDF and return the PDF bytes
    typst_pdf::pdf(&get_rendered_document(source), Smart::Auto, None)
}

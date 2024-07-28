use typst::{eval::Tracer, layout::Abs};
use typst::foundations::Smart;
use typst::model::Document;

mod typst_wrapper_world;

uniffi::include_scaffolding!("lib");


#[derive(Debug, thiserror::Error)]
pub enum DocumentRenderError {
    #[error("Typst Compilation Error: {0}")]
    TypstCompilationError(String),
}

fn add_fallback_font(source: String) -> String {
    format!(
        "#show math.equation: set text(font: \"STIX Two Math\")\
        \n#show raw: set text(font: \"IBM Plex Mono\")\
        \n#set text(font: (\"IBM Plex Sans\", \"LXGW WenKai Mono Lite\"))\
        \n{}",
        source
    )
}

fn get_rendered_document(source: String) -> Result<Document, DocumentRenderError> {
    let source = add_fallback_font(source);

    let world = typst_wrapper_world::TypstWrapperWorld::new(
        "./".to_owned(), source
    );

    // Render document
    let mut tracer = Tracer::default();

    let r = typst::compile(&world, &mut tracer);

    match r {
        Ok(document) => Ok(document),
        Err(e) => Err(DocumentRenderError::TypstCompilationError(
            e[0].message.to_string()
        )),
    }
}

pub fn get_rendered_document_svg(source: String) -> Result<String, DocumentRenderError> {
    // Render SVG and return the SVG string
    Ok(typst_svg::svg_merged(&get_rendered_document(source)?, Abs::pt(2.0)))
}

pub fn get_rendered_document_pdf(source: String) -> Result<Vec<u8>, DocumentRenderError> {
    // Render PDF and return the PDF bytes
    Ok(typst_pdf::pdf(&get_rendered_document(source)?, Smart::Auto, None))
}

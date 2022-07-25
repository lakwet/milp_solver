use super::super::super::forms::builder::StandardFormBuilder;
use super::super::super::forms::standard::StandardFormLP;

#[test]
fn forms_builder_standardformbuilder_new_empty() {
    let err = StandardFormBuilder::new().build().unwrap_err();

    // assert!();
}

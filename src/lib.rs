use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn latex_to_mathml(input: String) -> String {
    latex2mathml::latex_to_mathml(&input, latex2mathml::DisplayStyle::Block).unwrap()
}

pub use web_sys::{Element, HtmlElement};
pub use {gloo, gloo_console as console, js_sys, wasm_bindgen, wasm_dom as dom, web_sys};

pub use self::tabs::*;

pub mod tabs;

pub trait AsElement {
    fn as_element(&self) -> &Element;
}

pub trait AsElementMut {
    fn as_element_mut(&mut self) -> &mut Element;
}

pub trait AsHtmlElement {
    fn as_html_element(&self) -> &HtmlElement;
}

pub trait AsHtmlElementMut {
    fn as_html_element_mut(&mut self) -> &mut HtmlElement;
}

impl<T: AsRef<Element>> AsElement for T {
    fn as_element(&self) -> &Element {
        self.as_ref()
    }
}

impl<T: AsMut<Element>> AsElementMut for T {
    fn as_element_mut(&mut self) -> &mut Element {
        self.as_mut()
    }
}

impl<T: AsRef<HtmlElement>> AsHtmlElement for T {
    fn as_html_element(&self) -> &HtmlElement {
        self.as_ref()
    }
}

impl<T: AsMut<HtmlElement>> AsHtmlElementMut for T {
    fn as_html_element_mut(&mut self) -> &mut HtmlElement {
        self.as_mut()
    }
}

use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = HtmlElement)]
    pub type Tab;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = HtmlElement, extends = Tab)]
    pub type PrimaryTab;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = HtmlElement, extends = Tab, extends = PrimaryTab)]
    pub type MdPrimaryTab;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = HtmlElement)]
    pub type Tabs;

    #[wasm_bindgen(constructor, js_class = Tabs)]
    pub fn new() -> Tabs;

    #[wasm_bindgen(method, getter)]
    pub fn tabs(this: &Tabs) -> Vec<Tab>;

    #[wasm_bindgen(method, getter = activeTab)]
    pub fn active_tab(this: &Tabs) -> Tab;

    #[wasm_bindgen(method, setter = activeTab)]
    pub fn set_active_tab(this: &Tabs, tab: Tab);

    #[wasm_bindgen(method, getter = activeTabIndex)]
    pub fn active_tab_index(this: &Tabs) -> i32;

    #[wasm_bindgen(method, setter = activeTabIndex)]
    pub fn set_active_tab_index(this: &Tabs, tab_index: i32);

    #[wasm_bindgen(method, getter = autoActivate)]
    pub fn auto_activate(this: &Tabs) -> bool;

    #[wasm_bindgen(method, setter = autoActivate)]
    pub fn set_auto_activate(this: &Tabs, auto_activate: bool);

    #[wasm_bindgen(method, js_name = scrollToTab)]
    pub async fn scroll_to_tab(this: &Tabs, tab_to_scroll_to: Tab);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Tabs)]
    pub type MdTabs;
}

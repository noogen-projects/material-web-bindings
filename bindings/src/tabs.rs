use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = HtmlElement)]
    pub type Tab;

    #[wasm_bindgen(constructor, js_class = Tab)]
    pub fn new() -> Tab;

    #[wasm_bindgen(method, getter = isTab)]
    pub fn is_tab(this: &Tab) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn active(this: &Tab) -> bool;

    #[wasm_bindgen(method, setter = active)]
    pub fn set_active(this: &Tab, active: bool);

    #[wasm_bindgen(method, getter = hasIcon)]
    pub fn has_icon(this: &Tab) -> bool;

    #[wasm_bindgen(method, setter = hasIcon)]
    pub fn set_has_icon(this: &Tab, has_icon: bool);

    #[wasm_bindgen(method, getter = iconOnly)]
    pub fn icon_only(this: &Tab) -> bool;

    #[wasm_bindgen(method, setter = iconOnly)]
    pub fn set_icon_only(this: &Tab, icon_only: bool);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = HtmlElement, extends = Tab)]
    pub type PrimaryTab;

    #[wasm_bindgen(method, getter = inlineIcon)]
    pub fn inline_icon(this: &PrimaryTab) -> bool;

    #[wasm_bindgen(method, setter = inlineIcon)]
    pub fn set_inline_icon(this: &PrimaryTab, inline_icon: bool);
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

use leptos::{component, view, IntoView};
use material_web_bindings::wasm_bindgen::JsCast;
use material_web_bindings::web_sys::HtmlElement;
use material_web_bindings::{dom, Tabs};

#[component]
pub fn View() -> impl IntoView {
    view! {
        <md-tabs aria-label = "Primary tabs" active-tab-index = 0 on:change = {
            let mut current_panel: Option<HtmlElement> = None;
            move |event| {
                let mut panel = current_panel.take().unwrap_or_else(|| dom::existing::select_element("#panel-one"));
                panel.set_hidden(true);

                let tabs: Tabs = event
                    .target()
                    .expect("")
                    .unchecked_into();
                let tab = tabs.active_tab();
                let tab_element: &HtmlElement = tab.as_ref();
                let panel_id = tab_element.get_attribute("aria-controls").unwrap_or_default();

                panel = dom::existing::select_element(&format!("#{panel_id}"));
                panel.set_hidden(false);
                current_panel = Some(panel);
            }
        }>
            <md-primary-tab id = "tab-one" aria-controls = "panel-one">
                <md-icon slot = "icon">piano</md-icon>
                Keyboard
            </md-primary-tab>
            <md-primary-tab id = "tab-two" aria-controls = "panel-two">
                <md-icon slot = "icon">tune</md-icon>
                Guitar
            </md-primary-tab>
            <md-primary-tab id = "tab-three" aria-controls = "panel-three">
                <md-icon slot = "icon">graphic_eq</md-icon>
                Drums
            </md-primary-tab>
            <md-primary-tab id = "tab-four" aria-controls = "panel-four">
                <md-icon slot = "icon">speaker</md-icon>
                Bass
            </md-primary-tab>
            <md-primary-tab id = "tab-five" aria-controls = "panel-five">
                <md-icon slot = "icon">nightlife</md-icon>
                Saxophone
            </md-primary-tab>
        </md-tabs>

        <div
            role = "tabpanel"
            class = "md-typescale-body-medium"
            id = "panel-one"
            aria-labelledby = "tab-one"
        >
            Keyboard
        </div>
        <div
            role = "tabpanel"
            class = "md-typescale-body-medium"
            id = "panel-two"
            aria-labelledby = "tab-two"
            hidden
        >
            Guitar
        </div>
        <div
            role = "tabpanel"
            class = "md-typescale-body-medium"
            id = "panel-three"
            aria-labelledby = "tab-three"
            hidden
        >
            Drums
        </div>
        <div
            role = "tabpanel"
            class = "md-typescale-body-medium"
            id = "panel-four"
            aria-labelledby = "tab-four"
            hidden
        >
            Bass
        </div>
        <div
            role = "tabpanel"
            class = "md-typescale-body-medium"
            id = "panel-five"
            aria-labelledby = "tab-five"
            hidden
        >
            Saxophone
        </div>
    }
}

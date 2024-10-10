use leptos::{component, view, IntoView};

mod tabs;

#[component]
fn App() -> impl IntoView {
    view! {
        <header class = "top-app-bar">
            <section class = "top-app-bar__section top-app-bar__section-start">
                <md-icon-button>
                    <span class = "material-icons mdc-icon-button top-app-bar__navigation-icon">menu</span>
                </md-icon-button>
                <span class = "top-app-bar__title">"Material Web Bindings"</span>
            </section>
            <section class = "top-app-bar__section top-app-bar__section-end">
                <md-icon-button>
                    <span class = "material-icons mdc-icon-button top-app-bar__navigation-icon" aria-label = "Options">more_vert</span>
                </md-icon-button>
            </section>
        </header>
        <main class = "content-panel">
            <h1 class = "demo-title">Material Web components</h1>
            <a href = "#tabs">Tabs</a>

            <h2 class = "demo-title"><a href = "#" name = "tabs"></a>Tabs</h2>
            <tabs::View />
        </main>
        <footer>
        </footer>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}

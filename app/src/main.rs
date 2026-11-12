use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

// Static CyberChef files, mounted by the server below
const CYBERCHEF_URL: &str = "/cyberchef-app/";

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
enum Route {
    #[layout(Shell)]
        #[route("/")]
        Home {},
        #[route("/cyberchef")]
        CyberChef {},
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

/// Hosted services shown on the landing page and in the nav.
/// Add new services here.
#[derive(Clone, PartialEq)]
struct Service {
    name: &'static str,
    description: &'static str,
    to: Route,
}

fn services() -> Vec<Service> {
    vec![Service {
        name: "CyberChef",
        description: "Encode, decode, encrypt, compress and analyse data in the browser.",
        to: Route::CyberChef {},
    }]
}

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::server::axum::routing::get;
        use tower_http::services::ServeDir;

        let cyberchef_dir =
            std::env::var("CYBERCHEF_DIR").unwrap_or_else(|_| "cyberchef".to_string());

        let router = dioxus::server::router(App)
            .route("/health", get(|| async { "ok" }))
            .nest_service("/cyberchef-app", ServeDir::new(cyberchef_dir));

        Ok(router)
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Shell() -> Element {
    rsx! {
        div { class: "shell",
            header {
                Link { class: "brand", to: Route::Home {}, "util.xyz" }
                nav {
                    for service in services() {
                        Link { key: "{service.name}", to: service.to.clone(), "{service.name}" }
                    }
                }
            }
            main { Outlet::<Route> {} }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "home",
            h1 { "Hosted services" }
            div { class: "cards",
                for service in services() {
                    Link { key: "{service.name}", class: "card", to: service.to.clone(),
                        h2 { "{service.name}" }
                        p { "{service.description}" }
                    }
                }
            }
        }
    }
}

#[component]
fn CyberChef() -> Element {
    rsx! {
        div { class: "frame-page",
            div { class: "frame-bar",
                a { href: CYBERCHEF_URL, target: "_blank", rel: "noopener", "Open full screen ↗" }
            }
            iframe { class: "service-frame", title: "CyberChef", src: CYBERCHEF_URL }
        }
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { class: "not-found",
            h1 { "404" }
            p { "No page at /{segments.join(\"/\")}" }
            Link { to: Route::Home {}, "Back to home" }
        }
    }
}

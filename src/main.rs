
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    tracing_wasm::set_as_global_default();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx!{
        h1 { "count: {count}"}
        button {  
            onclick: move |_e| { count+=1},
            "+1"
        }
        button {  
            onclick: move |_e| { count.set(0)},
            "reset"
        }
        button {  
            onclick: move |_e| { count-=1},
            "-1"
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
        Router::<Route> {}
        
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            span {
                div { id: "links",
                    a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                    a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                    a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                    a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                    a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                    a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
                }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}

    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "layout",
            class: "flex min-h-screen",
            h1 {
                id: "title",
                class: "text-5xl font-extrabold text-left my-8 text-transparent bg-clip-text bg-gradient-to-r dark:from-blue-500 from-yellow-500 dark:to-purple-600 to-orange-600 drop-shadow-md hover:underline",
                "Title"
            }
            // Sidebar
            Sidebar {}

            // Main content area
            div {
                id: "content",
                class: "flex-grow flex flex-col",

                // Main content of the page
                Outlet::<Route> {}

                // Footer at the bottom
                Footer {}
            }
        }
    }
}


#[component]
fn Footer() -> Element{
    rsx!{
        div {
            id:"footer",
            "© 2025 My Website. All rights reserved."
        }
    }
}

#[component]
fn Sidebar() -> Element {
    rsx!{
        div {
            id: "sidebar",
            class: "bg-gray-200 w-64 p-4 h-full",
            ul {
                li { a { href: "/", "🏠 Home" } }
                li { a { href: "/blog/1", "📝 Blog" } }
                li { a { href: "#", "📚 Documentation" } }
                li { a { href: "#", "📞 Contact" } }
            }
        }
    }
}
#[derive(Props, PartialEq, Clone)]
struct Money {
    currency: String,
    amount: i32
}

#[component]
fn Transfer(transfer: ReadOnlySignal<Money>) -> Element{
    let transfer: &Money=&transfer.read();
    rsx!{
        p {
            "I have transfered {transfer.amount} {transfer.currency}."
        }
    }
}
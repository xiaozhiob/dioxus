//! An example where the dioxus vdom is running in a native thread, interacting with webview
//! Content is passed from the native thread into the webview
use dioxus_core::prelude::*;

fn main() {
    dioxus_webview::launch(
        |builder| {
            builder
                .title("Test Dioxus App")
                .size(320, 480)
                .resizable(false)
                .debug(true)
        },
        (),
        Example,
    )
    .expect("Webview finished");
}

static Example: FC<()> = |ctx, _props| {
    ctx.render(rsx! {
        div  {
            class: "flex items-center justify-center flex-col"
            div {
                class: "flex items-center justify-center"
                div {
                    class: "flex flex-col bg-white rounded p-4 w-full max-w-xs"
                    div { class: "font-bold text-xl", "Example desktop app" }
                    div { class: "text-sm text-gray-500", "This is running natively" }
                    div {
                        class: "flex flex-row items-center justify-center mt-6"
                        div { class: "font-medium text-6xl", "100%" }
                    }
                    div {
                        class: "flex flex-row justify-between mt-6"
                        a {
                            href: "https://www.dioxuslabs.com"
                            class: "underline"
                            "Made with dioxus"
                        }
                    }
                }
            }
        }
    })
};

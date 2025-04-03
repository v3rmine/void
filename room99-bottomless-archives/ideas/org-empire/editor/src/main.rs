use sycamore::prelude::*;

fn main() {
    sycamore::render(|ctx| {
        view! {
            ctx,
            div(class="hero min-h-screen bg-base-200") {
                div(class="text-center hero-content") {
                    div(class="max-w-md") {
                        h1(class="mb-5 text-4xl font-bold") {
                            "Hello World!"
                        }
                    }
                }
            }
        }
    });
}

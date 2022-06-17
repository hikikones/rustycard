use dioxus::prelude::*;

use crate::markdown;

#[allow(non_snake_case)]
#[inline_props]
pub fn MarkdownView<'a>(cx: Scope, text: &'a str) -> Element {
    cx.render(rsx! {
        div {
            dangerous_inner_html: format_args!("{}", markdown::to_html(text)),
        }
    })
}

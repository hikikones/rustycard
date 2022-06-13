use dioxus::prelude::*;
use pulldown_cmark::Parser;

#[allow(non_snake_case)]
#[inline_props]
pub fn MarkdownView<'a>(cx: Scope, text: &'a str) -> Element {
    cx.render(rsx! {
        div {
            dangerous_inner_html: format_args!("{}", markdown_to_html(text)),
        }
    })
}

fn markdown_to_html(md: &str) -> String {
    let parser = Parser::new(md);
    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    html_buf
}

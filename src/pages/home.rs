use dioxus::{events::FormEvent, prelude::*};
use pulldown_cmark::Parser;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    let markdown = use_state(&cx, || String::from("Yoyo"));

    cx.render(rsx! {
        h1 { "Home" }
        CardEditor {
            value: markdown,
            onchange: |evt: FormEvent| {
                println!("{:?}", evt);
                markdown.set(evt.value.clone());
            }
        }
    })
}

#[derive(Props)]
struct CardEditorProps<'a> {
    value: &'a str,
    onchange: EventHandler<'a, FormEvent>,
}

#[allow(non_snake_case)]
fn CardEditor<'a>(cx: Scope<'a, CardEditorProps<'a>>) -> Element {
    let value = use_state(&cx, || cx.props.value.to_owned());

    cx.render(rsx! {
        textarea {
            rows: "10",
            cols: "80",
            oninput: |evt| {
                value.set(evt.value.clone());
            },
            onchange: |evt| cx.props.onchange.call(evt),
            "{cx.props.value}"
        }
        MarkdownView {
            text: "{value}",
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn MarkdownView<'a>(cx: Scope, text: &'a str) -> Element {
    cx.render(rsx! {
        div {
            dangerous_inner_html: format_args!("{}", render_markdown(text)),
        }
    })
}

fn render_markdown(text: &str) -> String {
    let parser = Parser::new(text);
    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    html_buf
}

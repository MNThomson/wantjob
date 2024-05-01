use leptos::IntoView;
use minify_html::{minify, Cfg};
pub mod homepage;
pub mod layout;

#[tracing::instrument(skip_all)]
fn minify_html(str: String) -> String {
    return String::from_utf8(minify(str.as_bytes(), &Cfg::new())).unwrap();
}

#[tracing::instrument(skip_all)]
pub fn htmlify<F, N>(f: F) -> String
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    let ssr = leptos::ssr::render_to_string(f).to_string();

    return minify_html(ssr);
}

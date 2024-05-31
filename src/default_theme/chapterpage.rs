use futures::executor::block_on;
use leptos::{component, view, IntoView};
use rscx::html;
// use rscx_mdx::mdx::{Mdx, MdxComponentProps};
use crate::{models::Chapter, renderer::async_component::Async};
use pulldown_cmark::{html, Options, Parser};
use leptos_mdx::mdx::{Components, Mdx, MdxComponentProps};
use leptos::*;


#[component]
pub fn ChapterPage(
    #[prop(into)] chapter: Chapter,
) -> impl IntoView {

    let algo = chapter.title;
    let content = chapter.content.clone();
    let content = content.unwrap();
    let mut components = Components::new();

    view!{
        <div class="markdown-container prose dark:prose-invert max-w-none">
            <MarkdownRender content=content />
        </div>
    }

}


#[component]
pub fn MarkdownRender(content: String) -> impl IntoView {
    let mut components = Components::new();

    view! {
        <>
            <Mdx source=content components=components/>
        </>
    }
}

async fn handle(name: String, props: MdxComponentProps) -> String {
    match name.as_str() {
        _ => String::new(),
    }
}
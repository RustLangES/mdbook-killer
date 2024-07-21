use leptos::{component, view, IntoView};
// use rscx_mdx::mdx::{Mdx, MdxComponentProps};
use crate::models::Chapter;
use leptos_mdx::mdx::{Components, Mdx};

#[component]
pub fn ChapterPage(#[prop(into)] chapter: Chapter) -> impl IntoView {
    let content = chapter.content.clone();
    let content = content.unwrap();

    view! {
        <div class="markdown-container prose dark:prose-invert max-w-none">
            <MarkdownRender content=content />
        </div>
    }
}

#[component]
pub fn MarkdownRender(content: String) -> impl IntoView {
    let components = Components::new();

    view! {
        <>
            <Mdx source=content components=components/>
        </>
    }
}

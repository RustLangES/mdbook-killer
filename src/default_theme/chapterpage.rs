use futures::executor::block_on;
use leptos::{component, view, IntoView};
use rscx::html;
use rscx_mdx::mdx::{Mdx, MdxComponentProps};
use crate::{models::Chapter, renderer::async_component::Async};

#[component]
pub fn ChapterPage(
    #[prop(into)] chapter: Chapter,
) -> impl IntoView {

    let algo = chapter.title;
    let content = chapter.content.clone();
    let content = content.unwrap();

    view!{
        <div>
            <h1>{algo}</h1>
            {
                view!{
                    <div>
                        <Async view={move || generate_view(content.clone())} />
                    </div>                
                }
            }
        </div>
    }

}

async fn generate_view(content: String) -> impl IntoView {
    let res = html! {
        <Mdx source=content handler=handle />
    };

    view! {
        <div>
            {res}
        </div>
    }
}

async fn handle(name: String, props: MdxComponentProps) -> String {
    match name.as_str() {
        _ => String::new(),
    }
}
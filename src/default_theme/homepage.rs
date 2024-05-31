use futures::executor::block_on;
use leptos::{component, view, CollectView, IntoView};

use crate::{commands::CONFIG, default_theme::{chapterpage::ChapterPage, layout::Layout}, models::{Chapter, Config}};


async fn fetch_config() -> Config {
    CONFIG.read().await.clone().unwrap()
}
#[component]
pub fn Homepage(
    #[prop(optional)] chapter: Option<Chapter>,
    #[prop()] chapters: Vec<Chapter>
) -> impl IntoView {
    let first_chapter = chapters.first().unwrap().clone();

    view! {
        <Layout is_home=true wide=false>
            <nav class="dark:bg-[#101010] fixed left-0 min-w-52 border-r border-gray-700 h-full py-2">
                {
                    chapters.into_iter().map(|chapter| view! {
                        <div class="px-2 py-1">
                            <a href=format!("/en/{}.html", chapter.slug.unwrap()) >{chapter.title}</a>
                        </div>
                    }).collect_view()
                }
            </nav>
            <div class="ml-52 px-6">
                <div class="flex w-full flex-row flex-1 items-center mt-6">
                    <ChapterPage chapter={chapter.unwrap_or(first_chapter)} />
                </div>
            </div>
        </Layout>
    }
}

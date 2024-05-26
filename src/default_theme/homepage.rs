use futures::executor::block_on;
use leptos::{component, view, CollectView, IntoView};

use crate::{commands::CONFIG, default_theme::layout::Layout, models::Config};


async fn fetch_config() -> Config {
    CONFIG.read().await.clone().unwrap()
}
#[component]
pub fn Homepage() -> impl IntoView {
    let config = block_on(fetch_config());
    let title = config.book.title.unwrap();
    let description = config.book.description.unwrap();


    view! {
        <Layout is_home=true>
            <h1 class="font-semibold font-work-sans text-3xl text-center lg:text-left mt-2">
                {title}
            </h1>
            <p class="text-xl mb-3">
                {description}
            </p>
            <div class="flex w-full flex-row flex-1 items-center mt-6">
                <div class="w-[50%] flex flex-row items-center">
                    <h1 class="font-semibold font-work-sans text-3xl text-center lg:text-left my-4">
                        "Artículos"
                    </h1>
                </div>
                <div class="w-[50%] flex justify-end items-center gap-4">
                </div>
            </div>
        </Layout>
    }
}

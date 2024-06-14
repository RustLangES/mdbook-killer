use chrono::Datelike;
use futures::executor::block_on;
use leptos::Children;
use leptos::{component, view, IntoView};

use crate::commands::CONFIG;
use crate::default_theme::meta::Head;
use crate::default_theme::meta::Html;
use crate::models::Config;

fn get_year() -> i32 {
    chrono::Utc::now().year()
}
async fn fetch_config() -> Config {
    CONFIG.read().await.clone().unwrap()
}

#[component]
// This is a common Layout component that will be used by all pages.
pub fn Layout(
    #[prop(into, default="rustlanges_preview.webp".to_string())] slug: String,
    #[prop(into, default = false)] is_home: bool,
    #[prop(into, default = "".to_string())] language: String,
    #[prop(into, default = false)] wide: bool,
    children: Children,
) -> impl IntoView {
    let config = block_on(fetch_config());
    let title = config.book.title.unwrap();
    let title_clone = title.clone();
    let description = config.book.description.unwrap();
    let description_clone = title.clone();

    let language = if language.is_empty() {
        "en".to_string()
    } else {
        language
    };

    view! {
        <Html
            attrs=vec![("lang", language.as_str())]
            class="bg-[#fed7aac9] dark:bg-[#131313]/90 bg-center bg-fixed dark:bg-kaku dark:bri dark:bg-cover dark:bg-blend-darken dark:backdrop-blur-xl overflow-x-hidden dark:text-[#e2cea9] min-h-screen"
        />
        <Head>
            <meta charset="utf-8"/>
            <title>{title.clone()}</title>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <meta property="og:title" content=title.clone()/>
            <meta name="description" content=description.clone()/>
            <meta property="og:description" content=description.clone()/>
            <meta
                property="og:site_name"
                content=format!("Blog de Rust Lang en Español {}", get_year())
            />
            <meta property="og:url" content="https://rustlang-es.org"/>

            {if is_home {
                view! {
                    <>
                        <link rel="canonical" href="https://blog.rustlang-es.org"/>
                        <meta
                            property="og:image"
                            content=format!("https://rustlang-es.org/{slug}")
                        />
                        <meta
                            property="twitter:image"
                            content=format!("https://rustlang-es.org/{slug}")
                        />
                    </>
                }
            } else {
                view! {
                    <>
                        <link rel="canonical" href=format!("https://rustlang-es.org/{slug}")/>
                        <meta
                            property="og:image"
                            content=format!("https://rustlang-es.org/{slug}.png")
                        />
                        <meta
                            property="twitter:image"
                            content=format!("https://rustlang-es.org/{slug}.png")
                        />
                    </>
                }
            }}

            <meta name="twitter:card" content="summary_large_image"/>
            <meta name="twitter:site" content="@rustlang"/>
            <link rel="icon" href="/LogoSegunMichael-134de58fcd9af94e.ico"/>
            {if cfg!(debug_assertions) {
                view! { <link rel="stylesheet" href="/style.css"/> }
            } else {
                view! { <link rel="stylesheet" href="/style.css"/> }
                // view! { <link rel="stylesheet" href="https://blog.rustlang-es.org/output.css"/> }
            }}

            <style>
                {"
                body {
                margin: 0 auto;
                }
                "}
            </style>
            <script type="module">
                {"
                const API = 'https://rust-lang-en-espanol-api.shuttleapp.rs';
                const previous_domain = document.referrer || 'Undefined';
                if (previous_domain != 'Undefined') { previous_domain = new URL(previous_domain).host; }
                const urlParams = new URLSearchParams(window.location.search);
                const fromParam = urlParams.get('from');
                if (fromParam != null) previous_domain = fromParam;
                await fetch(API + '/track/count?reference=' + previous_domain, { method: 'POST' });
                "}
            </script>
        </Head>
        <Header title={title_clone} description={description_clone} />
        // Async is a component from the async_component module.
        // It will wrap an async function that returns an IntoView.
        <section class="w-full flex flex-col">

            // <Async view=navigation_bar />
            {if wide {
                view!{
                    <main class="container mx-auto">{children()}</main>
                }
            }else {
                view!{
                    <main class="">{children()}</main>
                }
            }}
        </section>
    }
}

#[component]
pub fn Header(#[prop(into)] title: String, #[prop(into)] description: String) -> impl IntoView {
    view! {
        <div>
            <nav class="sticky top-0 z-10 flex shadow-md shadow-black p-4 min-h-8 bg-gray-600">
                <div class="flex flex-wrap justify-between w-full">
                    <div class="items-center flex flex-1 min-w-0">
                        <button aria-label="Alternar barra lateral" aria-expanded="false" class="hidden mr-2" type="button">
                            <svg width="30" height="30" viewBox="0 0 30 30" aria-hidden="true"><path stroke="currentColor" stroke-linecap="round" stroke-miterlimit="10" stroke-width="2" d="M4 7h22M4 15h22M4 23h22"></path></svg>
                        </button>
                        <a class="items-center flex mr-4 min-w-0" href="/rust_book_es/">
                            <div class="flex-grow-0 shrink-0 basis-auto h-8 mr-2">
                                <img src="https://jalejotorresm.github.io/rust_book_es/img/ferris.png" alt="My Site Logo" class="max-h-full" />
                            </div>
                            <div>
                                <b class="flex-1 basis-auto overflow-hidden text-ellipsis whitespace-nowrap">{title}</b>
                                <p class="text-xl mb-2">{description}</p>
                            </div>
                        </a>
                    </div>
                    <div class="items-center flex flex-1 min-w-0 flex-shrink-0 flex-grow-0 basis-auto justify-end">
                        <a href="https://github.com/jalejotorresm/rust_book_es" target="_blank" rel="noopener noreferrer" class="inline-block p-3">
                            GitHub
                            <svg width="13.5" height="13.5" aria-hidden="true" viewBox="0 0 24 24" class="ml-1"><path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"></path></svg>
                        </a>
                        <div class="h-8 w-8">
                            <button class="items-center rounded-[50%] flex h-full justify-center w-full" type="button" title="Cambiar entre modo oscuro y claro (actualmente modo claro)" aria-label="Cambiar entre modo oscuro y claro (actualmente modo claro)" aria-live="polite">
                                <svg viewBox="0 0 24 24" width="24" height="24" class="hidden"><path fill="currentColor" d="M12,9c1.65,0,3,1.35,3,3s-1.35,3-3,3s-3-1.35-3-3S10.35,9,12,9 M12,7c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5 S14.76,7,12,7L12,7z M2,13l2,0c0.55,0,1-0.45,1-1s-0.45-1-1-1l-2,0c-0.55,0-1,0.45-1,1S1.45,13,2,13z M20,13l2,0c0.55,0,1-0.45,1-1 s-0.45-1-1-1l-2,0c-0.55,0-1,0.45-1,1S19.45,13,20,13z M11,2v2c0,0.55,0.45,1,1,1s1-0.45,1-1V2c0-0.55-0.45-1-1-1S11,1.45,11,2z M11,20v2c0,0.55,0.45,1,1,1s1-0.45,1-1v-2c0-0.55-0.45-1-1-1C11.45,19,11,19.45,11,20z M5.99,4.58c-0.39-0.39-1.03-0.39-1.41,0 c-0.39,0.39-0.39,1.03,0,1.41l1.06,1.06c0.39,0.39,1.03,0.39,1.41,0s0.39-1.03,0-1.41L5.99,4.58z M18.36,16.95 c-0.39-0.39-1.03-0.39-1.41,0c-0.39,0.39-0.39,1.03,0,1.41l1.06,1.06c0.39,0.39,1.03,0.39,1.41,0c0.39-0.39,0.39-1.03,0-1.41 L18.36,16.95z M19.42,5.99c0.39-0.39,0.39-1.03,0-1.41c-0.39-0.39-1.03-0.39-1.41,0l-1.06,1.06c-0.39,0.39-0.39,1.03,0,1.41 s1.03,0.39,1.41,0L19.42,5.99z M7.05,18.36c0.39-0.39,0.39-1.03,0-1.41c-0.39-0.39-1.03-0.39-1.41,0l-1.06,1.06 c-0.39,0.39-0.39,1.03,0,1.41s1.03,0.39,1.41,0L7.05,18.36z"></path></svg><svg viewBox="0 0 24 24" width="24" height="24" class="darkToggleIcon_wfgR"><path fill="currentColor" d="M9.37,5.51C9.19,6.15,9.1,6.82,9.1,7.5c0,4.08,3.32,7.4,7.4,7.4c0.68,0,1.35-0.09,1.99-0.27C17.45,17.19,14.93,19,12,19 c-3.86,0-7-3.14-7-7C5,9.07,6.81,6.55,9.37,5.51z M12,3c-4.97,0-9,4.03-9,9s4.03,9,9,9s9-4.03,9-9c0-0.46-0.04-0.92-0.1-1.36 c-0.98,1.37-2.58,2.26-4.4,2.26c-2.98,0-5.4-2.42-5.4-5.4c0-1.81,0.89-3.42,2.26-4.4C12.92,3.04,12.46,3,12,3L12,3z"></path></svg>
                            </button>
                        </div>
                        <div class="hidden"></div>
                    </div>
                </div>
                <div role="presentation" class="navbar-sidebar__backdrop"></div>
            </nav>
        </div>
    }
}

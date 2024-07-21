use std::collections::HashMap;

use leptos::{component, document, html::{Div, ToHtmlElement}, view, Fragment, HtmlElement, IntoView, View};

#[component]
pub fn CustomComponent(
    #[prop(into)] mut content: String,
    #[prop(into)] props: HashMap<String, String>
) -> impl IntoView {
    for (key, value) in props {
        content = content.replace(&format!("{{{key}}}"), &value);
    }

    // let algo = view!{<div inner_html=content/>};

    // let algo2 = document().create_element("template").unwrap();

    // algo2.set_inner_html(&content.clone());
    let algo = view!{ <template inner_html=content /> };
    
    // algo.child_nodes().;

    // let template = document().create_element("template").unwrap();
    // template.set_inner_html("<template>Just doing the Lord's work.</template>");
    if true {
        println!("algo");
    }


    // let algo2 = view!{<></>};

    // algo2.nodes;

    // let fragment = Fragment::new(algo.child_nodes())
    // let algo = view!{ <div inner_html=content /> };
    // let fragment = Fragment::new(algo.child_nodes())
    // view!{<>
    //         {algo.children()}
    //     </>
    // }
    
}
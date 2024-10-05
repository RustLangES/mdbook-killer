mod utils;
use dashmap::mapref::one::RefMut;
use utils::ToHtmlBuffer;

use std::fmt::Write;

use markdown::mdast as ast;

pub fn to_html(node: ast::Node) -> String {
    let mut buffer = ToHtmlBuffer::new(true);

    node_to_html(&mut buffer, node);

    buffer.buffer
}

fn node_to_html(buffer: &mut ToHtmlBuffer, node: ast::Node) {
    match node {
        ast::Node::Blockquote(node) => blockquote_to_html(buffer, node),
        ast::Node::Break(node) => break_to_html(buffer, node),
        ast::Node::Code(node) => code_to_html(buffer, node),
        ast::Node::Definition(node) => definition_to_html(buffer, node),
        ast::Node::Delete(node) => delete_to_html(buffer, node),
        ast::Node::Emphasis(node) => emphasis_to_html(buffer, node),
        ast::Node::FootnoteDefinition(node) => footnote_definition_to_html(buffer, node),
        ast::Node::FootnoteReference(node) => footnote_reference_to_html(buffer, node),
        ast::Node::Heading(node) => heading_to_html(buffer, node),
        ast::Node::Html(node) => html_to_html(buffer, node),
        ast::Node::Image(node) => image_to_html(buffer, node),
        ast::Node::ImageReference(node) => image_reference_to_html(buffer, node),
        ast::Node::InlineCode(node) => inline_code_to_html(buffer, node),
        ast::Node::InlineMath(node) => inline_math_to_html(buffer, node),
        ast::Node::Link(node) => link_to_html(buffer, node),
        ast::Node::LinkReference(node) => link_reference_to_html(buffer, node),
        ast::Node::List(node) => list_to_html(buffer, node),
        ast::Node::ListItem(node) => list_item_to_html(buffer, node),
        ast::Node::Math(node) => math_to_html(buffer, node),
        ast::Node::MdxFlowExpression(node) => mdx_flow_expression_to_html(buffer, node),
        ast::Node::MdxJsxFlowElement(node) => mdx_jsx_flow_element_to_html(buffer, node),
        ast::Node::MdxJsxTextElement(node) => mdx_jsx_text_element_to_html(buffer, node),
        ast::Node::MdxTextExpression(node) => mdx_text_expression_to_html(buffer, node),
        ast::Node::MdxjsEsm(node) => mdxjs_esm_to_html(buffer, node),
        ast::Node::Paragraph(node) => paragraph_to_html(buffer, node),
        ast::Node::Root(node) => root_to_html(buffer, node),
        ast::Node::Strong(node) => strong_to_html(buffer, node),
        ast::Node::Table(node) => table_to_html(buffer, node),
        ast::Node::TableCell(node) => table_cell_to_html(buffer, node),
        ast::Node::TableRow(node) => table_row_to_html(buffer, node),
        ast::Node::Text(node) => text_to_html(buffer, node),
        ast::Node::ThematicBreak(node) => thematic_break_to_html(buffer, node),
        ast::Node::Toml(node) => toml_to_html(buffer, node),
        ast::Node::Yaml(node) => yaml_to_html(buffer, node),
    }
}

fn children_to_html(buffer: &mut ToHtmlBuffer, children: Vec<ast::Node>) {
    children.into_iter().for_each(|c| node_to_html(buffer, c));
}

fn blockquote_to_html(buffer: &mut ToHtmlBuffer, node: ast::Blockquote) {
    let tag = buffer.tag("blockquote", "");
    children_to_html(tag.buffer, node.children);
}

fn break_to_html(_buffer: &mut ToHtmlBuffer, node: ast::Break) {
    todo!("{node:#?}")
}

fn code_to_html(buffer: &mut ToHtmlBuffer, node: ast::Code) {
    let attrs = format!(r#"class="language-{}""#, node.lang.unwrap_or(String::new()));
    let tag = buffer.tag("code", &attrs);
    _ = tag.buffer.write_str(&node.value);
}

fn definition_to_html(_buffer: &mut ToHtmlBuffer, node: ast::Definition) {
    todo!("{node:#?}")
}

fn delete_to_html(buffer: &mut ToHtmlBuffer, node: ast::Delete) {
    let tag = buffer.tag("s", "");
    children_to_html(tag.buffer, node.children)
}

fn emphasis_to_html(buffer: &mut ToHtmlBuffer, node: ast::Emphasis) {
    let tag = buffer.tag("i", "");
    children_to_html(tag.buffer, node.children);
}

fn footnote_definition_to_html(_buffer: &mut ToHtmlBuffer, node: ast::FootnoteDefinition) {
    todo!("{node:#?}");
}

fn footnote_reference_to_html(_buffer: &mut ToHtmlBuffer, node: ast::FootnoteReference) {
    todo!("{node:#?}")
}

fn heading_to_html(buffer: &mut ToHtmlBuffer, node: ast::Heading) {
    let tag = format!("h{}", node.depth);
    let tag = buffer.tag(&tag, "");
    children_to_html(tag.buffer, node.children);
}

fn html_to_html(_buffer: &mut ToHtmlBuffer, node: ast::Html) {
    todo!("{node:#?}")
}

fn image_reference_to_html(_buffer: &mut ToHtmlBuffer, node: ast::ImageReference) {
    todo!("{node:#?}")
}

fn image_to_html(buffer: &mut ToHtmlBuffer, node: ast::Image) {
    _ = write!(
        buffer,
        r#"<img src="{}" alt="{}" title="{}" />"#,
        node.url,
        node.alt,
        node.title.unwrap_or(String::new())
    );
}

fn inline_code_to_html(buffer: &mut ToHtmlBuffer, node: ast::InlineCode) {
    let tag = buffer.tag("code", "");
    _ = tag.buffer.write_str(&node.value);
}

fn inline_math_to_html(_buffer: &mut ToHtmlBuffer, node: ast::InlineMath) {
    todo!("{node:#?}")
}

fn link_reference_to_html(_buffer: &mut ToHtmlBuffer, node: ast::LinkReference) {
    todo!("{node:#?}")
}

fn link_to_html(buffer: &mut ToHtmlBuffer, node: ast::Link) {
    let attrs = format!(
        "href=\"{}\" title=\"{}\"",
        node.url,
        node.title.unwrap_or(String::new())
    );
    let tag = buffer.tag("a", &attrs);
    children_to_html(tag.buffer, node.children);
}

fn list_item_to_html(buffer: &mut ToHtmlBuffer, node: ast::ListItem) {
    let tag = buffer.tag("li", "");
    children_to_html(tag.buffer, node.children);
}

fn list_to_html(buffer: &mut ToHtmlBuffer, node: ast::List) {
    let tag = buffer.tag(if node.ordered { "ol" } else { "ul" }, "");
    children_to_html(tag.buffer, node.children);
}

fn math_to_html(_buffer: &mut ToHtmlBuffer, node: ast::Math) {
    todo!("{node:#?}")
}

fn mdx_flow_expression_to_html(_buffer: &mut ToHtmlBuffer, node: ast::MdxFlowExpression) {
    todo!("{node:#?}")
}

fn mdx_jsx_flow_element_to_html(buffer: &mut ToHtmlBuffer, node: ast::MdxJsxFlowElement) {
    let Some(name) = &node.name else {
        log::error!("Fragments are not supported");
        return;
    };

    let attrs = {
        // TODO: attrs
    };

    let id = {
        let mut id_ref = buffer.widgets.get_mut(name).unwrap_or_else(|| {
            buffer.widgets.insert(name.clone(), 0);
            buffer.widgets.get_mut(name).unwrap()
        });

        let id = id_ref.value().clone();

        *id_ref += 1;

        id
    };

    let attrs = format!(r#"style="display: contents;" id="widget-{name}-{id}""#);
    let tag = buffer.tag("div", &attrs);

    children_to_html(tag.buffer, node.children);
}

fn mdx_jsx_text_element_to_html(_buffer: &mut ToHtmlBuffer, node: ast::MdxJsxTextElement) {
    todo!("{node:#?}")
}

fn mdx_text_expression_to_html(_buffer: &mut ToHtmlBuffer, node: ast::MdxTextExpression) {
    todo!("{node:#?}")
}

fn mdxjs_esm_to_html(_buffer: &mut ToHtmlBuffer, node: ast::MdxjsEsm) {
    todo!("{node:#?}")
}

fn paragraph_to_html(buffer: &mut ToHtmlBuffer, node: ast::Paragraph) {
    let tag = buffer.tag("p", "");
    children_to_html(tag.buffer, node.children);
}

fn root_to_html(buffer: &mut ToHtmlBuffer, node: ast::Root) {
    children_to_html(buffer, node.children)
}

fn strong_to_html(buffer: &mut ToHtmlBuffer, node: ast::Strong) {
    let tag = buffer.tag("strong", "");
    children_to_html(tag.buffer, node.children);
}

fn table_cell_to_html(buffer: &mut ToHtmlBuffer, node: ast::TableCell) {
    let tag = buffer.tag("td", "");
    children_to_html(tag.buffer, node.children);
}

fn table_row_to_html(buffer: &mut ToHtmlBuffer, node: ast::TableRow) {
    let tag = buffer.tag("tr", "");
    children_to_html(tag.buffer, node.children);
}

fn table_to_html(buffer: &mut ToHtmlBuffer, node: ast::Table) {
    _ = buffer.tag("table", "");
    if node.children.len() < 2 {
        log::error!("Tables should have at least one row");
        return;
    }
    let mut head = node.children;
    let rows = head.split_off(1);
    {
        let head = head.into_iter().next().unwrap();
        let head_row = if let ast::Node::TableRow(row) = head {
            row
        } else {
            unreachable!();
        };
        _ = buffer.tag("thead", "");
        _ = buffer.tag("tr", "");
        for cell in head_row.children {
            if let ast::Node::TableCell(cell) = cell {
                let tag = buffer.tag("th", "");
                children_to_html(tag.buffer, cell.children);
            } else {
                unreachable!();
            }
        }
    }
    {
        let tag = buffer.tag("tbody", "");
        children_to_html(tag.buffer, rows);
    }
}

fn text_to_html(buffer: &mut ToHtmlBuffer, node: ast::Text) {
    _ = buffer.write_str(&node.value);
}

fn thematic_break_to_html(buffer: &mut ToHtmlBuffer, _node: ast::ThematicBreak) {
    _ = buffer.write_str("<hr/>");
    buffer.push_newline();
}

fn toml_to_html(_buffer: &mut ToHtmlBuffer, node: ast::Toml) {
    todo!("{node:#?}")
}

fn yaml_to_html(_buffer: &mut ToHtmlBuffer, node: ast::Yaml) {
    todo!("{node:#?}")
}

use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use tokio::fs;

#[derive(Clone, Debug)]
pub struct TreeNode {
    pub title: String,
    pub href: Option<String>,

    pub children: Vec<TreeNode>,
}

#[derive(Clone, Debug)]
pub struct Summary {
    pub path: PathBuf,
    pub all_files: HashSet<PathBuf>,
    pub root: Vec<TreeNode>,
    pub list: Vec<TreeNode>,
}

impl Summary {
    pub async fn from_path(path: PathBuf) -> Result<Self> {
        let raw = fs::read_to_string(&path)
            .await
            .map_err(|err| anyhow!("Cannot read {path:?}.\n  Cause: {err}"))?;
        let raw = markdown::to_mdast(&raw, &markdown::ParseOptions::mdx())
            .map_err(|err| anyhow!("ParseError {err}"))?;

        let mut all_files = HashSet::new();

        let Some((root, list)) = md_to_tree(&raw, &mut all_files) else {
            return Err(anyhow!("Cannot parse SUMMARY.md"));
        };

        Ok(Self {
            path,
            all_files,
            root,
            list,
        })
    }
}

fn md_to_tree(
    node: &markdown::mdast::Node,
    all_files: &mut HashSet<PathBuf>,
) -> Option<(Vec<TreeNode>, Vec<TreeNode>)> {
    use markdown::mdast as ast;

    match node {
        ast::Node::Root(ast::Root { children, .. }) => {
            let mut root = vec![];
            let mut list = vec![];

            for child in children {
                match child {
                    ast::Node::Paragraph(ast::Paragraph { children, .. }) => {
                        for child in children {
                            if let Some(a) = md_paragraph_node_to_tree(child, all_files) {
                                root.push(a);
                            }
                        }
                    }

                    ast::Node::List(ast::List { children, .. }) => {
                        list = md_list_to_tree(children, all_files)
                    }

                    e => {
                        log::warn!("Unexpected: {e:#?}");
                    }
                }
            }

            Some((root, list))
        }
        _ => None,
    }
}

fn md_paragraph_node_to_tree(
    node: &markdown::mdast::Node,
    all_files: &mut HashSet<PathBuf>,
) -> Option<TreeNode> {
    use markdown::mdast as ast;

    match node {
        ast::Node::Text(ast::Text { value: title, .. }) if !title.trim().is_empty() => {
            Some(TreeNode {
                title: title.clone(),
                href: None,

                children: Vec::new(),
            })
        }
        ast::Node::Link(ast::Link {
            url,
            children,
            position,
            ..
        }) => match children.get(0) {
            Some(ast::Node::Text(ast::Text { value: title, .. })) => {
                if !url.starts_with("http") {
                    all_files.insert(
                        PathBuf::from_str(url).expect(&format!("Cannot parse path {url:#?}")),
                    );
                }

                Some(TreeNode {
                    title: title.clone(),
                    href: Some(url.clone()),

                    children: Vec::new(),
                })
            }
            Some(_) => {
                if let Some(position) = position {
                    let line = position.start.line;
                    let column = position.start.column;

                    log::warn!("Link (at {line}:{column}) has no text child");
                } else {
                    log::warn!("Link (at <unknown>) has no text child");
                }

                None
            }
            None => {
                if let Some(position) = position {
                    let line = position.start.line;
                    let column = position.start.column;

                    log::warn!("Link (at {line}:{column}) has no children");
                } else {
                    log::warn!("Link (at <unknown>) has no children");
                }

                None
            }
        },
        _ => None,
    }
}

fn md_list_to_tree(
    node: &Vec<markdown::mdast::Node>,
    all_files: &mut HashSet<PathBuf>,
) -> Vec<TreeNode> {
    use markdown::mdast as ast;

    node.iter()
        .filter_map(|node| match node {
            ast::Node::ListItem(ast::ListItem { children, .. }) => {
                let mut root = match children.get(0) {
                    Some(ast::Node::Paragraph(ast::Paragraph { children, .. })) => {
                        md_paragraph_node_to_tree(children.get(0)?, all_files)?
                    }

                    Some(_) => {
                        log::warn!("A list item should have a Link or Text as first child");
                        return None;
                    }

                    None => unreachable!(),
                };

                let list = match children.get(1) {
                    Some(ast::Node::List(ast::List { children, .. })) => {
                        md_list_to_tree(children, all_files)
                    }

                    Some(_) => {
                        log::warn!("A list item may have a List as second child");
                        return None;
                    }

                    None => vec![],
                };

                root.children = list;

                Some(root)
            }
            _ => {
                log::warn!("A list must have ListItem as children");
                None
            }
        })
        .collect()
}

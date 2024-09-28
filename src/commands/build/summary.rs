use std::collections::HashSet;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use tokio::fs;

#[derive(Debug, thiserror::Error)]
pub enum SummaryError {
    #[error("{0:?} not found")]
    NotFound(PathBuf),
    #[error("Cannot open summary: {0}")]
    IO(#[from] io::Error),
    #[error("Cannot open {0}: {1}")]
    HandledIo(PathBuf, io::Error),
    #[error("Cannot parse summary: {0}")]
    Parse(markdown::message::Message),
    #[error("Summary has no root. Open an issue with all the context")]
    NoRoot,
}

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
    pub async fn from_path(path: PathBuf) -> Result<Self, SummaryError> {
        let raw = match fs::read_to_string(&path).await {
            Ok(ok) => ok,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                return Err(SummaryError::NotFound(path));
            }
            Err(err) => return Err(SummaryError::HandledIo(path, err)),
        };

        let raw = markdown::to_mdast(&raw, &markdown::ParseOptions::mdx())
            .map_err(|err| SummaryError::Parse(err))?;

        let mut all_files = HashSet::new();

        let (root, list) = md_to_tree(&raw, &mut all_files)?;

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
) -> Result<(Vec<TreeNode>, Vec<TreeNode>), SummaryError> {
    use markdown::mdast as ast;

    match node {
        ast::Node::Root(ast::Root { children, .. }) => {
            let mut root = vec![];
            let mut list = vec![];

            for child in children {
                match child {
                    ast::Node::Paragraph(ast::Paragraph { children, .. }) => children
                        .into_iter()
                        .filter_map(|c| md_paragraph_node_to_tree(c, all_files))
                        .for_each(|c| root.push(c)),

                    ast::Node::List(ast::List { children, .. }) => {
                        list = md_list_to_tree(children, all_files)
                    }

                    e => {
                        log::warn!("Unexpected: {e:#?}");
                    }
                }
            }

            Ok((root, list))
        }
        _ => Err(SummaryError::NoRoot),
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
                // Link can be to external sites, so we check it before add
                // to files list
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

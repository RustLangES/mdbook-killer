use std::collections::HashSet;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use markdown::mdast as ast;
use tokio::fs;

use crate::utils::SafeRemove;

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
    pub dir: PathBuf,
    pub list: Vec<TreeNode>,
    pub root: Vec<TreeNode>,
}

#[derive(Clone, Debug)]
pub struct SummaryParser<'a> {
    pub all_files: HashSet<PathBuf>,
    pub src_path: &'a PathBuf,
    summary_dir: Option<PathBuf>,
}

impl<'a> SummaryParser<'a> {
    pub fn new(src_path: &'a PathBuf) -> Self {
        Self {
            src_path,
            all_files: HashSet::new(),
            summary_dir: None,
        }
    }
}

impl<'a> SummaryParser<'a> {
    pub async fn parse_dir(&mut self, dir: &PathBuf) -> Result<Summary, SummaryError> {
        let sumary_path = dir.join("SUMMARY.md");

        let raw = match fs::read_to_string(&sumary_path).await {
            Ok(ok) => ok,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                return Err(SummaryError::NotFound(sumary_path));
            }
            Err(err) => return Err(SummaryError::HandledIo(sumary_path, err)),
        };

        let raw = markdown::to_mdast(&raw, &markdown::ParseOptions::mdx())
            .map_err(|err| SummaryError::Parse(err))?;

        self.summary_dir = Some(dir.to_path_buf());
        let (root, list) = self.parse(raw)?;
        let dir = self.summary_dir.take().unwrap();

        Ok(Summary { dir, list, root })
    }

    fn parse(&mut self, node: ast::Node) -> Result<(Vec<TreeNode>, Vec<TreeNode>), SummaryError> {
        match node {
            ast::Node::Root(root) => self.visit_root(root),
            _ => Err(SummaryError::NoRoot),
        }
    }

    fn visit_root(
        &mut self,
        root: ast::Root,
    ) -> Result<(Vec<TreeNode>, Vec<TreeNode>), SummaryError> {
        let mut root_list = vec![];
        let mut list = vec![];

        for child in root.children {
            match child {
                ast::Node::Paragraph(paragraph) => self
                    .visit_paragraph(paragraph)
                    .into_iter()
                    .for_each(|child| root_list.push(child)),

                ast::Node::List(list_node) => list = self.visit_list(list_node),

                e => {
                    log::warn!("Unexpected node: {e:#?}");
                }
            }
        }

        Ok((root_list, list))
    }

    fn visit_paragraph(&mut self, paragraph: ast::Paragraph) -> Vec<TreeNode> {
        paragraph
            .children
            .into_iter()
            .filter_map(|node| self.visit_paragraph_child(node))
            .collect()
    }

    fn visit_paragraph_child(&mut self, node: ast::Node) -> Option<TreeNode> {
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
                        let path = if url.starts_with("/") {
                            self.src_path.join(&url[1..])
                        } else {
                            self.summary_dir().join(&url)
                        };

                        self.all_files.insert(path);
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

    fn visit_list(&mut self, list: ast::List) -> Vec<TreeNode> {
        list.children
            .into_iter()
            .filter_map(|node| {
                if let ast::Node::ListItem(list_item) = node {
                    self.visit_list_item(list_item)
                } else {
                    log::warn!("A list must have ListItem as children");
                    None
                }
            })
            .collect()
    }

    fn visit_list_item(&mut self, mut list_item: ast::ListItem) -> Option<TreeNode> {
        // Get the second child first due to `swap_remove` behaviour
        // that keeps O(1) but break the order of the array
        let second_child = list_item.children.safe_remove(1);
        let first_child = list_item.children.safe_remove(0);

        let mut root = if let Some(ast::Node::Paragraph(mut paragraph)) = first_child {
            paragraph
                .children
                .safe_remove(0)
                .map(|first_child| self.visit_paragraph_child(first_child))
                .flatten()?
        } else {
            log::warn!("A list item should have a Link or Text as first child");
            return None;
        };

        root.children = second_child.map_or_else(
            || vec![],
            |child| {
                if let ast::Node::List(list) = child {
                    self.visit_list(list)
                } else {
                    log::warn!("A list item may have a List as second child");
                    vec![]
                }
            },
        );

        Some(root)
    }

    fn summary_dir(&mut self) -> &PathBuf {
        self.summary_dir
            .as_ref()
            .expect("Summary dir is setted before parse")
    }
}

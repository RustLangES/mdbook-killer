
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Configuration for localizations of this book
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreprocessorsConfig {
    pub html: HtmlPreprocessor,
    #[serde(flatten)]
    pub others: HashMap<String, Preprocessor>,
}

///
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Preprocessor {
    ///
    pub after: Option<Vec<String>>,
    ///
    pub before: Option<Vec<String>>,
    ///
    pub command: Option<String>,
    ///
    pub renders: Option<Vec<String>>,
    pub optional: Option<bool>,
}

///
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct HtmlPreprocessor {
    pub theme: Option<String>,
    pub default_theme: Option<String>,
    pub preferred_dark_theme: Option<String>,
    pub curly_quotes: Option<bool>,
    pub mathjax_support: Option<bool>,
    pub copy_fonts: Option<bool>,
    pub additional_css: Option<Vec<String>>,
    pub additional_js: Option<Vec<String>>,
    pub no_section_label: Option<bool>,
    pub git_repository_url: Option<String>,
    pub git_repository_icon: Option<String>,
    pub edit_url_template: Option<String>,
    pub site_url: Option<String>,
    pub cname: Option<String>,
    pub input_404: Option<String>,
    pub print: Option<HtmlPrint>,
    pub fold: Option<HtmlFold>,
    pub playground: Option<HtmlPlayground>,
    pub code: Option<HtmlCode>,
    pub search: Option<HtmlSearch>,
    pub redirect: Option<HtmlRedirect>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct HtmlPrint {
    /// include support for printable output
    enable: bool,
    /// insert page-break after each chapter
    page_break: Option<bool>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct HtmlFold {
    /// whether or not to enable section folding
    enable: bool,
    /// the depth to start folding
    level: Option<u16>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct HtmlPlayground {
    ///
    editable: Option<bool>,
    ///
    copyable: Option<bool>,
    ///
    copy_js: Option<bool>,
    ///
    runnable: Option<bool>,
    ///
    line_numbers: Option<bool>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct HtmlCode {
    ///
    hidelines: Option<HashMap<String, String>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct HtmlSearch {
    pub enable: bool,
    pub limit_results: Option<i64>,
    pub teaser_word_count: Option<i64>,
    pub use_boolean_and: Option<bool>,
    pub boost_title: Option<i64>,
    pub boost_hierarchy: Option<i64>,
    pub boost_paragraph: Option<i64>,
    pub expand: Option<bool>,
    pub heading_split_level: Option<i64>,
    pub copy_js: Option<bool>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct HtmlRedirect {
    ///
    redirect: HashMap<String, String>,
}

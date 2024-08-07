mod tokenizer;
use tantivy::{
    query::{BooleanQuery, ConstScoreQuery, Occur, Query, TermQuery},
    schema::IndexRecordOption,
    Term,
};
pub use tokenizer::tokenize_code;

use super::{corpus, IndexSchema};
use crate::api::code::CodeSearchQuery;

pub mod fields {
    pub const CHUNK_GIT_URL: &str = "chunk_git_url";
    pub const CHUNK_FILEPATH: &str = "chunk_filepath";
    pub const CHUNK_LANGUAGE: &str = "chunk_language";
    pub const CHUNK_BODY: &str = "chunk_body";
    pub const CHUNK_START_LINE: &str = "chunk_start_line";
}

/// 创建一个用于查询编程语言的终端查询对象。
///
/// 该函数创建一个终端查询，该查询在`schema.field_chunk_attributes`字段上搜索给定的`language`。
///
/// 如果输入的语言是JavaScript或TypeScript相关的语言，则会被映射为"javascript-typescript"。
///
/// # 参数
///
/// * `language` - 要搜索的编程语言。
///
/// # 返回值
///
/// * `Box<TermQuery>` - 一个包含在`schema.field_chunk_attributes`字段上搜索给定`language`的终端查询对象。
fn language_query(language: &str) -> Box<TermQuery> {
    let schema = IndexSchema::instance();
    let language = match language {
        "javascript" | "typescript" | "javascriptreact" | "typescriptreact" => {
            "javascript-typescript"
        }
        _ => language,
    };

    let mut term =
        Term::from_field_json_path(schema.field_chunk_attributes, fields::CHUNK_LANGUAGE, false);
    term.append_type_and_str(language);
    Box::new(TermQuery::new(term, IndexRecordOption::Basic))
}

/// 创建一个用于查询代码片段主体的查询对象。
///
/// 该函数创建一个布尔查询，该查询包含多个终端查询，
/// 每个终端查询都是对`schema.field_chunk_tokens`字段上的一个单词进行查询。
///
/// # 参数
///
/// * `tokens` - 代码片段主体的单词列表。
///
/// # 返回值
///
/// * `Box<dyn Query>` - 一个包含多个终端查询的布尔查询对象，用于查询代码片段主体。
pub fn body_query(tokens: &[String]) -> Box<dyn Query> {
    let schema = IndexSchema::instance();
    let subqueries: Vec<Box<dyn Query>> = tokens
        .iter()
        .map(|text| {
            let term = Term::from_field_text(schema.field_chunk_tokens, text);
            let term_query: Box<dyn Query> =
                Box::new(TermQuery::new(term, IndexRecordOption::Basic));

            term_query
        })
        .collect();

    Box::new(BooleanQuery::union(subqueries))
}

/// 创建一个用于查询Git URL的终端查询对象。
///
/// 该函数创建一个终端查询，该查询在`schema.field_chunk_attributes`字段上搜索给定的`git_url`。
///
/// # 参数
///
/// * `git_url` - 要搜索的Git URL。
///
/// # 返回值
///
/// * `Box<TermQuery>` - 一个包含在`schema.field_chunk_attributes`字段上搜索给定`git_url`的终端查询对象。
fn git_url_query(git_url: &str) -> Box<TermQuery> {
    let schema = IndexSchema::instance();
    let mut term =
        Term::from_field_json_path(schema.field_chunk_attributes, fields::CHUNK_GIT_URL, false);
    term.append_type_and_str(git_url);
    Box::new(TermQuery::new(term, IndexRecordOption::Basic))
}

fn filepath_query(filepath: &str) -> Box<TermQuery> {
    let schema = IndexSchema::instance();
    let mut term =
        Term::from_field_json_path(schema.field_chunk_attributes, fields::CHUNK_FILEPATH, false);
    term.append_type_and_str(filepath);
    Box::new(TermQuery::new(term, IndexRecordOption::Basic))
}

/// 创建一个代码搜索查询对象。
///
/// 该函数首先创建一个用于查询语料库的查询对象，然后创建一个用于查询Git URL的查询对象。
/// 然后，根据输入的`query`对象中的属性，创建相应的子查询并将其添加到布尔查询中。
///
/// 如果输入的`query`对象中包含语言属性，则会创建一个用于查询编程语言的查询对象并将其添加到布尔查询中。
///
/// 如果输入的`query`对象中包含文件路径属性，则会创建一个用于排除文件的查询对象并将其添加到布尔查询中。
///
/// # 参数
///
/// * `query` - 代码搜索查询对象。
/// * `chunk_tokens_query` - 用于查询代码片段的查询对象。
///
/// # 返回值
///
/// * `BooleanQuery` - 一个包含多个子查询的布尔查询对象。
pub fn code_search_query(
    query: &CodeSearchQuery,
    chunk_tokens_query: Box<dyn Query>,
) -> BooleanQuery {
    let schema = IndexSchema::instance();
    let corpus_query = schema.corpus_query(corpus::CODE);
    let git_url_query = git_url_query(&query.git_url);

    // language / git_url / filepath field shouldn't contribute to the score, mark them to 0.0.
    let mut subqueries: Vec<(Occur, Box<dyn Query>)> = vec![
        (
            Occur::Must,
            Box::new(ConstScoreQuery::new(corpus_query, 0.0)),
        ),
        (Occur::Must, Box::new(chunk_tokens_query)),
        (
            Occur::Must,
            Box::new(ConstScoreQuery::new(git_url_query, 0.0)),
        ),
    ];

    if let Some(language) = query.language.as_deref() {
        subqueries.push((
            Occur::Must,
            Box::new(ConstScoreQuery::new(language_query(language), 0.0)),
        ));
    }

    // When filepath presents, we exclude the file from the search.
    if let Some(filepath) = &query.filepath {
        subqueries.push((
            Occur::MustNot,
            Box::new(ConstScoreQuery::new(filepath_query(filepath), 0.0)),
        ))
    }

    BooleanQuery::new(subqueries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_query() {
        let lhs = language_query("javascript-typescript");
        assert_eq!(lhs.term(), language_query("javascript").term());
        assert_eq!(lhs.term(), language_query("typescript").term());
        assert_eq!(lhs.term(), language_query("typescriptreact").term());
        assert_eq!(lhs.term(), language_query("javascriptreact").term());
    }
}

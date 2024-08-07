pub mod code;
pub mod doc;

use std::borrow::Cow;

use lazy_static::lazy_static;
use tantivy::{
    query::{BooleanQuery, ConstScoreQuery, ExistsQuery, Occur, Query, RangeQuery, TermQuery},
    schema::{
        Field, IndexRecordOption, JsonObjectOptions, Schema, TextFieldIndexing, FAST, INDEXED,
        STORED, STRING,
    },
    DateTime, Term,
};

pub struct IndexSchema {
    pub schema: Schema,

    // === Fields for both document and chunk ===
    /// Corpus for the document, each corpus comes with its own schema for json fields (field_attributes / field_chunk_attributes)
    /// See ./doc or ./code as an example
    pub field_corpus: Field,

    /// Unique identifier (within corpus) for a group of documents.
    pub field_source_id: Field,

    /// Unique identifier (within corpus) for the document, each document could have multiple chunks indexed.
    pub field_id: Field,

    /// Last updated time for the document in index.
    pub field_updated_at: Field,
    // ==========================================

    // === Fields for document ===
    /// JSON attributes for the document, it's only stored but not indexed.
    pub field_attributes: Field,
    // ===========================

    // === Fields for chunk ===
    pub field_chunk_id: Field,
    /// JSON attributes for the chunk, it's indexed (thus can be used as filter in query) and stored.
    pub field_chunk_attributes: Field,
    /// Matching tokens for the chunk, it's indexed but not stored..
    pub field_chunk_tokens: Field,
    // =========================
}

const FIELD_CHUNK_ID: &str = "chunk_id";
const FIELD_UPDATED_AT: &str = "updated_at";

pub mod corpus {
    pub const CODE: &str = "code";
    pub const WEB: &str = "web";
}

impl IndexSchema {
    pub fn instance() -> &'static Self {
        &INDEX_SCHEMA
    }

    fn new() -> Self {
        let mut builder = Schema::builder();

        let field_corpus = builder.add_text_field("corpus", STRING | FAST);
        let field_source_id = builder.add_text_field("source_id", STRING | FAST);
        let field_id = builder.add_text_field("id", STRING | STORED);

        let field_updated_at = builder.add_date_field(FIELD_UPDATED_AT, INDEXED);
        let field_attributes = builder.add_text_field("attributes", STORED);

        let field_chunk_id = builder.add_text_field(FIELD_CHUNK_ID, STRING | FAST | STORED);
        let field_chunk_attributes = builder.add_json_field(
            "chunk_attributes",
            JsonObjectOptions::default()
                .set_stored()
                .set_indexing_options(
                    TextFieldIndexing::default()
                        .set_tokenizer("raw")
                        .set_fieldnorms(true)
                        .set_index_option(tantivy::schema::IndexRecordOption::Basic)
                        .set_fieldnorms(true),
                ),
        );

        let field_chunk_tokens = builder.add_text_field("chunk_tokens", STRING);
        let schema = builder.build();

        Self {
            schema,
            field_id,
            field_source_id,
            field_corpus,
            field_updated_at,
            field_attributes,

            field_chunk_id,
            field_chunk_attributes,
            field_chunk_tokens,
        }
    }

    pub fn source_query(&self, corpus: &str, source_id: &str) -> impl Query {
        let source_id_query = TermQuery::new(
            Term::from_field_text(self.field_source_id, source_id),
            tantivy::schema::IndexRecordOption::Basic,
        );

        BooleanQuery::new(vec![
            // Must match the corpus
            (Occur::Must, self.corpus_query(corpus)),
            // Must match the source id
            (Occur::Must, Box::new(source_id_query)),
        ])
    }

    /// Build a query to find the document with the given `doc_id`.
    pub fn doc_query(&self, corpus: &str, doc_id: &str) -> impl Query {
        let doc_id_query = TermQuery::new(
            Term::from_field_text(self.field_id, doc_id),
            tantivy::schema::IndexRecordOption::Basic,
        );

        BooleanQuery::new(vec![
            // Must match the corpus
            (Occur::Must, self.corpus_query(corpus)),
            // Must match the doc id
            (Occur::Must, Box::new(doc_id_query)),
            // Exclude chunk documents
            (
                Occur::MustNot,
                Box::new(ExistsQuery::new_exists_query(FIELD_CHUNK_ID.into())),
            ),
        ])
    }

    pub fn doc_indexed_after(
        &self,
        corpus: &str,
        doc_id: &str,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> impl Query {
        let doc_id_query = TermQuery::new(
            Term::from_field_text(self.field_id, doc_id),
            tantivy::schema::IndexRecordOption::Basic,
        );

        let updated_at = DateTime::from_timestamp_nanos(
            updated_at.timestamp_nanos_opt().expect("valid timestamp"),
        );

        BooleanQuery::new(vec![
            // Must match the corpus
            (Occur::Must, self.corpus_query(corpus)),
            // Must match the doc id
            (Occur::Must, Box::new(doc_id_query)),
            // Must match the updated_at
            (
                Occur::Must,
                Box::new(RangeQuery::new_date(
                    FIELD_UPDATED_AT.to_owned(),
                    updated_at..DateTime::MAX,
                )),
            ),
            // Exclude chunk documents
            (
                Occur::MustNot,
                Box::new(ExistsQuery::new_exists_query(FIELD_CHUNK_ID.into())),
            ),
        ])
    }

    /// Build a query to find the document with the given `doc_id`, include chunks.
    pub fn doc_query_with_chunks(&self, corpus: &str, doc_id: &str) -> impl Query {
        let doc_id_query = TermQuery::new(
            Term::from_field_text(self.field_id, doc_id),
            tantivy::schema::IndexRecordOption::Basic,
        );

        BooleanQuery::new(vec![
            // Must match the corpus
            (Occur::Must, self.corpus_query(corpus)),
            // Must match the doc id
            (Occur::Must, Box::new(doc_id_query)),
        ])
    }

    /// 创建一个用于查询语料库的查询对象。
    ///
    /// 该函数创建一个终端查询，该查询在`self.field_corpus`字段上搜索给定的`corpus`文本。
    ///
    /// # 参数
    ///
    /// * `corpus` - 要搜索的语料库文本。
    ///
    /// # 返回值
    ///
    /// * `Box<dyn Query>` - 一个包含在`self.field_corpus`字段上搜索给定`corpus`文本的查询对象。
    pub fn corpus_query(&self, corpus: &str) -> Box<dyn Query> {
        Box::new(TermQuery::new(
            Term::from_field_text(self.field_corpus, corpus),
            tantivy::schema::IndexRecordOption::Basic,
        ))
    }

    pub fn source_ids_query(&self, source_ids: &[String]) -> impl Query {
        BooleanQuery::new(
            source_ids
                .iter()
                .map(|source_id| -> (Occur, Box<(dyn Query)>) {
                    (
                        Occur::Should,
                        Box::new(TermQuery::new(
                            Term::from_field_text(self.field_source_id, source_id),
                            tantivy::schema::IndexRecordOption::Basic,
                        )),
                    )
                })
                .collect::<Vec<_>>(),
        )
    }
}

lazy_static! {
    static ref INDEX_SCHEMA: IndexSchema = IndexSchema::new();
}

/// 将嵌入向量二值化。
///
/// 该函数将`embedding`迭代器中的每个值转换为一个字符串，并根据值的大小来确定字符串的形式。
///
/// # 示例
///
/// ```
/// let embedding = vec![0.5, -0.3, 1.2, -0.8];
///
/// let binarized_embedding = binarize_embedding(embedding.iter());
///
/// let result: Vec<String> = binarized_embedding.collect();
///
/// assert_eq!(result, vec!["embedding_one_0", "embedding_zero_1", "embedding_one_2", "embedding_zero_3"]);
/// ```
///
/// # 参数
///
/// * `embedding` - 要二值化的嵌入向量迭代器。
///
/// # 返回值
///
/// * `impl Iterator<Item = String>` - 二值化后的嵌入向量迭代器，其中每个元素都是一个字符串。
pub fn binarize_embedding<'a>(
    embedding: impl Iterator<Item = &'a f32> + 'a,
) -> impl Iterator<Item = String> + 'a {
    embedding.enumerate().map(|(i, value)| {
        if *value <= 0.0 {
            format!("embedding_zero_{}", i)
        } else {
            format!("embedding_one_{}", i)
        }
    })
}

pub fn embedding_tokens_query<'a>(
    embedding_dims: usize,
    embedding: impl Iterator<Item = &'a f32> + 'a,
) -> BooleanQuery {
    let schema = IndexSchema::instance();
    let iter = binarize_embedding(embedding).map(Cow::Owned);
    new_multiterms_const_query(schema.field_chunk_tokens, embedding_dims, iter)
}

/// 创建一个具有固定分数的多术语布尔查询。
///
/// 该函数创建一个布尔查询，其中包含一个或多个子查询，每个子查询都是一个固定分数的终端查询。
///
/// # 示例
///
/// ```
/// let field = Field::new_text("title");
/// let embedding_dims = 100;
/// let terms = vec!["rust", "programming", "language"];
///
/// let query = new_multiterms_const_query(field, embedding_dims, terms.into_iter().map(Cow::from));
///
/// assert_eq!(query.to_string(), "title:rust title:programming title:language");
/// ```
///
/// # 参数
///
/// * `field` - 要在其上创建查询的字段。
/// * `embedding_dims` - 嵌入维度，用于计算每个子查询的固定分数。
/// * `terms` - 要在查询中包含的术语迭代器。
///
/// # 返回值
///
/// * `BooleanQuery` - 一个布尔查询，其中包含一个或多个具有固定分数的终端查询。
fn new_multiterms_const_query<'a>(
    field: Field,
    embedding_dims: usize,
    terms: impl Iterator<Item = Cow<'a, str>> + 'a,
) -> BooleanQuery {
    let subqueries: Vec<Box<dyn Query>> = terms
        .map(|text| {
            let term = Term::from_field_text(field, text.as_ref());
            let term_query: Box<dyn Query> =
                Box::new(TermQuery::new(term, IndexRecordOption::Basic));

            let score = 1.0 / embedding_dims as f32;
            let boxed: Box<dyn Query> = Box::new(ConstScoreQuery::new(term_query, score));

            boxed
        })
        .collect();

    BooleanQuery::union(subqueries)
}

#[cfg(test)]
mod tests {

    use tantivy::{
        collector::TopDocs,
        query::Query,
        schema::{Schema, STRING},
        Index, IndexWriter, TantivyDocument,
    };

    use super::*;

    #[test]
    fn test_new_multiterms_const_query() -> anyhow::Result<()> {
        let mut schema_builder = Schema::builder();
        let field1 = schema_builder.add_text_field("field1", STRING);
        let schema = schema_builder.build();
        let index = Index::create_in_ram(schema);
        {
            let mut index_writer: IndexWriter = index.writer(15_000_000)?;

            // doc1
            let mut doc = TantivyDocument::new();
            doc.add_text(field1, "value1");
            doc.add_text(field1, "value2");
            doc.add_text(field1, "value3");
            index_writer.add_document(doc)?;

            // doc2
            let mut doc = TantivyDocument::new();
            doc.add_text(field1, "value2");
            doc.add_text(field1, "value4");
            index_writer.add_document(doc)?;

            index_writer.commit()?;
        }
        let reader = index.reader()?;
        let searcher = reader.searcher();

        {
            let query = new_multiterms_const_query(
                field1,
                4,
                vec!["value1", "value3"].into_iter().map(Cow::Borrowed),
            );

            let top_docs = searcher.search(&query, &TopDocs::with_limit(1))?;
            eprintln!("explain {:?}", query.explain(&searcher, top_docs[0].1)?);

            assert_eq!(top_docs.len(), 1, "Expected 1 document");
            assert_eq!(top_docs[0].0, 0.5);
        }

        {
            let query = new_multiterms_const_query(
                field1,
                4,
                vec!["value1", "value2", "value3"]
                    .into_iter()
                    .map(Cow::Borrowed),
            );

            let top_docs = searcher.search(&query, &TopDocs::with_limit(1))?;

            assert_eq!(top_docs.len(), 1, "Expected 1 document");
            assert_eq!(top_docs[0].0, 0.75);
        }

        Ok(())
    }
}

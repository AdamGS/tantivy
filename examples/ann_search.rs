use tantivy::schema::{Schema, VectorFieldIndexing, VectorOptions, STORED, TEXT};
use tantivy::{Index, IndexWriter, TantivyDocument};
use tempfile::TempDir;

fn main() -> tantivy::Result<()> {
    let index_path = TempDir::new()?;

    let mut schema_builder = Schema::builder();
    let vector_field_options =
        VectorOptions::default().set_indexing_options(VectorFieldIndexing::from_dimension(768));
    schema_builder.add_vector_field("vector", vector_field_options);
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);

    let schema = schema_builder.build();

    let index = Index::create_in_dir(&index_path, schema.clone())?;
    let mut index_writer: IndexWriter = index.writer(50_000_000)?;

    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();
    let vector = schema.get_field("vector").unwrap();

    let mut old_man_doc = TantivyDocument::default();
    old_man_doc.add_text(title, "The Old Man and the Sea");
    old_man_doc.add_text(
        body,
        "He was an old man who fished alone in a skiff in the Gulf Stream and he had gone \
         eighty-four days now without taking a fish.",
    );
    old_man_doc.add_vector(vector, &[1.0_f32; 768]);
    index_writer.add_document(old_man_doc).unwrap();

    index_writer.commit().unwrap();

    Ok(())
}

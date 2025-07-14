//! Vector index

use std::collections::HashMap;

use crate::schema::{Field, FieldEntry, Schema};

pub(crate) struct PerFieldVectorWriter {
    per_field_vector_writers: Vec<VectorWriter>,
}

impl PerFieldVectorWriter {
    pub fn for_schema(schema: &Schema) -> Self {
        let per_field_vector_writers = schema
            .fields()
            .filter_map(|(_, field_entry)| vector_writer_from_field_entry(field_entry))
            .collect();
        PerFieldVectorWriter {
            per_field_vector_writers,
        }
    }

    pub(crate) fn get_for_field(&self, field: Field) -> &VectorWriter {
        &self.per_field_vector_writers[field.field_id() as usize]
    }

    pub(crate) fn get_for_field_mut(&mut self, field: Field) -> &mut VectorWriter {
        self.per_field_vector_writers
            .get_mut(field.field_id() as usize)
            .expect("Field doesn't have vector writer")
    }
}

pub(crate) struct VectorWriter {
    pub dimension: u32,
}

fn vector_writer_from_field_entry(field_entry: &FieldEntry) -> Option<VectorWriter> {
    if !field_entry.field_type().is_vector() {
        return None;
    }

    todo!()
}

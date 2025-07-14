use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VectorOptions {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    indexing: Option<VectorFieldIndexing>,
    stored: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorFieldIndexing {
    dimension: u32,
    pq_bits: u32,
}

impl VectorFieldIndexing {
    pub fn new(dimension: u32, pq_bits: u32) -> Self {
        Self { dimension, pq_bits }
    }

    pub fn from_dimension(dimension: u32) -> Self {
        Self {
            dimension,
            pq_bits: dimension / 4,
        }
    }
}

impl VectorOptions {
    /// Returns true if the value is indexed.
    #[inline]
    pub fn is_indexed(&self) -> bool {
        self.indexing.is_some()
    }

    /// Always false for vectors
    #[inline]
    pub fn is_fast(&self) -> bool {
        false
    }

    /// Returns true iff the value is stored.
    #[inline]
    pub fn is_stored(&self) -> bool {
        self.stored
    }

    /// Sets the field as indexed, with the specific indexing options.
    pub fn set_indexing_options(mut self, indexing: VectorFieldIndexing) -> Self {
        self.indexing = Some(indexing);
        self
    }

    /// Gets the field's indexing options, if its indexed
    pub fn get_indexing_options(&self) -> Option<&VectorFieldIndexing> {
        self.indexing.as_ref()
    }
}

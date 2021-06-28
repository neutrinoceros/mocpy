//! Contains the code relative to the deserialization and serialization of MOCs.
//!
//! In the current status of the IVOA MOC standard, the serialization/deserialization is based on a
//! hierarchical view of a MOC, from the cells of lower resolution (smaller depth) to the cells
//! of higher resolution (largest depth).
//! Hence, we cannot perform serialization/deserialization in streaming, with a low memory footprint.
//!
//! The module also contain experimental code for streaming compatible serialization/deserialization.

pub mod ascii;
pub mod json;
pub mod fits;
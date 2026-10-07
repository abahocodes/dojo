//! `/contribute`: describe a question; a model drafts the whole asset; dojo
//! computes test outputs and validates; the contributor accepts (opening a
//! PR) or asks for changes, round after round.

pub mod draft;
pub mod github;
pub mod keys;
pub mod llm;
pub mod ollama;
pub mod workflow;

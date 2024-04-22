use anyhow::Result;
use openai::embeddings::{Embedding, Embeddings};
use shuttle_runtime::SecretStore;

use crate::{
    contents::File,
    errors::{EmbeddingError, SetupError},
};

pub fn setup(secrets: &SecretStore) -> Result<()> {
    let openai_key = secrets
        .get("OPENAI_API_KEY")
        .ok_or(SetupError("OPENAI Key not available"))?;

    openai::set_key(openai_key);
    Ok(())
}

pub async fn embed_file(file: &File) -> Result<Embeddings> {
    let sentence_as_str: Vec<&str> = file.sentences.iter().map(|s| s.as_str()).collect();
    Embeddings::create("text-embedding-ada-002", sentence_as_str, "deta")
        .await
        .map_err(|_| EmbeddingError {}.into())
}

pub async fn embed_sentence(prompt: &str) -> Result<Embedding> {
    Embedding::create("text-embedding-ada-002", prompt, "deta")
        .await
        .map_err(|_| EmbeddingError {}.into())
}

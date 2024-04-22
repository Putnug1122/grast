use anyhow::Result;
use axum::{routing::get, Router};
use contents::File;
use qdrant_client::prelude::QdrantClient;
use vector::VectorDB;

mod contents;
mod errors;
mod openai;
mod vector;

struct AppState {
    files: Vec<File>,
    vector_db: VectorDB,
}

async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
    #[shuttle_qdrant::Qdrant(
        cloud_url = "{secrets.QDRANT_URL}",
        api_key = "{secrets.QDRANT_TOKEN}"
    )]
    qdrant_client: QdrantClient,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}

async fn embed_documentation(vector_db: &mut VectorDB, files: &Vec<File>) -> Result<()> {
    for file in files {
        let embeddings = openai::embed_file(file).await?;
        println!("Embedding: {:?}", file.path);
        for embedding in embeddings.data {
            vector_db.upsert_embedding(embedding, file).await?;
        }
    }

    Ok(())
}

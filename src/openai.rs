use anyhow::Result;
use shuttle_runtime::SecretStore;

use crate::errors::SetupError;

pub fn setup(secrets: &SecretStore) -> Result<()> {
    let openai_key = secrets
        .get("OPENAI_API_KEY")
        .ok_or(SetupError("OPENAI Key not available"))?;

    openai::set_key(openai_key);
    Ok(())
}

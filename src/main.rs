use eyre::Result;
use agenda::start_main_loop;

#[tokio::main]
async fn main() -> Result<()> {
    start_main_loop().await?;
    Ok(())
}


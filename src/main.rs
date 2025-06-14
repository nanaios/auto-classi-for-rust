use anyhow::{Ok, Result};
mod login;

fn main() -> Result<()> {
    let _cookie = login::login_classi()?;
    Ok(())
}

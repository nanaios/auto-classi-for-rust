use anyhow::{Ok, Result};
mod login;
use login::{login_classi, write_cookie};

fn main() -> Result<()> {
    let cookies = login_classi()?;

    write_cookie(cookies);

    Ok(())
}

use anyhow_loc::{msg, ErrorLocation};

fn f() -> anyhow::Result<()> {
    anyhow::bail!("oh no!");
}

fn main() -> anyhow::Result<()> {
    let t = String::from("error 2");
    f().with_location(|| msg!("error")) // sep
        .with_location(|| msg!("{}", t))?;
    Ok(())
}

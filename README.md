# anyhow location

Error with location

```rust
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
```

Output

```text
Error: error 2 
 at examples/basic.rs:10

Caused by:
    0: error 
        at examples/basic.rs:9
    1: oh no!
```

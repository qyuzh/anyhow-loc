use std::fmt::Display;

pub struct Location {
    pub file: &'static str,
    pub line: u32,
    pub msg: String,
}

pub trait ErrorLocation<T, E>
where
    E: Display,
    Result<T, E>: anyhow::Context<T, E>,
{
    /// Used with `msg!` macro
    fn location(self, msg: Location) -> anyhow::Result<T>;

    /// Used with `msg!` macro
    fn with_location<F>(self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce() -> Location;
}

impl<T, E> ErrorLocation<T, E> for Result<T, E>
where
    E: Display,
    Result<T, E>: anyhow::Context<T, E>,
{
    fn location(self, msg: Location) -> anyhow::Result<T> {
        use anyhow::Context;
        self.with_context(|| format!("{} \n at {}:{}", msg.msg, msg.file, msg.line,))
    }

    fn with_location<F>(self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce() -> Location,
    {
        use anyhow::Context;
        let msg = f();
        self.with_context(|| format!("{} \n at {}:{}", msg.msg, msg.file, msg.line,))
    }
}

#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {
        $crate::Location {
            file: file!(),
            line: line!(),
            msg: format!($($arg)*),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f() -> anyhow::Result<()> {
        anyhow::bail!("oh no!");
    }

    #[test]
    #[should_panic(expected = "error 2")]
    fn it_location() {
        let t = String::from("error 2");
        f().location(msg!("error 1")) // t
            .location(msg!("{}", t))
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "error 2")]
    fn it_with_location() {
        let t = String::from("error 2");
        f().location(msg!("error 1")) // t
            .location(msg!("{}", t))
            .unwrap();
    }
}

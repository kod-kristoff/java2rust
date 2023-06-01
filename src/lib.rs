pub mod parser;
pub mod translator;

mod rewrite_rules;
use crate::parser::Tree;
pub fn generate_rust(tree: &Tree, dst: &mut dyn std::io::Write) -> anyhow::Result<()> {
    rewrite_rules::file(tree, dst)?;
    Ok(())
}

#[macro_export]
macro_rules! format_to {
    ($buf:expr) => ();
    ($buf:expr, $lit:literal, $($arg:tt)*) => {
        { use ::std::fmt::Write as _; ::std::write!($buf, $lit, $($arg)*)?; }

    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

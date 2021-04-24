use super::Fix;
use crate::{common::*, lint_kind::*};

pub(crate) struct TrailingWhitespaceFixer {}

impl Default for TrailingWhitespaceFixer {
    fn default() -> Self {
        Self {}
    }
}

impl Fix for TrailingWhitespaceFixer {
    fn name(&self) -> LintKind {
        LintKind::TrailingWhitespace
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        line.raw_string = line.raw_string.trim_end().to_string();

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn fix_line_test() {
        let mut fixer = TrailingWhitespaceFixer::default();
        let mut line = line_entry(1, 1, "DEBUG_HTTP=true  ");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("DEBUG_HTTP=true", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut TrailingWhitespaceFixer::default(),
            vec![
                TestLine::new("FOO=BAR ")
                    .warning(LintKind::TrailingWhitespace, "Trailing whitespace detected"),
                TestLine::new("Z=Y"),
                TestLine::new(""),
            ]
            .into(),
        );

        assert_eq!(Some(1), fix_count);
        assert_eq!(vec!["FOO=BAR", "Z=Y", ""], fixed_lines);
    }
}

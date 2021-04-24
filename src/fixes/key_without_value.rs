use super::Fix;
use crate::{common::*, lint_kind::*};

pub(crate) struct KeyWithoutValueFixer {}

impl Default for KeyWithoutValueFixer {
    fn default() -> Self {
        Self {}
    }
}

impl Fix for KeyWithoutValueFixer {
    fn name(&self) -> LintKind {
        LintKind::KeyWithoutValue
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        line.raw_string.push('=');

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn fix_line_test() {
        let mut fixer = KeyWithoutValueFixer::default();
        let mut line = line_entry(1, 1, "FOO");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut KeyWithoutValueFixer::default(),
            vec![
                TestLine::new("FOO").warning(
                    LintKind::KeyWithoutValue,
                    "The FOO key should be with a value or have an equal sign",
                ),
                TestLine::new("Z=Y"),
                TestLine::new("\n"),
            ]
            .into(),
        );

        assert_eq!(Some(1), fix_count);
        assert_eq!(vec!["FOO=", "Z=Y", "\n"], fixed_lines);
    }
}

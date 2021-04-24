use super::Fix;
use crate::{common::*, lint_kind::*};

pub(crate) struct LowercaseKeyFixer {}

impl Default for LowercaseKeyFixer {
    fn default() -> Self {
        Self {}
    }
}

impl Fix for LowercaseKeyFixer {
    fn name(&self) -> LintKind {
        LintKind::LowercaseKey
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        let key = line.get_key()?;
        let key = key.to_uppercase();
        line.raw_string = format!("{}={}", key, line.get_value()?);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn fix_line_test() {
        let mut fixer = LowercaseKeyFixer::default();
        let mut line = line_entry(1, 1, "foO=BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=BAR", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut LowercaseKeyFixer::default(),
            vec![
                TestLine::new("foO=BAR")
                    .warning(LintKind::LowercaseKey, "The FOO key should be in uppercase"),
                TestLine::new("Z=Y"),
                TestLine::new(""),
            ]
            .into(),
        );

        assert_eq!(Some(1), fix_count);
        assert_eq!(vec!["FOO=BAR", "Z=Y", ""], fixed_lines);
    }
}

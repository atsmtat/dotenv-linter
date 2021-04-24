use super::Fix;
use crate::{common::*, lint_kind::*};

pub(crate) struct EndingBlankLineFixer {}

impl Default for EndingBlankLineFixer {
    fn default() -> Self {
        Self {}
    }
}

impl Fix for EndingBlankLineFixer {
    fn name(&self) -> LintKind {
        LintKind::EndingBlankLine
    }

    fn fix_warnings(
        &mut self,
        _warnings: Vec<&mut Warning>,
        lines: &mut Vec<LineEntry>,
    ) -> Option<usize> {
        let last_line = lines.last()?;

        if last_line.raw_string.ends_with(LF) {
            return Some(0);
        }

        let file = lines.first()?.file.clone();
        lines.push(LineEntry::new(lines.len() + 1, file, LF));

        Some(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn fix_warnings_test() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut EndingBlankLineFixer::default(),
            vec![
                TestLine::new("FOO=BAR"),
                TestLine::new("Z=Y").warning(
                    LintKind::EndingBlankLine,
                    "No blank line at the end of the file",
                ),
            ]
            .into(),
        );

        assert_eq!(Some(1), fix_count);
        assert_eq!(vec!["FOO=BAR", "Z=Y", "\n"], fixed_lines);
    }

    #[test]
    fn ending_blank_line_exist_test() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut EndingBlankLineFixer::default(),
            vec![TestLine::new("FOO=BAR"), TestLine::new("\n")].into(),
        );

        assert_eq!(Some(0), fix_count);
        assert_eq!(vec!["FOO=BAR", "\n"], fixed_lines);
    }
}

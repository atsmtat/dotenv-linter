use super::Fix;
use crate::{common::*, lint_kind::*};
use std::collections::HashSet;

pub(crate) struct DuplicatedKeyFixer {}

impl Default for DuplicatedKeyFixer {
    fn default() -> Self {
        Self {}
    }
}

impl Fix for DuplicatedKeyFixer {
    fn name(&self) -> LintKind {
        LintKind::DuplicatedKey
    }

    fn fix_warnings(
        &mut self,
        warnings: Vec<&mut Warning>,
        lines: &mut Vec<LineEntry>,
    ) -> Option<usize> {
        let mut keys = HashSet::with_capacity(lines.len());
        let mut is_disabled = false;

        for line in lines {
            if let Some(comment) = line.get_control_comment() {
                if comment.checks.contains(&self.name()) {
                    is_disabled = comment.is_disabled();
                }
            }
            if is_disabled {
                continue;
            }

            if let Some(key) = line.get_key() {
                if keys.contains(key) {
                    self.fix_line(line)?;
                } else {
                    keys.insert(key.to_string());
                }
            }
        }

        Some(warnings.len())
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        line.raw_string = format!("# {}", line.raw_string);

        Some(())
    }

    fn is_mandatory(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn fix_warnings() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut DuplicatedKeyFixer::default(),
            vec![
                TestLine::new("FOO=BAR"),
                TestLine::new("Z=Y"),
                TestLine::new("FOO=BAZ")
                    .warning(LintKind::DuplicatedKey, "The Foo key is duplicated"),
                TestLine::new("Z=X").warning(LintKind::DuplicatedKey, "The Z key is duplicated"),
            ]
            .into(),
        );

        assert_eq!(Some(2), fix_count);
        assert_eq!(vec!["FOO=BAR", "Z=Y", "# FOO=BAZ", "# Z=X"], fixed_lines);
    }

    #[test]
    fn fix_lines_without_warnings() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut DuplicatedKeyFixer::default(),
            vec![
                TestLine::new("FOO=BAR"),
                TestLine::new("FOO=BAZ"),
                TestLine::new("Z=Y"),
                TestLine::new("Z=X"),
            ]
            .into(),
        );

        assert_eq!(Some(0), fix_count);
        assert_eq!(vec!["FOO=BAR", "# FOO=BAZ", "Z=Y", "# Z=X"], fixed_lines);
    }

    #[test]
    fn control_comment_at_first_line() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut DuplicatedKeyFixer::default(),
            vec![
                TestLine::new("# dotenv-linter:off DuplicatedKey"),
                TestLine::new("FOO=BAR"),
                TestLine::new("FOO=BAZ"),
                TestLine::new("Z=Y"),
                TestLine::new("Z=X"),
            ]
            .into(),
        );

        assert_eq!(Some(0), fix_count);
        assert_eq!(
            vec![
                "# dotenv-linter:off DuplicatedKey",
                "FOO=BAR",
                "FOO=BAZ",
                "Z=Y",
                "Z=X"
            ],
            fixed_lines
        );
    }

    #[test]
    fn control_comment_in_the_middle() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut DuplicatedKeyFixer::default(),
            vec![
                TestLine::new("FOO=BAR"),
                TestLine::new("# dotenv-linter:off DuplicatedKey"),
                TestLine::new("FOO=BAZ"),
                TestLine::new("Z=Y"),
                TestLine::new("# dotenv-linter:on DuplicatedKey"),
                TestLine::new("Z=X"),
            ]
            .into(),
        );

        assert_eq!(Some(0), fix_count);
        assert_eq!(
            vec![
                "FOO=BAR",
                "# dotenv-linter:off DuplicatedKey",
                "FOO=BAZ",
                "Z=Y",
                "# dotenv-linter:on DuplicatedKey",
                "Z=X"
            ],
            fixed_lines
        );
    }

    #[test]
    fn unrelated_control_comment() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut DuplicatedKeyFixer::default(),
            vec![
                TestLine::new("# dotenv-linter:off LowercaseKey"),
                TestLine::new("FOO=BAR"),
                TestLine::new("FOO=BAZ"),
                TestLine::new("Z=Y"),
                TestLine::new("Z=X"),
            ]
            .into(),
        );

        assert_eq!(Some(0), fix_count);
        assert_eq!(
            vec![
                "# dotenv-linter:off LowercaseKey",
                "FOO=BAR",
                "# FOO=BAZ",
                "Z=Y",
                "# Z=X"
            ],
            fixed_lines
        );
    }
}

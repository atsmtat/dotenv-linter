use super::Fix;
use crate::{common::*, lint_kind::*};

pub(crate) struct SpaceCharacterFixer {}

impl Default for SpaceCharacterFixer {
    fn default() -> Self {
        Self {}
    }
}

impl Fix for SpaceCharacterFixer {
    fn name(&self) -> LintKind {
        LintKind::SpaceCharacter
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        let key = line.get_key()?;
        let value = line.get_value()?;
        line.raw_string = format!("{}={}", key.trim_end(), value.trim_start());

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn fix_line_test() {
        let mut fixer = SpaceCharacterFixer::default();
        let mut line = line_entry(1, 1, "FOO = BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=BAR", line.raw_string);
    }

    #[test]
    fn trailing_should_not_be_fixed() {
        let mut fixer = SpaceCharacterFixer::default();
        let mut line = line_entry(1, 1, "DEBUG_HTTP=true ");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("DEBUG_HTTP=true ", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut SpaceCharacterFixer::default(),
            vec![
                TestLine::new("FOO= BAR").warning(
                    LintKind::SpaceCharacter,
                    "The line has spaces around equal sign",
                ),
                TestLine::new("Z =Y").warning(
                    LintKind::SpaceCharacter,
                    "The line has spaces around equal sign",
                ),
                TestLine::new(""),
            ]
            .into(),
        );

        assert_eq!(Some(2), fix_count);
        assert_eq!(vec!["FOO=BAR", "Z=Y", ""], fixed_lines);
    }
}

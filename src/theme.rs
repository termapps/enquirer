use console::{Style, StyledObject};
use dialoguer::theme::Theme;
use std::fmt;

pub struct Colored {
    brblack: Style,
    prompts_style: Style,
    cyan: Style,
    green: Style,
}

impl Default for Colored {
    fn default() -> Self {
        Colored {
            brblack: Style::new().black().bold(),
            prompts_style: Style::new().bold(),
            cyan: Style::new().cyan(),
            green: Style::new().green(),
        }
    }
}

impl Colored {
    fn empty(&self) -> (StyledObject<&str>, StyledObject<&str>) {
        (
            self.prompts_style.apply_to(""),
            self.prompts_style.apply_to(""),
        )
    }
}

impl Theme for Colored {
    fn format_confirmation_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> fmt::Result {
        let details = match default {
            None => self.empty(),
            Some(true) => (self.brblack.apply_to("(Y/n)"), self.cyan.apply_to("true")),
            Some(false) => (self.brblack.apply_to("(y/N)"), self.cyan.apply_to("false")),
        };

        write!(
            f,
            "{} {} {} {} {} ",
            self.cyan.apply_to("?"),
            self.prompts_style.apply_to(prompt),
            details.0,
            self.brblack.apply_to("›"),
            details.1,
        )?;

        Ok(())
    }

    fn format_confirmation_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selection: bool,
    ) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.green.apply_to("✔"),
            self.prompts_style.apply_to(prompt),
            self.brblack.apply_to("·"),
            self.green
                .apply_to(if selection { "true" } else { "false" }),
        )?;

        Ok(())
    }
}

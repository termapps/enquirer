use console::{Style, StyledObject};
use dialoguer::theme::{ColorfulTheme, Theme};
use std::fmt;

pub struct Colored {
    styles: ColorfulTheme,
    prompts_style: Style,
    question_style: Style,
    tick_style: Style,
}

impl Default for Colored {
    fn default() -> Self {
        Colored {
            styles: ColorfulTheme::default(),
            prompts_style: Style::new().bold(),
            question_style: Style::new().cyan(),
            tick_style: Style::new().green(),
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
            Some(true) => (
                self.styles.defaults_style.apply_to("(Y/n)"),
                self.styles.values_style.apply_to("true"),
            ),
            Some(false) => (
                self.styles.defaults_style.apply_to("(y/N)"),
                self.styles.values_style.apply_to("false"),
            ),
        };

        write!(
            f,
            "{} {} {} {} {} ",
            self.question_style.apply_to("?"),
            self.prompts_style.apply_to(prompt),
            details.0,
            self.styles.defaults_style.apply_to("›"),
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
            self.tick_style.apply_to("✔"),
            self.prompts_style.apply_to(prompt),
            self.styles.defaults_style.apply_to("·"),
            {
                if selection {
                    self.styles.yes_style.apply_to("true")
                } else {
                    self.styles.no_style.apply_to("false")
                }
            }
        )?;

        Ok(())
    }
}

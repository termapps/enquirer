use console::{Style, StyledObject};
use dialoguer::theme::Theme;
use std::fmt;

#[allow(clippy::needless_doctest_main)]
/// Provides a colored theme for dialoguer
///
/// # Examples
///
/// ```
/// use dialoguer::Confirmation;
/// use enquirer::ColoredTheme;
///
/// fn main() {
///     let prompt = Confirmation::with_theme(&ColoredTheme::default())
///         .with_text("Do you want to continue?")
///         .with_default(true);
///
///     if prompt.interact()? {
///         println!("Looks like you want to continue");
///     } else {
///         println!("nevermind then :(");
///     }
/// }
/// ```
pub struct ColoredTheme {
    brblack: Style,
    prompts_style: Style,
    cyan: Style,
    green: Style,
}

impl Default for ColoredTheme {
    fn default() -> Self {
        ColoredTheme {
            brblack: Style::new().black().bold(),
            prompts_style: Style::new().bold(),
            cyan: Style::new().cyan(),
            green: Style::new().green(),
        }
    }
}

impl ColoredTheme {
    fn empty(&self) -> (StyledObject<&str>, StyledObject<&str>) {
        (
            self.prompts_style.apply_to(""),
            self.prompts_style.apply_to(""),
        )
    }
}

impl Theme for ColoredTheme {
    // Input
    fn format_singleline_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> fmt::Result {
        let details = match default {
            Some(default) => format!(" ({})", default),
            None => "".to_string(),
        };

        write!(
            f,
            "{} {}{} {} ",
            self.cyan.apply_to("?"),
            self.prompts_style.apply_to(prompt),
            self.brblack.apply_to(details),
            self.brblack.apply_to("›"),
        )?;

        Ok(())
    }

    // Input Selection
    fn format_single_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selection: &str,
    ) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.green.apply_to("✔"),
            self.prompts_style.apply_to(prompt),
            self.brblack.apply_to("·"),
            self.green.apply_to(selection),
        )?;

        Ok(())
    }

    // Confirm
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

    // Confirm Selection
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

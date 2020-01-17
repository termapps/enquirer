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
    defaults_style: Style,
    prompts_style: Style,
    prefixes_style: Style,
    values_style: Style,
    errors_style: Style,
}

impl Default for ColoredTheme {
    fn default() -> Self {
        ColoredTheme {
            defaults_style: Style::new().black().bold(),
            prompts_style: Style::new().bold(),
            prefixes_style: Style::new().cyan(),
            values_style: Style::new().green(),
            errors_style: Style::new().red(),
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
    // Error
    fn format_error(&self, f: &mut dyn fmt::Write, err: &str) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.errors_style.apply_to("✘"),
            self.errors_style.apply_to(err)
        )?;

        Ok(())
    }

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
            self.prefixes_style.apply_to("?"),
            self.prompts_style.apply_to(prompt),
            self.defaults_style.apply_to(details),
            self.defaults_style.apply_to("›"),
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
            self.values_style.apply_to("✔"),
            self.prompts_style.apply_to(prompt),
            self.defaults_style.apply_to("·"),
            self.values_style.apply_to(selection),
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
            Some(true) => (
                self.defaults_style.apply_to("(Y/n)"),
                self.prefixes_style.apply_to("true"),
            ),
            Some(false) => (
                self.defaults_style.apply_to("(y/N)"),
                self.prefixes_style.apply_to("false"),
            ),
        };

        write!(
            f,
            "{} {} {} {} {} ",
            self.prefixes_style.apply_to("?"),
            self.prompts_style.apply_to(prompt),
            details.0,
            self.defaults_style.apply_to("›"),
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
            self.values_style.apply_to("✔"),
            self.prompts_style.apply_to(prompt),
            self.defaults_style.apply_to("·"),
            self.values_style
                .apply_to(if selection { "true" } else { "false" }),
        )?;

        Ok(())
    }

    // Password Selection
    fn format_password_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
    ) -> fmt::Result {
        self.format_single_prompt_selection(f, prompt, "********")
    }
}

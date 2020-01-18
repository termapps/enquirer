use console::{Style, StyledObject};
use dialoguer::theme::{SelectionStyle, Theme};
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
    pub defaults_style: Style,
    pub prompts_style: Style,
    pub prefixes_style: Style,
    pub values_style: Style,
    pub errors_style: Style,
    pub selected_style: Style,
    pub unselected_style: Style,
    pub inline_selections: bool,
}

impl Default for ColoredTheme {
    fn default() -> Self {
        ColoredTheme {
            defaults_style: Style::new().black().bold(),
            prompts_style: Style::new().bold(),
            prefixes_style: Style::new().cyan(),
            values_style: Style::new().green(),
            errors_style: Style::new().red(),
            selected_style: Style::new().cyan().bold(),
            unselected_style: Style::new(),
            inline_selections: true,
        }
    }
}

impl ColoredTheme {
    /// Checkboxes print the selected values on the prompt line.
    /// This option allows the user to customize whether
    /// those will be printed on the prompts line or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use enquirer::ColoredTheme;
    ///
    /// let theme = ColoredTheme::default().inline_selections(false);
    /// ```
    pub fn inline_selections(mut self, val: bool) -> Self {
        self.inline_selections = val;
        self
    }

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

    // Prompt
    fn format_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.prefixes_style.apply_to("?"),
            self.prompts_style.apply_to(prompt),
            self.defaults_style.apply_to("›")
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

    // Selection
    fn format_selection(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        style: SelectionStyle,
    ) -> fmt::Result {
        let strings = match style {
            SelectionStyle::CheckboxCheckedSelected => (
                self.values_style.apply_to("✔"),
                self.selected_style.apply_to(text),
            ),
            SelectionStyle::CheckboxCheckedUnselected => (
                self.values_style.apply_to("✔"),
                self.unselected_style.apply_to(text),
            ),
            SelectionStyle::CheckboxUncheckedSelected => (
                self.defaults_style.apply_to("✔"),
                self.selected_style.apply_to(text),
            ),
            SelectionStyle::CheckboxUncheckedUnselected => (
                self.defaults_style.apply_to("✔"),
                self.unselected_style.apply_to(text),
            ),
            SelectionStyle::MenuSelected => (
                self.values_style.apply_to("❯"),
                self.selected_style.apply_to(text),
            ),
            SelectionStyle::MenuUnselected => (
                self.defaults_style.apply_to(" "),
                self.unselected_style.apply_to(text),
            ),
        };

        write!(f, "{} {}", strings.0, strings.1)?;

        Ok(())
    }

    // Multi Prompt Selection
    fn format_multi_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.values_style.apply_to("✔"),
            self.prompts_style.apply_to(prompt),
            self.defaults_style.apply_to("·"),
        )?;

        if self.inline_selections {
            let selections_last_index = selections.len() - 1;

            for (i, v) in selections.iter().enumerate() {
                if i == selections_last_index {
                    write!(f, " {}", self.values_style.apply_to(v))?;
                } else {
                    write!(f, " {},", self.values_style.apply_to(v))?;
                }
            }
        }

        Ok(())
    }
}

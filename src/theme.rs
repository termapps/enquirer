use dialoguer::theme::{ColorfulTheme, Theme};
use std::fmt::{Result, Write};

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
    pub base: ColorfulTheme,
    /// Defaults to `true`
    pub inline_selections: bool,
}

impl Default for ColoredTheme {
    fn default() -> Self {
        ColoredTheme {
            base: ColorfulTheme::default(),
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
}

impl Theme for ColoredTheme {
    // Multi Prompt Selection
    fn format_multi_select_prompt_selection(
        &self,
        f: &mut dyn Write,
        prompt: &str,
        selections: &[&str],
    ) -> Result {
        write!(
            f,
            "{} {} {} ",
            &self.base.success_prefix,
            self.base.prompt_style.apply_to(prompt),
            &self.base.success_suffix,
        )?;

        if self.inline_selections {
            for (i, v) in selections.iter().enumerate() {
                write!(
                    f,
                    "{}{}",
                    if i == 0 { "" } else { ", " },
                    self.base.values_style.apply_to(v)
                )?;
            }
        }

        Ok(())
    }

    fn format_prompt(&self, f: &mut dyn Write, prompt: &str) -> Result {
        self.base.format_prompt(f, prompt)
    }

    fn format_error(&self, f: &mut dyn Write, err: &str) -> Result {
        self.base.format_error(f, err)
    }

    fn format_confirm_prompt(
        &self,
        f: &mut dyn Write,
        prompt: &str,
        default: Option<bool>,
    ) -> Result {
        self.base.format_confirm_prompt(f, prompt, default)
    }

    fn format_confirm_prompt_selection(
        &self,
        f: &mut dyn Write,
        prompt: &str,
        selection: bool,
    ) -> Result {
        self.base
            .format_confirm_prompt_selection(f, prompt, selection)
    }

    fn format_input_prompt(
        &self,
        f: &mut dyn Write,
        prompt: &str,
        default: Option<&str>,
    ) -> Result {
        self.base.format_input_prompt(f, prompt, default)
    }

    fn format_input_prompt_selection(&self, f: &mut dyn Write, prompt: &str, sel: &str) -> Result {
        self.base.format_input_prompt_selection(f, prompt, sel)
    }

    fn format_password_prompt_selection(&self, f: &mut dyn Write, prompt: &str) -> Result {
        self.base.format_password_prompt_selection(f, prompt)
    }

    fn format_select_prompt_item(&self, f: &mut dyn Write, text: &str, active: bool) -> Result {
        self.base.format_select_prompt_item(f, text, active)
    }

    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> Result {
        self.base
            .format_multi_select_prompt_item(f, text, checked, active)
    }

    fn format_sort_prompt_item(
        &self,
        f: &mut dyn Write,
        text: &str,
        picked: bool,
        active: bool,
    ) -> Result {
        self.base.format_sort_prompt_item(f, text, picked, active)
    }
}

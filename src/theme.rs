use crate::OutputLabel;
use dialoguer::{
	console::{style, Style, StyledObject},
	theme::Theme,
};
use std::fmt;

macro_rules! write_label {
	($dst:expr, $label:expr, $($arg:tt)*) => {
		write!(
			$dst,
			"{}",
			$crate::pretty_output($label, format!($($arg)*), false)
		)
	};
}

/// The theme that goes with others logging utilities in this crate
pub struct LabelTheme {
	/// The style for default values
	pub defaults_style: Style,
	/// The style for prompt
	pub prompt_style: Style,
	/// Prompt prefix value and style
	pub prompt_prefix: StyledObject<String>,
	/// Prompt suffix value and style
	pub prompt_suffix: StyledObject<String>,
	/// Prompt on success prefix value and style
	pub success_prefix: StyledObject<String>,
	/// Prompt on success suffix value and style
	pub success_suffix: StyledObject<String>,
	/// Error prefix value and style
	pub error_prefix: StyledObject<String>,
	/// The style for error message
	pub error_style: Style,
	/// The style for hints
	pub hint_style: Style,
	/// The style for values on prompt success
	pub values_style: Style,
	/// The style for active items
	pub active_item_style: Style,
	/// The style for inactive items
	pub inactive_item_style: Style,
	/// Active item in select prefix value and style
	pub active_item_prefix: StyledObject<String>,
	/// Inactive item in select prefix value and style
	pub inactive_item_prefix: StyledObject<String>,
	/// Checked item in multi select prefix value and style
	pub checked_item_prefix: StyledObject<String>,
	/// Unchecked item in multi select prefix value and style
	pub unchecked_item_prefix: StyledObject<String>,
	/// Picked item in sort prefix value and style
	pub picked_item_prefix: StyledObject<String>,
	/// Unpicked item in sort prefix value and style
	pub unpicked_item_prefix: StyledObject<String>,
	/// Formats the cursor for a fuzzy select prompt
	#[cfg(feature = "fuzzy-select")]
	pub fuzzy_cursor_style: Style,
	/// Show the selections from certain prompts inline
	pub inline_selections: bool,
}

impl Default for LabelTheme {
	fn default() -> Self {
		Self {
			defaults_style: Style::new().for_stderr().cyan(),
			prompt_style: Style::new().for_stderr().bold(),
			prompt_prefix: style("?".to_string()).for_stderr().yellow(),
			prompt_suffix: style("›".to_string()).for_stderr().black().bright(),
			success_prefix: style("✔".to_string()).for_stderr().green(),
			success_suffix: style("·".to_string()).for_stderr().black().bright(),
			error_prefix: style("✘".to_string()).for_stderr().red(),
			error_style: Style::new().for_stderr().red(),
			hint_style: Style::new().for_stderr().black().bright(),
			values_style: Style::new().for_stderr().green(),
			active_item_style: Style::new().for_stderr().cyan(),
			inactive_item_style: Style::new().for_stderr(),
			active_item_prefix: style("❯".to_string()).for_stderr().green(),
			inactive_item_prefix: style(" ".to_string()).for_stderr(),
			checked_item_prefix: style("✔".to_string()).for_stderr().green(),
			unchecked_item_prefix: style("✔".to_string()).for_stderr().black(),
			picked_item_prefix: style("❯".to_string()).for_stderr().green(),
			unpicked_item_prefix: style(" ".to_string()).for_stderr(),
			#[cfg(feature = "fuzzy-select")]
			fuzzy_cursor_style: Style::new().for_stderr().black().on_white(),
			inline_selections: true,
		}
	}
}

impl Theme for LabelTheme {
	/// Formats a prompt.
	fn format_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
		write_label!(
			f,
			OutputLabel::Prompt(self.prompt_prefix.to_string().as_str()),
			"{}",
			self.prompt_style.apply_to(prompt)
		)
	}

	/// Formats an error
	fn format_error(&self, f: &mut dyn fmt::Write, err: &str) -> fmt::Result {
		write_label!(
			f,
			OutputLabel::Prompt(self.error_prefix.to_string().as_str()),
			"{}",
			self.error_style.apply_to(err)
		)
	}

	/// Formats an input prompt.
	fn format_input_prompt(
		&self,
		f: &mut dyn fmt::Write,
		prompt: &str,
		default: Option<&str>,
	) -> fmt::Result {
		let suffix = match default {
			Some(default) => format!(
				"{} {} ",
				self.hint_style.apply_to(&format!("({})", default)),
				&self.prompt_suffix
			),
			None => format!("{} ", self.prompt_suffix),
		};

		write_label!(
			f,
			OutputLabel::Prompt(self.prompt_prefix.to_string().as_str()),
			"{} {}",
			self.prompt_style.apply_to(prompt),
			suffix
		)
	}

	/// Formats a confirm prompt.
	fn format_confirm_prompt(
		&self,
		f: &mut dyn fmt::Write,
		prompt: &str,
		default: Option<bool>,
	) -> fmt::Result {
		let yes_no_hint = match default {
			None => format!(
				"{} {}",
				self.hint_style.apply_to("(y/n)"),
				&self.prompt_suffix
			),
			Some(true) => format!(
				"{} {} {}",
				self.hint_style.apply_to("(y/n)"),
				&self.prompt_suffix,
				self.defaults_style.apply_to("yes")
			),
			Some(false) => format!(
				"{} {} {}",
				self.hint_style.apply_to("(y/n)"),
				&self.prompt_suffix,
				self.defaults_style.apply_to("no")
			),
		};

		write_label!(
			f,
			OutputLabel::Prompt(self.prompt_prefix.to_string().as_str()),
			"{} {}",
			self.prompt_style.apply_to(prompt),
			yes_no_hint
		)
	}

	/// Formats a confirm prompt after selection.
	fn format_confirm_prompt_selection(
		&self,
		f: &mut dyn fmt::Write,
		prompt: &str,
		selection: Option<bool>,
	) -> fmt::Result {
		if !prompt.is_empty() {
			write_label!(
				f,
				OutputLabel::Success(self.success_prefix.to_string().as_str()),
				"{}",
				self.prompt_style.apply_to(prompt)
			)?;
		}

		let selection = selection.map(|b| if b { "yes" } else { "no" });

		match selection {
			// TODO: see
			Some(selection) => {
				write!(
					f,
					"{} {}",
					&self.success_suffix,
					self.values_style.apply_to(selection)
				)
			}
			None => {
				write!(f, "{}", &self.success_suffix)
			}
		}
	}

	/// Formats an input prompt after selection.
	fn format_input_prompt_selection(
		&self,
		f: &mut dyn fmt::Write,
		prompt: &str,
		sel: &str,
	) -> fmt::Result {
		if !prompt.is_empty() {
			write_label!(
				f,
				OutputLabel::Success(self.success_prefix.to_string().as_str()),
				"{}",
				self.prompt_style.apply_to(prompt)
			)?;
		}

		write!(
			f,
			" {} {}",
			&self.success_suffix,
			self.values_style.apply_to(sel)
		)
	}

	/// Formats a password prompt after selection.
	#[cfg(feature = "password")]
	fn format_password_prompt_selection(
		&self,
		f: &mut dyn fmt::Write,
		prompt: &str,
	) -> fmt::Result {
		self.format_input_prompt_selection(f, prompt, "********")
	}

	/// Formats a multi select prompt after selection.
	fn format_multi_select_prompt_selection(
		&self,
		f: &mut dyn fmt::Write,
		prompt: &str,
		selections: &[&str],
	) -> fmt::Result {
		if !prompt.is_empty() {
			write_label!(
				f,
				OutputLabel::Success(self.success_prefix.to_string().as_str()),
				"{}",
				self.prompt_style.apply_to(prompt)
			)?;
		}

		write!(f, "{} ", &self.success_suffix)?;

		if self.inline_selections {
			for (idx, sel) in selections.iter().enumerate() {
				write!(
					f,
					"{}{}",
					if idx == 0 { "" } else { ", " },
					self.values_style.apply_to(sel)
				)?;
			}
		}

		Ok(())
	}

	/// Formats a select prompt item.
	fn format_select_prompt_item(
		&self,
		f: &mut dyn fmt::Write,
		text: &str,
		active: bool,
	) -> fmt::Result {
		let details = if active {
			(
				&self.active_item_prefix,
				self.active_item_style.apply_to(text),
			)
		} else {
			(
				&self.inactive_item_prefix,
				self.inactive_item_style.apply_to(text),
			)
		};

		write_label!(f, OutputLabel::None, "{} {}", details.0, details.1)
	}

	/// Formats a multi select prompt item.
	fn format_multi_select_prompt_item(
		&self,
		f: &mut dyn fmt::Write,
		text: &str,
		checked: bool,
		active: bool,
	) -> fmt::Result {
		let details = match (checked, active) {
			(true, true) => (
				&self.checked_item_prefix,
				self.active_item_style.apply_to(text),
			),
			(true, false) => (
				&self.checked_item_prefix,
				self.inactive_item_style.apply_to(text),
			),
			(false, true) => (
				&self.unchecked_item_prefix,
				self.active_item_style.apply_to(text),
			),
			(false, false) => (
				&self.unchecked_item_prefix,
				self.inactive_item_style.apply_to(text),
			),
		};

		write_label!(f, OutputLabel::None, "{} {}", details.0, details.1)
	}

	/// Formats a sort prompt item.
	fn format_sort_prompt_item(
		&self,
		f: &mut dyn fmt::Write,
		text: &str,
		picked: bool,
		active: bool,
	) -> fmt::Result {
		let details = match (picked, active) {
			(true, true) => (
				&self.picked_item_prefix,
				self.active_item_style.apply_to(text),
			),
			(false, true) => (
				&self.unpicked_item_prefix,
				self.active_item_style.apply_to(text),
			),
			(_, false) => (
				&self.unpicked_item_prefix,
				self.inactive_item_style.apply_to(text),
			),
		};

		write_label!(f, OutputLabel::None, "{} {}", details.0, details.1)
	}

	/// Formats a fuzzy-select prompt after selection.
	#[cfg(feature = "fuzzy-select")]
	fn format_fuzzy_select_prompt(
		&self,
		f: &mut dyn fmt::Write,
		prompt: &str,
		search_term: &str,
		cursor_pos: usize,
	) -> fmt::Result {
		if !prompt.is_empty() {
			write_label!(
				f,
				OutputLabel::Success(self.success_prefix.to_string().as_str()),
				"{}",
				self.prompt_style.apply_to(prompt)
			)?;
		}

		if cursor_pos < search_term.len() {
			let st_head = search_term[0..cursor_pos].to_string();
			let st_tail = search_term[cursor_pos + 1..search_term.len()].to_string();
			let st_cursor = self
				.fuzzy_cursor_style
				.apply_to(search_term.to_string().chars().nth(cursor_pos).unwrap());

			write!(
				f,
				" {} {}{}{}",
				&self.prompt_suffix, st_head, st_cursor, st_tail
			)
		} else {
			let cursor = self.fuzzy_cursor_style.apply_to(" ");
			write!(f, " {} {}{}", &self.prompt_suffix, search_term, cursor)
		}
	}
}
use json_gettext::{get_text, static_json_gettext_build, JSONGetText};
use once_cell::sync::{Lazy, OnceCell};

use std::sync::RwLock;

pub static LOCALIZATION_CTX: Lazy<JSONGetText<'static>> = Lazy::new(|| {
	static_json_gettext_build!(
		"en_US",
		"en_US",
		"locale/en.json",
		"de_DE",
		"locale/de.json"
	)
	.unwrap()
});

pub static LANG: OnceCell<RwLock<&'static str>> = OnceCell::new();

pub fn localized_string(key: &str) -> String {
	let lang = LANG.get().expect("LANG not set").read().unwrap();

	if let Some(text) = get_text!(LOCALIZATION_CTX, *lang, key) {
		let text = text.to_string();
		if text.is_empty() {
			key.to_owned()
		} else {
			text
		}
	} else {
		key.to_owned()
	}
}

/// Returns a localized `timeago::Formatter`.
/// If user has chosen a language whic his not supported by `timeago` we fallback to english.
pub fn localized_timeago_formatter() -> timeago::Formatter<Box<dyn timeago::Language>> {
	let lang = LANG.get().expect("LANG not set").read().unwrap();
	let isolang = isolang::Language::from_locale(&lang).unwrap();

	// this step might fail if timeago does not support the chosen language.
	// In that case we fallback to `en_US`.
	if let Some(timeago_lang) = timeago::from_isolang(isolang) {
		timeago::Formatter::with_language(timeago_lang)
	} else {
		timeago::Formatter::with_language(Box::new(timeago::English))
	}
}

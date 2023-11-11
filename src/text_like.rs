use std::borrow::Cow;

use crate::static_text_repos::register_text_as_key;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TextLike<'a> {
    Static(&'static str),
    Ref(&'a str),
    Owned(String),
}

impl<'a> std::fmt::Display for TextLike<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> TextLike<'a> {
    pub(crate) fn into_registered_name(self) -> &'static str {
        register_text_as_key(self)
    }
    pub fn as_str(&self) -> &str {
        match self {
            TextLike::Static(static_ref) => static_ref,
            TextLike::Ref(reference) => reference,
            TextLike::Owned(owned) => owned.as_str(),
        }
    }
}

impl<'a> From<TextLike<'a>> for Cow<'static, str> {
    fn from(value: TextLike<'a>) -> Self {
        match value {
            TextLike::Ref(refernce) => Cow::Owned(refernce.to_string()),
            TextLike::Static(static_ref) => Cow::Borrowed(static_ref),
            TextLike::Owned(string) => Cow::Owned(string),
        }
    }
}

impl<'a> From<&'static str> for TextLike<'a> {
    fn from(value: &'static str) -> Self {
        Self::Static(value)
    }
}

impl<'a, T> From<&'a T> for TextLike<'a>
where
    T: AsRef<str>,
{
    fn from(value: &'a T) -> Self {
        Self::Ref(value.as_ref())
    }
}
impl<'a> From<String> for TextLike<'a> {
    fn from(value: String) -> Self {
        Self::Owned(value)
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn to_correct_variants() {
        let static_only = "static";
        assert_eq!(TextLike::Static(static_only), TextLike::from(static_only));
        let data = String::from("reference");
        let text_like = TextLike::from(&data);
        assert_eq!(TextLike::Ref(&data), text_like);
        test_ref(&String::from("aaa"));
        test_static("aaa");
        test_owned(String::from("aaa"));

        fn test_ref<'a>(text: impl Into<TextLike<'a>>) {
            assert_eq!(TextLike::Ref("aaa"), text.into());
        }
        fn test_static<'a>(text: impl Into<TextLike<'a>>) {
            assert_eq!(TextLike::Static("aaa"), text.into());
        }

        fn test_owned<'a>(text: impl Into<TextLike<'a>>) {
            assert_eq!(TextLike::Owned(String::from("aaa")), text.into());
        }
    }
}

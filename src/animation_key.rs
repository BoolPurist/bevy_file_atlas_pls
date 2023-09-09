use std::{borrow::Borrow, sync::Arc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct AnimationKey(Arc<str>);

impl std::fmt::Display for AnimationKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for AnimationKey {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}
impl From<&str> for AnimationKey {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl Borrow<str> for AnimationKey {
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for AnimationKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AnimationKey {
    pub fn new(key: &str) -> Self {
        Self(key.into())
    }
}

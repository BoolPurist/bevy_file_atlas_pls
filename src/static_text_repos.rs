use std::sync::Mutex;

use bevy::utils::HashMap;
use once_cell::sync::Lazy;

use crate::text_like::TextLike;

type HackToGetLifeTimeOfKey = HashMap<&'static str, &'static str>;
static TEXT_REPOS: Lazy<Mutex<HackToGetLifeTimeOfKey>> =
    Lazy::new(|| Mutex::new(Default::default()));
fn hack_insert(hack: &mut HackToGetLifeTimeOfKey, leaked: &'static str) {
    _ = hack.insert(leaked, leaked);
}
pub fn register_text_as_key(to_register: TextLike) -> &'static str {
    match to_register {
        TextLike::Static(already_static) => already_static,
        TextLike::Ref(to_may_leak) => {
            let mut lock = TEXT_REPOS.lock().unwrap();
            if let Some(already_leaked) = lock.get(&to_may_leak) {
                let already_leaked: &'static str = already_leaked;
                already_leaked
            } else {
                let leaked = Box::leak(to_may_leak.to_string().into_boxed_str());
                hack_insert(&mut lock, leaked);
                leaked
            }
        }
        TextLike::Owned(owned) => {
            let mut lock = TEXT_REPOS.lock().unwrap();
            if let Some(&already_leaked) = lock.get(owned.as_str()) {
                already_leaked
            } else {
                let leaked = Box::leak(owned.into_boxed_str());
                hack_insert(&mut lock, leaked);
                leaked
            }
        }
    }
}

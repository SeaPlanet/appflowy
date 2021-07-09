pub mod module;
pub use module::*;

use flowy_dispatch::prelude::*;
use module::build_modules;

pub struct FlowySDK {}

impl FlowySDK {
    pub fn init_log(directory: &str) { flowy_log::init_log("flowy", directory, "Debug").unwrap(); }

    pub fn init(path: &str) {
        tracing::trace!("🔥 Root path: {}", path);

        let config = ModuleConfig {
            root: path.to_string(),
        };

        EventDispatch::construct(|| build_modules(config));
    }
}

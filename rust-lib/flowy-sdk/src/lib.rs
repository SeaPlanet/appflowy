pub mod module;
pub use module::*;

use flowy_dispatch::prelude::*;
use module::build_modules;
pub struct FlowySDK {}

impl FlowySDK {
    pub fn init_log(directory: &str) { flowy_log::init_log("flowy", directory, "Debug").unwrap(); }

    pub fn init(path: &str) {
        log::info!("🔥 Start running");
        tracing::info!("🔥 Root path: {}", path);
        EventDispatch::construct(|| build_modules());
    }
}

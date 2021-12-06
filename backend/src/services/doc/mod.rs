#![allow(clippy::module_inception)]
pub(crate) use crud::*;
pub use router::*;

pub mod crud;
mod edit;
pub mod manager;
pub mod router;
mod ws_actor;

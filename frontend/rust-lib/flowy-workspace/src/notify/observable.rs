use dart_notify::DartNotifyBuilder;
use flowy_derive::ProtoBuf_Enum;
const OBSERVABLE_CATEGORY: &'static str = "Workspace";

// Opti: Using the Rust macro to generate the serde code automatically that can
// be use directly in flutter
#[derive(ProtoBuf_Enum, Debug)]
pub(crate) enum WorkspaceNotification {
    Unknown              = 0,
    UserCreateWorkspace  = 10,
    UserDeleteWorkspace  = 11,
    WorkspaceUpdated     = 12,
    WorkspaceListUpdated = 13,
    WorkspaceAppsChanged = 14,
    AppUpdated           = 21,
    AppViewsChanged      = 24,
    ViewUpdated          = 31,
    ViewDeleted          = 32,
    ViewRestored         = 33,
    UserUnauthorized     = 100,
    TrashUpdated         = 1000,
}

impl std::default::Default for WorkspaceNotification {
    fn default() -> Self { WorkspaceNotification::Unknown }
}

impl std::convert::Into<i32> for WorkspaceNotification {
    fn into(self) -> i32 { self as i32 }
}

#[tracing::instrument(level = "debug")]
pub(crate) fn send_dart_notification(id: &str, ty: WorkspaceNotification) -> DartNotifyBuilder {
    DartNotifyBuilder::new(id, ty, OBSERVABLE_CATEGORY)
}

#[tracing::instrument(level = "debug")]
pub(crate) fn send_anonymous_dart_notification(ty: WorkspaceNotification) -> DartNotifyBuilder {
    DartNotifyBuilder::new("", ty, OBSERVABLE_CATEGORY)
}

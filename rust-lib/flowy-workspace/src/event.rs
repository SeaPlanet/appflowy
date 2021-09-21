use flowy_derive::{Flowy_Event, ProtoBuf_Enum};
use strum_macros::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display, Hash, ProtoBuf_Enum, Flowy_Event)]
#[event_err = "WorkspaceError"]
pub enum WorkspaceEvent {
    #[event(input = "CreateWorkspaceRequest", output = "Workspace")]
    CreateWorkspace   = 0,

    #[event(output = "Workspace")]
    ReadCurWorkspace  = 1,

    #[event(input = "QueryWorkspaceRequest", output = "RepeatedWorkspace")]
    ReadWorkspaces    = 2,

    #[event(input = "DeleteWorkspaceRequest")]
    DeleteWorkspace   = 3,

    #[event(input = "QueryWorkspaceRequest", output = "Workspace")]
    OpenWorkspace     = 4,

    #[event(input = "QueryWorkspaceRequest", output = "RepeatedApp")]
    ReadWorkspaceApps = 5,

    #[event(input = "CreateAppRequest", output = "App")]
    CreateApp         = 101,

    #[event(input = "DeleteAppRequest")]
    DeleteApp         = 102,

    #[event(input = "QueryAppRequest", output = "App")]
    ReadApp           = 103,

    #[event(input = "UpdateAppRequest")]
    UpdateApp         = 104,

    #[event(input = "CreateViewRequest", output = "View")]
    CreateView        = 201,

    #[event(input = "QueryViewRequest", output = "View")]
    ReadView          = 202,

    #[event(input = "UpdateViewRequest")]
    UpdateView        = 203,

    #[event(input = "DeleteViewRequest")]
    DeleteView        = 204,

    #[event(input = "OpenViewRequest", output = "Doc")]
    OpenView          = 205,

    #[event(input = "ApplyChangesetRequest", output = "Doc")]
    ApplyChangeset    = 206,
}

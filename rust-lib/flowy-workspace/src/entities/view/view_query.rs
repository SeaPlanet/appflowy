use crate::{entities::view::parser::ViewId, errors::WorkspaceError};
use flowy_derive::ProtoBuf;
use flowy_document::entities::doc::DocIdentifier;
use std::convert::TryInto;

#[derive(Default, ProtoBuf)]
pub struct QueryViewRequest {
    #[pb(index = 1)]
    pub view_id: String,
}

impl QueryViewRequest {
    pub fn new(view_id: &str) -> Self {
        Self {
            view_id: view_id.to_owned(),
        }
    }
}

#[derive(Default, ProtoBuf, Clone, Debug)]
pub struct ViewIdentifier {
    #[pb(index = 1)]
    pub view_id: String,
}

impl ViewIdentifier {
    pub fn new(view_id: &str) -> Self {
        Self {
            view_id: view_id.to_owned(),
        }
    }
}

impl std::convert::From<String> for ViewIdentifier {
    fn from(view_id: String) -> Self { ViewIdentifier { view_id } }
}

impl std::convert::Into<DocIdentifier> for ViewIdentifier {
    fn into(self) -> DocIdentifier { DocIdentifier { doc_id: self.view_id } }
}

impl TryInto<ViewIdentifier> for QueryViewRequest {
    type Error = WorkspaceError;
    fn try_into(self) -> Result<ViewIdentifier, Self::Error> {
        let view_id = ViewId::parse(self.view_id)
            .map_err(|e| WorkspaceError::view_id().context(e))?
            .0;

        Ok(ViewIdentifier { view_id })
    }
}

#[derive(Default, ProtoBuf)]
pub struct OpenViewRequest {
    #[pb(index = 1)]
    pub view_id: String,
}

impl std::convert::TryInto<DocIdentifier> for OpenViewRequest {
    type Error = WorkspaceError;

    fn try_into(self) -> Result<DocIdentifier, Self::Error> {
        let view_id = ViewId::parse(self.view_id)
            .map_err(|e| WorkspaceError::view_id().context(e))?
            .0;
        Ok(DocIdentifier { doc_id: view_id })
    }
}

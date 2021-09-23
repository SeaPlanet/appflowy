use crate::entities::doc::Revision;
use bytes::Bytes;
use flowy_derive::{ProtoBuf, ProtoBuf_Enum};
use std::convert::TryInto;

#[derive(Debug, Clone, ProtoBuf_Enum, Eq, PartialEq, Hash)]
pub enum WsDataType {
    Command = 0,
    Delta   = 1,
}

impl std::default::Default for WsDataType {
    fn default() -> Self { WsDataType::Command }
}

#[derive(ProtoBuf, Default, Debug, Clone)]
pub struct WsDocumentData {
    #[pb(index = 1)]
    pub id: String,

    #[pb(index = 2)]
    pub ty: WsDataType,

    #[pb(index = 3)]
    pub data: Vec<u8>, // Delta
}

impl std::convert::From<Revision> for WsDocumentData {
    fn from(revision: Revision) -> Self {
        let id = revision.doc_id.clone();
        let bytes: Bytes = revision.try_into().unwrap();
        let data = bytes.to_vec();
        Self {
            id,
            ty: WsDataType::Delta,
            data,
        }
    }
}

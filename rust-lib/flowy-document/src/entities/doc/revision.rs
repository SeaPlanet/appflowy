use crate::services::util::md5;
use flowy_derive::{ProtoBuf, ProtoBuf_Enum};

#[derive(Debug, ProtoBuf_Enum, Clone, Eq, PartialEq)]
pub enum RevType {
    Local  = 0,
    Remote = 1,
}

impl std::default::Default for RevType {
    fn default() -> Self { RevType::Local }
}

#[derive(Debug, Clone, Default, ProtoBuf)]
pub struct Revision {
    #[pb(index = 1)]
    pub base_rev_id: i64,

    #[pb(index = 2)]
    pub rev_id: i64,

    #[pb(index = 3)]
    pub delta_data: Vec<u8>,

    #[pb(index = 4)]
    pub md5: String,

    #[pb(index = 5)]
    pub doc_id: String,

    #[pb(index = 6)]
    pub ty: RevType,
}

impl Revision {
    pub fn new(base_rev_id: i64, rev_id: i64, delta_data: Vec<u8>, doc_id: &str, ty: RevType) -> Revision {
        let md5 = md5(&delta_data);
        let doc_id = doc_id.to_owned();
        Self {
            base_rev_id,
            rev_id,
            delta_data,
            md5,
            doc_id,
            ty,
        }
    }
}

#[derive(Debug, Clone, Default, ProtoBuf)]
pub struct RevisionRange {
    #[pb(index = 1)]
    pub doc_id: String,

    #[pb(index = 2)]
    pub from_rev_id: i64,

    #[pb(index = 3)]
    pub to_rev_id: i64,
}

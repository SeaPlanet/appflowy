use flowy_derive::ProtoBuf;

#[derive(PartialEq, ProtoBuf, Default, Debug, Clone)]
pub struct TrashIdentifier {
    #[pb(index = 1)]
    pub id: String,
}

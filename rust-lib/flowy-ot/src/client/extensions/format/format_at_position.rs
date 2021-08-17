use crate::{
    client::extensions::FormatExt,
    core::{Attribute, AttributeKey, CharMetric, Delta, DeltaBuilder, DeltaIter, Interval},
};

pub struct FormatLinkAtCaretPositionExt {}

impl FormatExt for FormatLinkAtCaretPositionExt {
    fn ext_name(&self) -> &str { "FormatLinkAtCaretPositionExt" }

    fn apply(&self, delta: &Delta, interval: Interval, attribute: &Attribute) -> Option<Delta> {
        if attribute.key != AttributeKey::Link || interval.size() != 0 {
            return None;
        }

        let mut iter = DeltaIter::from_offset(delta, interval.start);
        let (before, after) = (iter.last_op_before_index(interval.size()), iter.next_op());
        let mut start = interval.end;
        let mut retain = 0;

        if let Some(before) = before {
            if before.contain_attribute(attribute) {
                start -= before.len();
                retain += before.len();
            }
        }

        if let Some(after) = after {
            if after.contain_attribute(attribute) {
                if retain != 0 {
                    retain += after.len();
                }
            }
        }

        if retain == 0 {
            return None;
        }

        Some(
            DeltaBuilder::new()
                .retain(start)
                .retain_with_attributes(retain, (attribute.clone()).into())
                .build(),
        )
    }
}

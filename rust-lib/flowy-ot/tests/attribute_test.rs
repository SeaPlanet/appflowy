pub mod helper;

use crate::helper::{TestOp::*, *};
use flowy_ot::{
    client::{FlowyDoc, PlainDoc},
    core::{Interval, NEW_LINE, WHITESPACE},
};

#[test]
fn attributes_bold_added() {
    let ops = vec![
        Insert(0, "123456", 0),
        Bold(0, Interval::new(3, 5), true),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"123"},
            {"insert":"45","attributes":{"bold":"true"}},
            {"insert":"6"}
            ]"#,
        ),
    ];
    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_bold_added_and_invert_all() {
    let ops = vec![
        Insert(0, "123", 0),
        Bold(0, Interval::new(0, 3), true),
        AssertOpsJson(0, r#"[{"insert":"123","attributes":{"bold":"true"}}]"#),
        Bold(0, Interval::new(0, 3), false),
        AssertOpsJson(0, r#"[{"insert":"123"}]"#),
    ];
    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_bold_added_and_invert_partial_suffix() {
    let ops = vec![
        Insert(0, "1234", 0),
        Bold(0, Interval::new(0, 4), true),
        AssertOpsJson(0, r#"[{"insert":"1234","attributes":{"bold":"true"}}]"#),
        Bold(0, Interval::new(2, 4), false),
        AssertOpsJson(0, r#"[{"insert":"12","attributes":{"bold":"true"}},{"insert":"34"}]"#),
    ];
    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_bold_added_and_invert_partial_suffix2() {
    let ops = vec![
        Insert(0, "1234", 0),
        Bold(0, Interval::new(0, 4), true),
        AssertOpsJson(0, r#"[{"insert":"1234","attributes":{"bold":"true"}}]"#),
        Bold(0, Interval::new(2, 4), false),
        AssertOpsJson(0, r#"[{"insert":"12","attributes":{"bold":"true"}},{"insert":"34"}]"#),
        Bold(0, Interval::new(2, 4), true),
        AssertOpsJson(0, r#"[{"insert":"1234","attributes":{"bold":"true"}}]"#),
    ];
    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_bold_added_with_new_line() {
    let ops = vec![
        Insert(0, "123456", 0),
        Bold(0, Interval::new(0, 6), true),
        AssertOpsJson(0, r#"[{"insert":"123456","attributes":{"bold":"true"}},{"insert":"\n"}]"#),
        Insert(0, "\n", 3),
        AssertOpsJson(
            0,
            r#"[{"insert":"123","attributes":{"bold":"true"}},{"insert":"\n"},{"insert":"456","attributes":{"bold":"true"}},{"insert":"\n"}]"#,
        ),
        Insert(0, "\n", 4),
        AssertOpsJson(
            0,
            r#"[{"insert":"123","attributes":{"bold":"true"}},{"insert":"\n\n"},{"insert":"456","attributes":{"bold":"true"}},{"insert":"\n"}]"#,
        ),
        Insert(0, "a", 4),
        AssertOpsJson(
            0,
            r#"[{"insert":"123","attributes":{"bold":"true"}},{"insert":"\na\n"},{"insert":"456","attributes":{"bold":"true"}},{"insert":"\n"}]"#,
        ),
    ];
    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_bold_added_and_invert_partial_prefix() {
    let ops = vec![
        Insert(0, "1234", 0),
        Bold(0, Interval::new(0, 4), true),
        AssertOpsJson(0, r#"[{"insert":"1234","attributes":{"bold":"true"}}]"#),
        Bold(0, Interval::new(0, 2), false),
        AssertOpsJson(0, r#"[{"insert":"12"},{"insert":"34","attributes":{"bold":"true"}}]"#),
    ];
    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_bold_added_consecutive() {
    let ops = vec![
        Insert(0, "1234", 0),
        Bold(0, Interval::new(0, 1), true),
        AssertOpsJson(0, r#"[{"insert":"1","attributes":{"bold":"true"}},{"insert":"234"}]"#),
        Bold(0, Interval::new(1, 2), true),
        AssertOpsJson(0, r#"[{"insert":"12","attributes":{"bold":"true"}},{"insert":"34"}]"#),
    ];
    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_bold_added_italic() {
    let ops = vec![
        Insert(0, "1234", 0),
        Bold(0, Interval::new(0, 4), true),
        Italic(0, Interval::new(0, 4), true),
        AssertOpsJson(
            0,
            r#"[{"insert":"1234","attributes":{"italic":"true","bold":"true"}},{"insert":"\n"}]"#,
        ),
        Insert(0, "5678", 4),
        AssertOpsJson(
            0,
            r#"[{"insert":"12345678","attributes":{"bold":"true","italic":"true"}},{"insert":"\n"}]"#,
        ),
    ];
    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_bold_added_italic2() {
    let ops = vec![
        Insert(0, "123456", 0),
        Bold(0, Interval::new(0, 6), true),
        AssertOpsJson(0, r#"[{"insert":"123456","attributes":{"bold":"true"}}]"#),
        Italic(0, Interval::new(0, 2), true),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"12","attributes":{"italic":"true","bold":"true"}},
            {"insert":"3456","attributes":{"bold":"true"}}]
            "#,
        ),
        Italic(0, Interval::new(4, 6), true),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"12","attributes":{"italic":"true","bold":"true"}},
            {"insert":"34","attributes":{"bold":"true"}},
            {"insert":"56","attributes":{"italic":"true","bold":"true"}}]
            "#,
        ),
    ];

    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_bold_added_italic3() {
    let ops = vec![
        Insert(0, "123456789", 0),
        Bold(0, Interval::new(0, 5), true),
        Italic(0, Interval::new(0, 2), true),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"12","attributes":{"bold":"true","italic":"true"}},
            {"insert":"345","attributes":{"bold":"true"}},{"insert":"6789"}]
            "#,
        ),
        Italic(0, Interval::new(2, 4), true),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"1234","attributes":{"bold":"true","italic":"true"}},
            {"insert":"5","attributes":{"bold":"true"}},
            {"insert":"6789"}]
            "#,
        ),
        Bold(0, Interval::new(7, 9), true),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"1234","attributes":{"bold":"true","italic":"true"}},
            {"insert":"5","attributes":{"bold":"true"}},
            {"insert":"67"},
            {"insert":"89","attributes":{"bold":"true"}}]
            "#,
        ),
    ];

    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_bold_added_italic_delete() {
    let ops = vec![
        Insert(0, "123456789", 0),
        Bold(0, Interval::new(0, 5), true),
        Italic(0, Interval::new(0, 2), true),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"12","attributes":{"italic":"true","bold":"true"}},
            {"insert":"345","attributes":{"bold":"true"}},{"insert":"6789"}]
            "#,
        ),
        Italic(0, Interval::new(2, 4), true),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"1234","attributes":{"bold":"true","italic":"true"}}
            ,{"insert":"5","attributes":{"bold":"true"}},{"insert":"6789"}]"#,
        ),
        Bold(0, Interval::new(7, 9), true),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"1234","attributes":{"bold":"true","italic":"true"}},
            {"insert":"5","attributes":{"bold":"true"}},{"insert":"67"},
            {"insert":"89","attributes":{"bold":"true"}}]
            "#,
        ),
        Delete(0, Interval::new(0, 5)),
        AssertOpsJson(0, r#"[{"insert":"67"},{"insert":"89","attributes":{"bold":"true"}}]"#),
    ];

    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_merge_inserted_text_with_same_attribute() {
    let ops = vec![
        InsertBold(0, "123", Interval::new(0, 3)),
        AssertOpsJson(0, r#"[{"insert":"123","attributes":{"bold":"true"}}]"#),
        InsertBold(0, "456", Interval::new(3, 6)),
        AssertOpsJson(0, r#"[{"insert":"123456","attributes":{"bold":"true"}}]"#),
    ];
    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_compose_attr_attributes_with_attr_attributes_test() {
    let ops = vec![
        InsertBold(0, "123456", Interval::new(0, 6)),
        AssertOpsJson(0, r#"[{"insert":"123456","attributes":{"bold":"true"}}]"#),
        InsertBold(1, "7", Interval::new(0, 1)),
        AssertOpsJson(1, r#"[{"insert":"7","attributes":{"bold":"true"}}]"#),
        Transform(0, 1),
        AssertOpsJson(0, r#"[{"insert":"1234567","attributes":{"bold":"true"}}]"#),
        AssertOpsJson(1, r#"[{"insert":"1234567","attributes":{"bold":"true"}}]"#),
    ];

    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_compose_attr_attributes_with_attr_attributes_test2() {
    let ops = vec![
        Insert(0, "123456", 0),
        Bold(0, Interval::new(0, 6), true),
        Italic(0, Interval::new(0, 2), true),
        Italic(0, Interval::new(4, 6), true),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"12","attributes":{"bold":"true","italic":"true"}},
            {"insert":"34","attributes":{"bold":"true"}},
            {"insert":"56","attributes":{"italic":"true","bold":"true"}}]
            "#,
        ),
        InsertBold(1, "7", Interval::new(0, 1)),
        AssertOpsJson(1, r#"[{"insert":"7","attributes":{"bold":"true"}}]"#),
        Transform(0, 1),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"12","attributes":{"italic":"true","bold":"true"}},
            {"insert":"34","attributes":{"bold":"true"}},
            {"insert":"56","attributes":{"italic":"true","bold":"true"}},
            {"insert":"7","attributes":{"bold":"true"}}]
            "#,
        ),
        AssertOpsJson(
            1,
            r#"[
            {"insert":"12","attributes":{"italic":"true","bold":"true"}},
            {"insert":"34","attributes":{"bold":"true"}},
            {"insert":"56","attributes":{"italic":"true","bold":"true"}},
            {"insert":"7","attributes":{"bold":"true"}}]
            "#,
        ),
    ];

    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_compose_attr_attributes_with_no_attr_attributes_test() {
    let expected = r#"[{"insert":"123456","attributes":{"bold":"true"}},{"insert":"7"}]"#;

    let ops = vec![
        InsertBold(0, "123456", Interval::new(0, 6)),
        AssertOpsJson(0, r#"[{"insert":"123456","attributes":{"bold":"true"}}]"#),
        Insert(1, "7", 0),
        AssertOpsJson(1, r#"[{"insert":"7"}]"#),
        Transform(0, 1),
        AssertOpsJson(0, expected),
        AssertOpsJson(1, expected),
    ];
    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_replace_heading() {
    let ops = vec![
        InsertBold(0, "123456", Interval::new(0, 6)),
        AssertOpsJson(0, r#"[{"insert":"123456","attributes":{"bold":"true"}}]"#),
        Delete(0, Interval::new(0, 2)),
        AssertOpsJson(0, r#"[{"insert":"3456","attributes":{"bold":"true"}}]"#),
    ];

    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_replace_trailing() {
    let ops = vec![
        InsertBold(0, "123456", Interval::new(0, 6)),
        AssertOpsJson(0, r#"[{"insert":"123456","attributes":{"bold":"true"}}]"#),
        Delete(0, Interval::new(5, 6)),
        AssertOpsJson(0, r#"[{"insert":"12345","attributes":{"bold":"true"}}]"#),
    ];

    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_replace_middle() {
    let ops = vec![
        InsertBold(0, "123456", Interval::new(0, 6)),
        AssertOpsJson(0, r#"[{"insert":"123456","attributes":{"bold":"true"}}]"#),
        Delete(0, Interval::new(0, 2)),
        AssertOpsJson(0, r#"[{"insert":"3456","attributes":{"bold":"true"}}]"#),
        Delete(0, Interval::new(2, 4)),
        AssertOpsJson(0, r#"[{"insert":"34","attributes":{"bold":"true"}}]"#),
    ];

    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_replace_all() {
    let ops = vec![
        InsertBold(0, "123456", Interval::new(0, 6)),
        AssertOpsJson(0, r#"[{"insert":"123456","attributes":{"bold":"true"}}]"#),
        Delete(0, Interval::new(0, 6)),
        AssertOpsJson(0, r#"[]"#),
    ];

    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_replace_with_text() {
    let ops = vec![
        InsertBold(0, "123456", Interval::new(0, 6)),
        AssertOpsJson(0, r#"[{"insert":"123456","attributes":{"bold":"true"}}]"#),
        Replace(0, Interval::new(0, 3), "ab"),
        AssertOpsJson(0, r#"[{"insert":"ab"},{"insert":"456","attributes":{"bold":"true"}}]"#),
    ];

    TestBuilder::new().run_script::<PlainDoc>(ops);
}

#[test]
fn attributes_header_insert_newline_at_middle() {
    let ops = vec![
        Insert(0, "123456", 0),
        Header(0, Interval::new(0, 6), 1),
        AssertOpsJson(0, r#"[{"insert":"123456"},{"insert":"\n","attributes":{"header":"1"}}]"#),
        Insert(0, "\n", 3),
        AssertOpsJson(
            0,
            r#"[{"insert":"123"},{"insert":"\n","attributes":{"header":"1"}},{"insert":"456"},{"insert":"\n","attributes":{"header":"1"}}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_header_insert_double_newline_at_middle() {
    let ops = vec![
        Insert(0, "123456", 0),
        Header(0, Interval::new(0, 6), 1),
        Insert(0, "\n", 3),
        AssertOpsJson(
            0,
            r#"[{"insert":"123"},{"insert":"\n","attributes":{"header":"1"}},{"insert":"456"},{"insert":"\n","attributes":{"header":"1"}}]"#,
        ),
        Insert(0, "\n", 4),
        AssertOpsJson(
            0,
            r#"[{"insert":"123"},{"insert":"\n\n","attributes":{"header":"1"}},{"insert":"456"},{"insert":"\n","attributes":{"header":"1"}}]"#,
        ),
        Insert(0, "\n", 4),
        AssertOpsJson(
            0,
            r#"[{"insert":"123"},{"insert":"\n\n","attributes":{"header":"1"}},{"insert":"\n456"},{"insert":"\n","attributes":{"header":"1"}}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_header_insert_newline_at_trailing() {
    let ops = vec![
        Insert(0, "123456", 0),
        Header(0, Interval::new(0, 6), 1),
        Insert(0, "\n", 6),
        AssertOpsJson(
            0,
            r#"[{"insert":"123456"},{"insert":"\n","attributes":{"header":"1"}},{"insert":"\n"}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_header_insert_double_newline_at_trailing() {
    let ops = vec![
        Insert(0, "123456", 0),
        Header(0, Interval::new(0, 6), 1),
        Insert(0, "\n", 6),
        Insert(0, "\n", 7),
        AssertOpsJson(
            0,
            r#"[{"insert":"123456"},{"insert":"\n","attributes":{"header":"1"}},{"insert":"\n\n"}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_link_added() {
    let ops = vec![
        Insert(0, "123456", 0),
        Link(0, Interval::new(0, 6), "https://appflowy.io"),
        AssertOpsJson(
            0,
            r#"[{"insert":"123456","attributes":{"link":"https://appflowy.io"}},{"insert":"\n"}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_link_format_with_bold() {
    let ops = vec![
        Insert(0, "123456", 0),
        Link(0, Interval::new(0, 6), "https://appflowy.io"),
        Bold(0, Interval::new(0, 3), true),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"123","attributes":{"bold":"true","link":"https://appflowy.io"}},
            {"insert":"456","attributes":{"link":"https://appflowy.io"}},
            {"insert":"\n"}]
            "#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_link_insert_char_at_head() {
    let ops = vec![
        Insert(0, "123456", 0),
        Link(0, Interval::new(0, 6), "https://appflowy.io"),
        AssertOpsJson(
            0,
            r#"[{"insert":"123456","attributes":{"link":"https://appflowy.io"}},{"insert":"\n"}]"#,
        ),
        Insert(0, "a", 0),
        AssertOpsJson(
            0,
            r#"[{"insert":"a"},{"insert":"123456","attributes":{"link":"https://appflowy.io"}},{"insert":"\n"}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_link_insert_char_at_middle() {
    let ops = vec![
        Insert(0, "1256", 0),
        Link(0, Interval::new(0, 4), "https://appflowy.io"),
        Insert(0, "34", 2),
        AssertOpsJson(
            0,
            r#"[{"insert":"123456","attributes":{"link":"https://appflowy.io"}},{"insert":"\n"}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_link_insert_char_at_trailing() {
    let ops = vec![
        Insert(0, "123456", 0),
        Link(0, Interval::new(0, 6), "https://appflowy.io"),
        AssertOpsJson(
            0,
            r#"[{"insert":"123456","attributes":{"link":"https://appflowy.io"}},{"insert":"\n"}]"#,
        ),
        Insert(0, "a", 6),
        AssertOpsJson(
            0,
            r#"[{"insert":"123456","attributes":{"link":"https://appflowy.io"}},{"insert":"a\n"}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_link_insert_newline_at_middle() {
    let ops = vec![
        Insert(0, "123456", 0),
        Link(0, Interval::new(0, 6), "https://appflowy.io"),
        Insert(0, NEW_LINE, 3),
        AssertOpsJson(
            0,
            r#"[{"insert":"123","attributes":{"link":"https://appflowy.io"}},{"insert":"\n"},{"insert":"456","attributes":{"link":"https://appflowy.io"}},{"insert":"\n"}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_link_auto_format() {
    let site = "https://appflowy.io";
    let ops = vec![
        Insert(0, site, 0),
        AssertOpsJson(0, r#"[{"insert":"https://appflowy.io\n"}]"#),
        Insert(0, WHITESPACE, site.len()),
        AssertOpsJson(
            0,
            r#"[{"insert":"https://appflowy.io","attributes":{"link":"https://appflowy.io/"}},{"insert":" \n"}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_link_auto_format_exist() {
    let site = "https://appflowy.io";
    let ops = vec![
        Insert(0, site, 0),
        Link(0, Interval::new(0, site.len()), site),
        Insert(0, WHITESPACE, site.len()),
        AssertOpsJson(
            0,
            r#"[{"insert":"https://appflowy.io","attributes":{"link":"https://appflowy.io/"}},{"insert":" \n"}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_link_auto_format_exist2() {
    let site = "https://appflowy.io";
    let ops = vec![
        Insert(0, site, 0),
        Link(0, Interval::new(0, site.len() / 2), site),
        Insert(0, WHITESPACE, site.len()),
        AssertOpsJson(
            0,
            r#"[{"insert":"https://a","attributes":{"link":"https://appflowy.io"}},{"insert":"ppflowy.io \n"}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_bullet_added() {
    let ops = vec![
        Insert(0, "12", 0),
        Bullet(0, Interval::new(0, 1), true),
        AssertOpsJson(0, r#"[{"insert":"12"},{"insert":"\n","attributes":{"list":"bullet"}}]"#),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_bullet_added_2() {
    let ops = vec![
        Insert(0, "1", 0),
        Bullet(0, Interval::new(0, 1), true),
        AssertOpsJson(0, r#"[{"insert":"1"},{"insert":"\n","attributes":{"list":"bullet"}}]"#),
        Insert(0, NEW_LINE, 1),
        AssertOpsJson(0, r#"[{"insert":"1"},{"insert":"\n\n","attributes":{"list":"bullet"}}]"#),
        Insert(0, "2", 2),
        AssertOpsJson(
            0,
            r#"[{"insert":"1"},{"insert":"\n","attributes":{"list":"bullet"}},{"insert":"2"},{"insert":"\n","attributes":{"list":"bullet"}}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_bullet_remove_partial() {
    let ops = vec![
        Insert(0, "1", 0),
        Bullet(0, Interval::new(0, 1), true),
        Insert(0, NEW_LINE, 1),
        Insert(0, "2", 2),
        Bullet(0, Interval::new(2, 3), false),
        AssertOpsJson(
            0,
            r#"[{"insert":"1"},{"insert":"\n","attributes":{"list":"bullet"}},{"insert":"2\n"}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_bullet_auto_exit() {
    let ops = vec![
        Insert(0, "1", 0),
        Bullet(0, Interval::new(0, 1), true),
        Insert(0, NEW_LINE, 1),
        Insert(0, NEW_LINE, 2),
        AssertOpsJson(
            0,
            r#"[{"insert":"1"},{"insert":"\n","attributes":{"list":"bullet"}},{"insert":"\n"}]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_preserve_block_when_insert_newline_inside() {
    let ops = vec![
        Insert(0, "12", 0),
        Bullet(0, Interval::new(0, 2), true),
        Insert(0, NEW_LINE, 2),
        AssertOpsJson(0, r#"[{"insert":"12"},{"insert":"\n\n","attributes":{"list":"bullet"}}]"#),
        Insert(0, "34", 3),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"12"},{"insert":"\n","attributes":{"list":"bullet"}},
            {"insert":"34"},{"insert":"\n","attributes":{"list":"bullet"}}
            ]"#,
        ),
        Insert(0, NEW_LINE, 3),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"12"},{"insert":"\n\n","attributes":{"list":"bullet"}},
            {"insert":"34"},{"insert":"\n","attributes":{"list":"bullet"}}
            ]"#,
        ),
        Insert(0, "ab", 3),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"12"},{"insert":"\n","attributes":{"list":"bullet"}},
            {"insert":"ab"},{"insert":"\n","attributes":{"list":"bullet"}},
            {"insert":"34"},{"insert":"\n","attributes":{"list":"bullet"}}
            ]"#,
        ),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_preserve_header_format_on_merge() {
    let ops = vec![
        Insert(0, "123456", 0),
        Header(0, Interval::new(0, 6), 1),
        Insert(0, NEW_LINE, 3),
        AssertOpsJson(
            0,
            r#"[{"insert":"123"},{"insert":"\n","attributes":{"header":"1"}},{"insert":"456"},{"insert":"\n","attributes":{"header":"1"}}]"#,
        ),
        Delete(0, Interval::new(3, 4)),
        AssertOpsJson(0, r#"[{"insert":"123456"},{"insert":"\n","attributes":{"header":"1"}}]"#),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

#[test]
fn attributes_preserve_list_format_on_merge() {
    let ops = vec![
        Insert(0, "123456", 0),
        Bullet(0, Interval::new(0, 6), true),
        Insert(0, NEW_LINE, 3),
        AssertOpsJson(
            0,
            r#"[{"insert":"123"},{"insert":"\n","attributes":{"list":"bullet"}},{"insert":"456"},{"insert":"\n","attributes":{"list":"bullet"}}]"#,
        ),
        Delete(0, Interval::new(3, 4)),
        AssertOpsJson(0, r#"[{"insert":"123456"},{"insert":"\n","attributes":{"list":"bullet"}}]"#),
    ];

    TestBuilder::new().run_script::<FlowyDoc>(ops);
}

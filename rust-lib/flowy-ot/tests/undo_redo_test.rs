pub mod helper;

use crate::helper::{TestOp::*, *};
use flowy_ot::{
    client::RECORD_THRESHOLD,
    core::{Interval, NEW_LINE, WHITESPACE},
};

#[test]
fn history_insert_undo() {
    let ops = vec![
        Insert(0, "123", 0),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"\n"}]"#),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_insert_undo_with_lagging() {
    let ops = vec![
        Insert(0, "123", 0),
        Wait(RECORD_THRESHOLD),
        Insert(0, "456", 0),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"123\n"}]"#),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"\n"}]"#),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_insert_redo() {
    let ops = vec![
        Insert(0, "123", 0),
        AssertOpsJson(0, r#"[{"insert":"123\n"}]"#),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"\n"}]"#),
        Redo(0),
        AssertOpsJson(0, r#"[{"insert":"123\n"}]"#),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_insert_redo_with_lagging() {
    let ops = vec![
        Insert(0, "123", 0),
        Wait(RECORD_THRESHOLD),
        Insert(0, "456", 3),
        Wait(RECORD_THRESHOLD),
        AssertStr(0, "123456\n"),
        AssertOpsJson(0, r#"[{"insert":"123456\n"}]"#),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"123\n"}]"#),
        Redo(0),
        AssertOpsJson(0, r#"[{"insert":"123456\n"}]"#),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"123\n"}]"#),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_bold_undo() {
    let ops = vec![
        Insert(0, "123", 0),
        Bold(0, Interval::new(0, 3), true),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"\n"}]"#),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_bold_undo_with_lagging() {
    let ops = vec![
        Insert(0, "123", 0),
        Wait(RECORD_THRESHOLD),
        Bold(0, Interval::new(0, 3), true),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"123\n"}]"#),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_bold_redo() {
    let ops = vec![
        Insert(0, "123", 0),
        Bold(0, Interval::new(0, 3), true),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"\n"}]"#),
        Redo(0),
        AssertOpsJson(
            0,
            r#" [{"insert":"123","attributes":{"bold":"true"}},{"insert":"\n"}]"#,
        ),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_bold_redo_with_lagging() {
    let ops = vec![
        Insert(0, "123", 0),
        Wait(RECORD_THRESHOLD),
        Bold(0, Interval::new(0, 3), true),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"123\n"}]"#),
        Redo(0),
        AssertOpsJson(
            0,
            r#"[{"insert":"123","attributes":{"bold":"true"}},{"insert":"\n"}]"#,
        ),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_delete_undo() {
    let ops = vec![
        Insert(0, "123", 0),
        AssertOpsJson(0, r#"[{"insert":"123"}]"#),
        Delete(0, Interval::new(0, 3)),
        AssertOpsJson(0, r#"[]"#),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"123"}]"#),
    ];
    OpTester::new().run_script(ops);
}

#[test]
fn history_delete_undo_2() {
    let ops = vec![
        Insert(0, "123", 0),
        Bold(0, Interval::new(0, 3), true),
        Delete(0, Interval::new(0, 1)),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"23","attributes":{"bold":"true"}},
            {"insert":"\n"}]
            "#,
        ),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"\n"}]"#),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_delete_undo_with_lagging() {
    let ops = vec![
        Insert(0, "123", 0),
        Wait(RECORD_THRESHOLD),
        Bold(0, Interval::new(0, 3), true),
        Wait(RECORD_THRESHOLD),
        Delete(0, Interval::new(0, 1)),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"23","attributes":{"bold":"true"}},
            {"insert":"\n"}]
            "#,
        ),
        Undo(0),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"123","attributes":{"bold":"true"}},
            {"insert":"\n"}]
            "#,
        ),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_delete_redo() {
    let ops = vec![
        Insert(0, "123", 0),
        Wait(RECORD_THRESHOLD),
        Delete(0, Interval::new(0, 3)),
        AssertOpsJson(0, r#"[{"insert":"\n"}]"#),
        Undo(0),
        Redo(0),
        AssertOpsJson(0, r#"[{"insert":"\n"}]"#),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_replace_undo() {
    let ops = vec![
        Insert(0, "123", 0),
        Bold(0, Interval::new(0, 3), true),
        Replace(0, Interval::new(0, 2), "ab"),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"ab"},
            {"insert":"3","attributes":{"bold":"true"}},{"insert":"\n"}]
            "#,
        ),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"\n"}]"#),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_replace_undo_with_lagging() {
    let ops = vec![
        Insert(0, "123", 0),
        Wait(RECORD_THRESHOLD),
        Bold(0, Interval::new(0, 3), true),
        Wait(RECORD_THRESHOLD),
        Replace(0, Interval::new(0, 2), "ab"),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"ab"},
            {"insert":"3","attributes":{"bold":"true"}},{"insert":"\n"}]
            "#,
        ),
        Undo(0),
        AssertOpsJson(
            0,
            r#"[{"insert":"123","attributes":{"bold":"true"}},{"insert":"\n"}]"#,
        ),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_replace_redo() {
    let ops = vec![
        Insert(0, "123", 0),
        Bold(0, Interval::new(0, 3), true),
        Replace(0, Interval::new(0, 2), "ab"),
        Undo(0),
        Redo(0),
        AssertOpsJson(
            0,
            r#"[
            {"insert":"ab"},
            {"insert":"3","attributes":{"bold":"true"}},{"insert":"\n"}]
            "#,
        ),
    ];
    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_header_added_undo() {
    let ops = vec![
        Insert(0, "123456", 0),
        Header(0, Interval::new(0, 6), 1, true),
        Insert(0, "\n", 3),
        Insert(0, "\n", 4),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"\n"}]"#),
        Redo(0),
        AssertOpsJson(
            0,
            r#"[{"insert":"123"},{"insert":"\n\n","attributes":{"header":"1"}},{"insert":"456"},{"insert":"\n","attributes":{"header":"1"}}]"#,
        ),
    ];

    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_link_added_undo() {
    let site = "https://appflowy.io";
    let ops = vec![
        Insert(0, site, 0),
        Wait(RECORD_THRESHOLD),
        Link(0, Interval::new(0, site.len()), site, true),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"https://appflowy.io\n"}]"#),
        Redo(0),
        AssertOpsJson(
            0,
            r#"[{"insert":"https://appflowy.io","attributes":{"link":"https://appflowy.io"}},{"insert":"\n"}]"#,
        ),
    ];

    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_link_auto_format_undo_with_lagging() {
    let site = "https://appflowy.io";
    let ops = vec![
        Insert(0, site, 0),
        AssertOpsJson(0, r#"[{"insert":"https://appflowy.io\n"}]"#),
        Wait(RECORD_THRESHOLD),
        Insert(0, WHITESPACE, site.len()),
        AssertOpsJson(
            0,
            r#"[{"insert":"https://appflowy.io","attributes":{"link":"https://appflowy.io/"}},{"insert":" \n"}]"#,
        ),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"https://appflowy.io\n"}]"#),
    ];

    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_bullet_undo() {
    let ops = vec![
        Insert(0, "1", 0),
        Bullet(0, Interval::new(0, 1), true),
        Insert(0, NEW_LINE, 1),
        Insert(0, "2", 2),
        AssertOpsJson(
            0,
            r#"[{"insert":"1"},{"insert":"\n","attributes":{"bullet":"true"}},{"insert":"2"},{"insert":"\n","attributes":{"bullet":"true"}}]"#,
        ),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"\n"}]"#),
        Redo(0),
        AssertOpsJson(
            0,
            r#"[{"insert":"1"},{"insert":"\n","attributes":{"bullet":"true"}},{"insert":"2"},{"insert":"\n","attributes":{"bullet":"true"}}]"#,
        ),
    ];

    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_bullet_undo_with_lagging() {
    let ops = vec![
        Insert(0, "1", 0),
        Bullet(0, Interval::new(0, 1), true),
        Wait(RECORD_THRESHOLD),
        Insert(0, NEW_LINE, 1),
        Insert(0, "2", 2),
        Wait(RECORD_THRESHOLD),
        AssertOpsJson(
            0,
            r#"[{"insert":"1"},{"insert":"\n","attributes":{"bullet":"true"}},{"insert":"2"},{"insert":"\n","attributes":{"bullet":"true"}}]"#,
        ),
        Undo(0),
        AssertOpsJson(
            0,
            r#"[{"insert":"1"},{"insert":"\n","attributes":{"bullet":"true"}}]"#,
        ),
        Undo(0),
        AssertOpsJson(0, r#"[{"insert":"\n"}]"#),
        Redo(0),
        Redo(0),
        AssertOpsJson(
            0,
            r#"[{"insert":"1"},{"insert":"\n","attributes":{"bullet":"true"}},{"insert":"2"},{"insert":"\n","attributes":{"bullet":"true"}}]"#,
        ),
    ];

    OpTester::new().run_script_with_newline(ops);
}

#[test]
fn history_undo_attribute_on_merge_between_line() {
    let ops = vec![
        Insert(0, "123456", 0),
        Bullet(0, Interval::new(0, 6), true),
        Wait(RECORD_THRESHOLD),
        Insert(0, NEW_LINE, 3),
        Wait(RECORD_THRESHOLD),
        AssertOpsJson(
            0,
            r#"[{"insert":"123"},{"insert":"\n","attributes":{"bullet":"true"}},{"insert":"456"},{"insert":"\n","attributes":{"bullet":"true"}}]"#,
        ),
        Delete(0, Interval::new(3, 4)), // delete the newline
        AssertOpsJson(
            0,
            r#"[{"insert":"123456"},{"insert":"\n","attributes":{"bullet":"true"}}]"#,
        ),
        Undo(0),
        AssertOpsJson(
            0,
            r#"[{"insert":"123"},{"insert":"\n","attributes":{"bullet":"true"}},{"insert":"456"},{"insert":"\n","attributes":{"bullet":"true"}}]"#,
        ),
    ];

    OpTester::new().run_script_with_newline(ops);
}

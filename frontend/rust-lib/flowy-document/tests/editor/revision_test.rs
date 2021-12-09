use flowy_test::editor::{EditorScript::*, *};
use lib_ot::revision::RevState;

#[tokio::test]
async fn doc_rev_state_test1() {
    let scripts = vec![
        InsertText("123", 0),
        AssertCurrentRevId(1),
        AssertRevisionState(1, RevState::Local),
        SimulateAckedMessage(1),
        AssertRevisionState(1, RevState::Acked),
        AssertNextSendingRevision(None),
        AssertJson(r#"[{"insert":"123\n"}]"#),
    ];
    EditorTest::new().await.run_scripts(scripts).await;
}

#[tokio::test]
async fn doc_rev_state_test2() {
    let scripts = vec![
        InsertText("1", 0),
        InsertText("2", 1),
        InsertText("3", 2),
        AssertCurrentRevId(3),
        AssertRevisionState(1, RevState::Local),
        AssertRevisionState(2, RevState::Local),
        AssertRevisionState(3, RevState::Local),
        SimulateAckedMessage(1),
        AssertRevisionState(1, RevState::Acked),
        AssertNextSendingRevision(Some(2)),
        SimulateAckedMessage(2),
        AssertRevisionState(2, RevState::Acked),
        //
        AssertNextSendingRevision(Some(3)),
        AssertRevisionState(3, RevState::Local),
        AssertJson(r#"[{"insert":"123\n"}]"#),
    ];
    EditorTest::new().await.run_scripts(scripts).await;
}

use crate::helper::ViewTest;
use flowy_document_infra::entities::doc::DocIdentifier;
use flowy_workspace_infra::entities::view::ViewIdentifiers;

#[actix_rt::test]
async fn doc_read() {
    let test = ViewTest::new().await;

    let params = DocIdentifier {
        doc_id: test.view.id.clone(),
    };

    let doc = test.server.read_doc(params).await;
    assert_eq!(doc.is_some(), true);
}

#[actix_rt::test]
async fn doc_delete() {
    let test = ViewTest::new().await;
    let delete_params = ViewIdentifiers {
        view_ids: vec![test.view.id.clone()],
    };
    test.server.delete_view(delete_params).await;

    let params = DocIdentifier {
        doc_id: test.view.id.clone(),
    };
    let doc = test.server.read_doc(params).await;
    assert_eq!(doc.is_none(), true);
}

use crate::helper::*;
use flowy_test::TestSDKBuilder;

#[test]
fn file_create_test() {
    let sdk = TestSDKBuilder::new().sign_up().build();
    let doc_desc = create_doc(&sdk, "hello world", "flutter ❤️ rust", "123");
    dbg!(&doc_desc);

    let doc = read_doc_data(&sdk, &doc_desc.id, &doc_desc.path);
    assert_eq!(doc.text, "123".to_owned());
}

#[test]
fn file_update_text_test() {
    let sdk = TestSDKBuilder::new().sign_up().build();
    let doc_desc = create_doc(&sdk, "hello world", "flutter ❤️ rust", "");
    dbg!(&doc_desc);

    let content = "😁😁😁😁😁😁😁😁😁😁".to_owned();
    save_doc(&sdk, &doc_desc, &content);

    let doc = read_doc_data(&sdk, &doc_desc.id, &doc_desc.path);
    assert_eq!(doc.text, content);
}

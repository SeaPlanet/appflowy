use crate::helper::*;

use flowy_workspace::entities::{
    app::{QueryAppRequest, UpdateAppRequest},
    view::*,
};

#[test]
fn app_create() {
    let workspace = create_workspace("Workspace", "").1;
    let app = create_app("App A", "AppFlowy Github Project", &workspace.id);
    dbg!(&app);
}

#[test]
#[should_panic]
fn app_delete() {
    let workspace = create_workspace("Workspace", "").1;
    let app = create_app("App A", "AppFlowy Github Project", &workspace.id);
    delete_app(&app.id);
    let query = QueryAppRequest::new(&app.id);
    let _ = read_app(query);
}

#[test]
fn app_read() {
    let workspace = create_workspace("Workspace", "").1;
    let app = create_app("App A", "AppFlowy Github Project", &workspace.id);
    let query = QueryAppRequest::new(&app.id);
    let app_from_db = read_app(query);
    assert_eq!(app_from_db, app);
}

#[test]
fn app_create_with_view() {
    let workspace = create_workspace("Workspace", "").1;
    let app = create_app("App A", "AppFlowy Github Project", &workspace.id);
    let request_a = CreateViewRequest {
        belong_to_id: app.id.clone(),
        name: "View A".to_string(),
        desc: "".to_string(),
        thumbnail: None,
        view_type: ViewType::Doc,
    };

    let request_b = CreateViewRequest {
        belong_to_id: app.id.clone(),
        name: "View B".to_string(),
        desc: "".to_string(),
        thumbnail: None,
        view_type: ViewType::Doc,
    };

    let view_a = create_view_with_request(request_a);
    let view_b = create_view_with_request(request_b);

    let query = QueryAppRequest::new(&app.id).set_read_views(true);
    let view_from_db = read_app(query);

    assert_eq!(view_from_db.belongings[0], view_a);
    assert_eq!(view_from_db.belongings[1], view_b);
}

#[test]
fn app_set_trash_flag() {
    let app_id = create_app_with_trash_flag();
    let query = QueryAppRequest::new(&app_id).set_is_trash(true);
    let _ = read_app(query);
}

#[test]
#[should_panic]
fn app_set_trash_flag_2() {
    let app_id = create_app_with_trash_flag();
    let query = QueryAppRequest::new(&app_id);
    let _ = read_app(query);
}

fn create_app_with_trash_flag() -> String {
    let workspace = create_workspace("Workspace", "").1;
    let app = create_app("App A", "AppFlowy Github Project", &workspace.id);
    let request = UpdateAppRequest {
        app_id: app.id.clone(),
        name: None,
        desc: None,
        color_style: None,
        is_trash: Some(true),
    };
    update_app(request);

    app.id
}

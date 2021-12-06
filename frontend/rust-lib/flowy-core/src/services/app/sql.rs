use crate::{
    entities::{
        app::{App, ColorStyle, UpdateAppParams},
        trash::{Trash, TrashType},
        view::RepeatedView,
    },
    services::workspace::sql::WorkspaceTable,
};
use diesel::sql_types::Binary;
use flowy_database::{
    prelude::*,
    schema::{app_table, app_table::dsl},
    SqliteConnection,
};
use serde::{Deserialize, Serialize, __private::TryFrom};
use std::convert::TryInto;

use crate::errors::WorkspaceError;

pub struct AppTableSql {}

impl AppTableSql {
    pub(crate) fn create_app(app_table: AppTable, conn: &SqliteConnection) -> Result<(), WorkspaceError> {
        match diesel_record_count!(app_table, &app_table.id, conn) {
            0 => diesel_insert_table!(app_table, &app_table, conn),
            _ => {
                let changeset = AppTableChangeset::from_table(app_table);
                diesel_update_table!(app_table, changeset, conn)
            },
        }
        Ok(())
    }

    pub(crate) fn update_app(changeset: AppTableChangeset, conn: &SqliteConnection) -> Result<(), WorkspaceError> {
        diesel_update_table!(app_table, changeset, conn);
        Ok(())
    }

    pub(crate) fn read_app(app_id: &str, conn: &SqliteConnection) -> Result<AppTable, WorkspaceError> {
        let filter = dsl::app_table.filter(app_table::id.eq(app_id)).into_boxed();
        let app_table = filter.first::<AppTable>(conn)?;
        Ok(app_table)
    }

    pub(crate) fn read_workspace_apps(
        workspace_id: &str,
        is_trash: bool,
        conn: &SqliteConnection,
    ) -> Result<Vec<AppTable>, WorkspaceError> {
        let app_table = dsl::app_table
            .filter(app_table::workspace_id.eq(workspace_id))
            .filter(app_table::is_trash.eq(is_trash))
            .order(app_table::create_time.asc())
            .load::<AppTable>(conn)?;

        Ok(app_table)
    }

    pub(crate) fn delete_app(app_id: &str, conn: &SqliteConnection) -> Result<AppTable, WorkspaceError> {
        let app_table = dsl::app_table
            .filter(app_table::id.eq(app_id))
            .first::<AppTable>(conn)?;
        diesel_delete_table!(app_table, app_id, conn);
        Ok(app_table)
    }

    // pub(crate) fn read_views_belong_to_app(
    //     &self,
    //     app_id: &str,
    // ) -> Result<Vec<ViewTable>, WorkspaceError> {
    //     let conn = self.database.db_connection()?;
    //
    //     let views = conn.immediate_transaction::<_, WorkspaceError, _>(|| {
    //         let app_table: AppTable = dsl::app_table
    //             .filter(app_table::id.eq(app_id))
    //             .first::<AppTable>(&*(conn))?;
    //         let views =
    // ViewTable::belonging_to(&app_table).load::<ViewTable>(&*conn)?;
    //         Ok(views)
    //     })?;
    //
    //     Ok(views)
    // }
}

#[derive(PartialEq, Clone, Debug, Queryable, Identifiable, Insertable, Associations)]
#[belongs_to(WorkspaceTable, foreign_key = "workspace_id")]
#[table_name = "app_table"]
pub(crate) struct AppTable {
    pub id: String,
    pub workspace_id: String, // equal to #[belongs_to(Workspace, foreign_key = "workspace_id")].
    pub name: String,
    pub desc: String,
    pub color_style: ColorStyleCol,
    pub last_view_id: Option<String>,
    pub modified_time: i64,
    pub create_time: i64,
    pub version: i64,
    pub is_trash: bool,
}

impl AppTable {
    pub fn new(app: App) -> Self {
        Self {
            id: app.id,
            workspace_id: app.workspace_id,
            name: app.name,
            desc: app.desc,
            color_style: ColorStyleCol::default(),
            last_view_id: None,
            modified_time: app.modified_time,
            create_time: app.create_time,
            version: 0,
            is_trash: false,
        }
    }
}

impl std::convert::From<AppTable> for Trash {
    fn from(table: AppTable) -> Self {
        Trash {
            id: table.id,
            name: table.name,
            modified_time: table.modified_time,
            create_time: table.create_time,
            ty: TrashType::App,
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default, FromSqlRow, AsExpression)]
#[sql_type = "Binary"]
pub(crate) struct ColorStyleCol {
    pub(crate) theme_color: String,
}

impl std::convert::From<ColorStyle> for ColorStyleCol {
    fn from(s: ColorStyle) -> Self {
        Self {
            theme_color: s.theme_color,
        }
    }
}

impl std::convert::TryInto<Vec<u8>> for &ColorStyleCol {
    type Error = String;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> { bincode::serialize(self).map_err(|e| format!("{:?}", e)) }
}

impl std::convert::TryFrom<&[u8]> for ColorStyleCol {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        bincode::deserialize(value).map_err(|e| format!("{:?}", e))
    }
}

impl_sql_binary_expression!(ColorStyleCol);

#[derive(AsChangeset, Identifiable, Default, Debug)]
#[table_name = "app_table"]
pub(crate) struct AppTableChangeset {
    pub id: String,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub is_trash: Option<bool>,
}

impl AppTableChangeset {
    pub(crate) fn new(params: UpdateAppParams) -> Self {
        AppTableChangeset {
            id: params.app_id,
            name: params.name,
            desc: params.desc,
            is_trash: params.is_trash,
        }
    }

    pub(crate) fn from_table(table: AppTable) -> Self {
        AppTableChangeset {
            id: table.id,
            name: Some(table.name),
            desc: Some(table.desc),
            is_trash: Some(table.is_trash),
        }
    }
}
impl std::convert::From<AppTable> for App {
    fn from(table: AppTable) -> Self {
        App {
            id: table.id,
            workspace_id: table.workspace_id,
            name: table.name,
            desc: table.desc,
            belongings: RepeatedView::default(),
            version: table.version,
            modified_time: table.modified_time,
            create_time: table.create_time,
        }
    }
}

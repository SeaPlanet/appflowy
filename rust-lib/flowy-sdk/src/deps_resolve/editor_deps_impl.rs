use flowy_document::{errors::DocError, module::DocumentUser};

use flowy_user::services::user::UserSession;
use std::{path::Path, sync::Arc};

pub struct EditorUserImpl {
    pub(crate) user_session: Arc<UserSession>,
}

impl DocumentUser for EditorUserImpl {
    fn user_doc_dir(&self) -> Result<String, DocError> {
        let dir = self.user_session.user_dir().map_err(|e| DocError::unauthorized().context(e))?;

        let doc_dir = format!("{}/doc", dir);
        if !Path::new(&doc_dir).exists() {
            // TODO: Make sure to unwrap? 😁
            std::fs::create_dir_all(&doc_dir).unwrap();
        }
        Ok(doc_dir)
    }

    fn user_id(&self) -> Result<String, DocError> { self.user_session.user_id().map_err(|e| DocError::internal().context(e)) }

    fn token(&self) -> Result<String, DocError> { self.user_session.token().map_err(|e| DocError::internal().context(e)) }
}

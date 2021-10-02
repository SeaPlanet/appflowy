use crate::{
    entities::doc::RevId,
    errors::{internal_error, DocResult},
    services::doc::{
        edit::{message::EditMsg, DocId},
        Document,
    },
    sql_tables::{DocTableChangeset, DocTableSql},
};
use async_stream::stream;
use flowy_database::ConnectionPool;
use flowy_ot::core::Delta;
use futures::stream::StreamExt;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

pub struct DocumentEditActor {
    doc_id: DocId,
    document: Arc<RwLock<Document>>,
    pool: Arc<ConnectionPool>,
    receiver: Option<mpsc::UnboundedReceiver<EditMsg>>,
}

impl DocumentEditActor {
    pub fn new(
        doc_id: &str,
        delta: Delta,
        pool: Arc<ConnectionPool>,
        receiver: mpsc::UnboundedReceiver<EditMsg>,
    ) -> Self {
        let doc_id = doc_id.to_string();
        let document = Arc::new(RwLock::new(Document::from_delta(delta)));
        Self {
            doc_id,
            document,
            pool,
            receiver: Some(receiver),
        }
    }

    pub async fn run(mut self) {
        let mut receiver = self.receiver.take().expect("Should only call once");
        let stream = stream! {
            loop {
                match receiver.recv().await {
                    Some(msg) => yield msg,
                    None => break,
                }
            }
        };
        stream
            .for_each(|msg| async {
                match self.handle_message(msg).await {
                    Ok(_) => {},
                    Err(e) => log::error!("{:?}", e),
                }
            })
            .await;
    }

    async fn handle_message(&self, msg: EditMsg) -> DocResult<()> {
        match msg {
            EditMsg::Delta { delta, ret } => {
                let result = self.document.write().await.compose_delta(&delta);
                let _ = ret.send(result);
            },
            EditMsg::Insert { index, data, ret } => {
                let delta = self.document.write().await.insert(index, data);
                let _ = ret.send(delta);
            },
            EditMsg::Delete { interval, ret } => {
                let result = self.document.write().await.delete(interval);
                let _ = ret.send(result);
            },
            EditMsg::Format {
                interval,
                attribute,
                ret,
            } => {
                let result = self.document.write().await.format(interval, attribute);
                let _ = ret.send(result);
            },
            EditMsg::Replace { interval, data, ret } => {
                let result = self.document.write().await.replace(interval, data);
                let _ = ret.send(result);
            },
            EditMsg::CanUndo { ret } => {
                let _ = ret.send(self.document.read().await.can_undo());
            },
            EditMsg::CanRedo { ret } => {
                let _ = ret.send(self.document.read().await.can_redo());
            },
            EditMsg::Undo { ret } => {
                let result = self.document.write().await.undo();
                let _ = ret.send(result);
            },
            EditMsg::Redo { ret } => {
                let result = self.document.write().await.redo();
                let _ = ret.send(result);
            },
            EditMsg::Doc { ret } => {
                let data = self.document.read().await.to_json();
                let _ = ret.send(Ok(data));
            },
            EditMsg::SaveRevision { rev_id, ret } => {
                let result = self.save_to_disk(rev_id).await;
                let _ = ret.send(result);
            },
        }
        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(self, rev_id), err)]
    async fn save_to_disk(&self, rev_id: RevId) -> DocResult<()> {
        let data = self.document.read().await.to_json();
        let changeset = DocTableChangeset {
            id: self.doc_id.clone(),
            data,
            rev_id: rev_id.into(),
        };
        let sql = DocTableSql {};
        let conn = self.pool.get().map_err(internal_error)?;
        let _ = sql.update_doc_table(changeset, &*conn)?;
        Ok(())
    }
}

// #[tracing::instrument(level = "debug", skip(self, params), err)]
// fn update_doc_on_server(&self, params: UpdateDocParams) -> Result<(),
//     DocError> {     let token = self.user.token()?;
//     let server = self.server.clone();
//     tokio::spawn(async move {
//         match server.update_doc(&token, params).await {
//             Ok(_) => {},
//             Err(e) => {
//                 // TODO: retry?
//                 log::error!("Update doc failed: {}", e);
//             },
//         }
//     });
//     Ok(())
// }

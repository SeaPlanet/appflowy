mod deps_resolve;
// mod flowy_server;
pub mod module;

use crate::deps_resolve::WorkspaceDepsResolver;
use flowy_dispatch::prelude::*;
use flowy_document::prelude::FlowyDocument;
use flowy_net::config::ServerConfig;
use flowy_user::services::user::{UserSession, UserSessionBuilder, UserStatus};
use flowy_workspace::{errors::WorkspaceError, prelude::WorkspaceController};
use module::mk_modules;
pub use module::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::broadcast;

static INIT_LOG: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone)]
pub struct FlowySDKConfig {
    root: String,
    log_filter: String,
    server_config: ServerConfig,
}

impl FlowySDKConfig {
    pub fn new(root: &str, server_config: ServerConfig) -> Self {
        FlowySDKConfig {
            root: root.to_owned(),
            log_filter: crate_log_filter(None),
            server_config,
        }
    }

    pub fn log_filter(mut self, filter: &str) -> Self {
        self.log_filter = crate_log_filter(Some(filter.to_owned()));
        self
    }
}

fn crate_log_filter(level: Option<String>) -> String {
    let level = level.unwrap_or(std::env::var("RUST_LOG").unwrap_or("info".to_owned()));
    let mut filters = vec![];
    filters.push(format!("flowy_sdk={}", level));
    filters.push(format!("flowy_workspace={}", level));
    filters.push(format!("flowy_user={}", level));
    filters.push(format!("flowy_document={}", level));
    filters.push(format!("flowy_dart_notify={}", level));
    filters.push(format!("flowy_ot={}", level));
    filters.push(format!("flowy_ws={}", level));
    filters.push(format!("flowy_ws={}", level));
    filters.push(format!("flowy_infra={}", level));
    filters.join(",")
}

#[derive(Clone)]
pub struct FlowySDK {
    config: FlowySDKConfig,
    pub user_session: Arc<UserSession>,
    pub flowy_document: Arc<FlowyDocument>,
    pub workspace: Arc<WorkspaceController>,
    pub dispatch: Arc<EventDispatch>,
}

impl FlowySDK {
    pub fn new(config: FlowySDKConfig) -> Self {
        init_log(&config);
        init_kv(&config.root);
        tracing::debug!("🔥 {:?}", config);

        let user_session = Arc::new(
            UserSessionBuilder::new()
                .root_dir(&config.root, &config.server_config)
                .build(),
        );
        let flowy_document = mk_document_module(user_session.clone(), &config.server_config);
        let workspace = mk_workspace(user_session.clone(), flowy_document.clone(), &config.server_config);
        let modules = mk_modules(workspace.clone(), user_session.clone());
        let dispatch = Arc::new(EventDispatch::construct(|| modules));
        _init(&dispatch, user_session.clone(), workspace.clone());

        Self {
            config,
            user_session,
            flowy_document,
            workspace,
            dispatch,
        }
    }

    pub fn dispatch(&self) -> Arc<EventDispatch> { self.dispatch.clone() }
}

fn _init(dispatch: &EventDispatch, user_session: Arc<UserSession>, workspace_controller: Arc<WorkspaceController>) {
    let subscribe = user_session.status_subscribe();
    dispatch.spawn(async move {
        user_session.init();
        _listen_user_status(subscribe, workspace_controller).await;
    });
}

async fn _listen_user_status(
    mut subscribe: broadcast::Receiver<UserStatus>,
    workspace_controller: Arc<WorkspaceController>,
) {
    loop {
        match subscribe.recv().await {
            Ok(status) => {
                let result = || async {
                    match status {
                        UserStatus::Login { .. } => {
                            let _ = workspace_controller.user_did_login()?;
                        },
                        UserStatus::Logout { .. } => {
                            workspace_controller.user_did_logout();
                        },
                        UserStatus::Expired { .. } => {
                            workspace_controller.user_session_expired();
                        },
                        UserStatus::SignUp { .. } => {
                            let _ = workspace_controller.user_did_sign_up().await?;
                        },
                    }
                    Ok::<(), WorkspaceError>(())
                };

                match result().await {
                    Ok(_) => {},
                    Err(e) => log::error!("{}", e),
                }
            },
            Err(_) => {},
        }
    }
}

fn init_kv(root: &str) {
    match flowy_infra::kv::KV::init(root) {
        Ok(_) => {},
        Err(e) => tracing::error!("Init kv store failedL: {}", e),
    }
}

fn init_log(config: &FlowySDKConfig) {
    if !INIT_LOG.load(Ordering::SeqCst) {
        INIT_LOG.store(true, Ordering::SeqCst);

        let _ = flowy_log::Builder::new("flowy-client", &config.root)
            .env_filter(&config.log_filter)
            .build();
    }
}

fn mk_workspace(
    user_session: Arc<UserSession>,
    flowy_document: Arc<FlowyDocument>,
    server_config: &ServerConfig,
) -> Arc<WorkspaceController> {
    let workspace_deps = WorkspaceDepsResolver::new(user_session.clone());
    let (user, database) = workspace_deps.split_into();
    let workspace_controller = flowy_workspace::module::mk_workspace(user, database, flowy_document, server_config);
    workspace_controller
}

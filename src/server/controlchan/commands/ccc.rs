//! The RFC 2228 Clear Command Channel (`CCC`) command

use crate::auth::UserDetail;
use crate::server::chancomms::InternalMsg;
use crate::server::controlchan::error::ControlChanError;
use crate::server::controlchan::handler::CommandContext;
use crate::server::controlchan::handler::CommandHandler;
use crate::server::controlchan::{Reply, ReplyCode};
use crate::storage;
use async_trait::async_trait;
use futures::channel::mpsc::Sender;
use futures::prelude::*;
use log::warn;
pub struct Ccc;

#[async_trait]
impl<S, U> CommandHandler<S, U> for Ccc
where
    U: UserDetail + 'static,
    S: 'static + storage::StorageBackend<U> + Sync + Send,
    S::File: tokio::io::AsyncRead + Send,
    S::Metadata: storage::Metadata,
{
    async fn handle(&self, args: CommandContext<S, U>) -> Result<Reply, ControlChanError> {
        let mut tx: Sender<InternalMsg> = args.tx.clone();
        let session = args.session.lock().await;
        if session.cmd_tls {
            tokio::spawn(async move {
                if let Err(err) = tx.send(InternalMsg::PlaintextControlChannel).await {
                    warn!("{}", err);
                }
            });
            Ok(Reply::new(ReplyCode::CommandOkay, "control channel in plaintext now"))
        } else {
            Ok(Reply::new(ReplyCode::Resp533, "control channel already in plaintext mode"))
        }
    }
}

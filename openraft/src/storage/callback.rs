//! Callbacks used by Storage API

use std::io;

use tokio::sync::oneshot;

use crate::async_runtime::AsyncOneshotSendExt;
use crate::raft_state::io_state::log_io_id::LogIOId;
use crate::type_config::alias::OneshotSenderOf;
use crate::LogId;
use crate::RaftTypeConfig;
use crate::StorageIOError;

/// A oneshot callback for completion of log io operation.
pub struct LogFlushed<C>
where C: RaftTypeConfig
{
    log_io_id: LogIOId<C::NodeId>,
    tx: OneshotSenderOf<C, Result<LogIOId<C::NodeId>, io::Error>>,
}

impl<C> LogFlushed<C>
where C: RaftTypeConfig
{
    pub(crate) fn new(
        log_io_id: LogIOId<C::NodeId>,
        tx: OneshotSenderOf<C, Result<LogIOId<C::NodeId>, io::Error>>,
    ) -> Self {
        Self { log_io_id, tx }
    }

    /// Report log io completion event.
    ///
    /// It will be called when the log is successfully appended to the storage or an error occurs.
    pub fn log_io_completed(self, result: Result<(), io::Error>) {
        let res = if let Err(e) = result {
            tracing::error!("LogFlush error: {}, while flushing upto {}", e, self.log_io_id);
            self.tx.send(Err(e))
        } else {
            self.tx.send(Ok(self.log_io_id))
        };

        if let Err(e) = res {
            tracing::error!("failed to send log io completion event: {:?}", e);
        }
    }
}

/// A oneshot callback for completion of applying logs to state machine.
pub struct LogApplied<C>
where C: RaftTypeConfig
{
    last_log_id: LogId<C::NodeId>,
    tx: oneshot::Sender<Result<(LogId<C::NodeId>, Vec<C::R>), StorageIOError<C::NodeId>>>,
}

impl<C> LogApplied<C>
where C: RaftTypeConfig
{
    #[allow(dead_code)]
    pub(crate) fn new(
        last_log_id: LogId<C::NodeId>,
        tx: oneshot::Sender<Result<(LogId<C::NodeId>, Vec<C::R>), StorageIOError<C::NodeId>>>,
    ) -> Self {
        Self { last_log_id, tx }
    }

    /// Report apply io completion event.
    ///
    /// It will be called when the log is successfully applied to the state machine or an error
    /// occurs.
    pub fn completed(self, result: Result<Vec<C::R>, StorageIOError<C::NodeId>>) {
        let res = match result {
            Ok(x) => {
                tracing::debug!("LogApplied upto {}", self.last_log_id);
                let resp = (self.last_log_id, x);
                self.tx.send(Ok(resp))
            }
            Err(e) => {
                tracing::error!("LogApplied error: {}, while applying upto {}", e, self.last_log_id);
                self.tx.send(Err(e))
            }
        };

        if let Err(_e) = res {
            tracing::error!("failed to send apply complete event, last_log_id: {}", self.last_log_id);
        }
    }
}

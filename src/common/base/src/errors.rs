use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RobustMQError {
    #[error("io error")]
    IOJsonError(#[from] io::Error),

    #[error("{0}")]
    CommonError(String),

    #[error("No available nodes in the cluster")]
    ClusterNoAvailableNode,

    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Description The interface {0} submitted logs to the commit log")]
    RaftLogCommitTimeout(String),
}

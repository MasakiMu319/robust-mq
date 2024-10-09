use std::sync::Arc;

use common_base::errors::RobustMQError;

use super::rocksdb::RocksDBEngine;

pub struct KvStorage {
    rocksdb_engine_handler: Arc<RocksDBEngine>,
}

impl KvStorage {
    pub fn new(rocksdb_engine_handler: Arc<RocksDBEngine>) -> Self {
        KvStorage {
            rocksdb_engine_handler,
        }
    }

    pub fn set(&self, key: String, value: String) -> Result<(), RobustMQError> {
        return;
    }
}

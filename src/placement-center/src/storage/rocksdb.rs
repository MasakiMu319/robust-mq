use core::error;
use std::{collections::HashMap, hash::Hash, path::Path};

use common_base::{config::placement_center::PlacementCenterConfig, errors::RobustMQError};
use log::error;
use rocksdb::{ColumnFamily, DBCompactionStyle, Options, SliceTransform, DB};
use serde::{
    de::{value, DeserializeOwned},
    Deserialize, Serialize,
};

pub const DB_COLUMN_FAMILY_CLUSTER: &str = "cluster";

fn column_family_list() -> Vec<String> {
    let mut list = Vec::new();
    list.push(DB_COLUMN_FAMILY_CLUSTER.to_string());
    return list;
}

pub struct RocksDBEngine {
    pub db: DB,
}

impl RocksDBEngine {
    fn open_db_opts() -> Options {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_max_open_files(1000);
        opts.set_use_fsync(false);
        opts.set_bytes_per_sync(8388608);
        opts.optimize_for_point_lookup(1024);
        opts.set_table_cache_num_shard_bits(6);
        opts.set_max_write_buffer_number(32);
        opts.set_write_buffer_size(536870912);
        opts.set_target_file_size_base(1073741824);
        opts.set_min_write_buffer_number_to_merge(4);
        opts.set_level_zero_stop_writes_trigger(2000);
        opts.set_level_zero_slowdown_writes_trigger(0);
        opts.set_compaction_style(DBCompactionStyle::Universal);
        opts.set_disable_auto_compactions(true);

        let transform = SliceTransform::create_fixed_prefix(10);
        opts.set_prefix_extractor(transform);
        opts.set_memtable_prefix_bloom_ratio(0.2);

        opts
    }

    pub fn cf_cluster(&self) -> &ColumnFamily {
        self.db.cf_handle(&DB_COLUMN_FAMILY_CLUSTER).unwrap()
    }

    pub fn get_column_family(&self) -> &ColumnFamily {
        self.cf_cluster()
    }

    pub fn exist(&self, cf: &ColumnFamily, key: &str) -> bool {
        self.db.key_may_exist_cf(cf, key)
    }

    pub fn new(config: &PlacementCenterConfig) -> Self {
        let opts: Options = Self::open_db_opts();
        let db_path = format!("{}/{}", config.data_path, "_storage_rocksdb");

        if !Path::new(&db_path).exists() {
            DB::open(&opts, db_path.clone()).unwrap();
        }

        let cf_list = rocksdb::DB::list_cf(&opts, &db_path).unwrap();
        let mut instance = DB::open_cf(&opts, db_path.clone(), &cf_list).unwrap();

        for family in column_family_list().iter() {
            if cf_list.iter().find(|cf| cf == &family).is_none() {
                match instance.create_cf(&family, &opts) {
                    Ok(()) => {}
                    Err(e) => {
                        panic!("{}", e);
                    }
                }
            }
        }

        return RocksDBEngine { db: instance };
    }

    pub fn write<T: Serialize + std::fmt::Debug>(
        &self,
        cf: &ColumnFamily,
        key: &str,
        value: &T,
    ) -> Result<(), String> {
        match serde_json::to_string(&value) {
            Ok(serialized) => self
                .db
                .put_cf(cf, key, serialized.into_bytes())
                .map_err(|err| format!("Failed to put to ColumnFamily:{:?}", err)),
            Err(err) => Err(format!(
                "Failed to serialize to String. T: {:?}, err: {:?}",
                value, err
            )),
        }
    }

    pub fn write_str(&self, cf: &ColumnFamily, key: &str, value: String) -> Result<(), String> {
        self.db
            .put_cf(cf, key, value.into_bytes())
            .map_err(|err| format!("Failed to put to ColumnFamily:{:?}", err))
    }

    pub fn read<T: DeserializeOwned>(
        &self,
        cf: &ColumnFamily,
        key: &str,
    ) -> Result<Option<T>, String> {
        match self.db.get_cf(cf, key) {
            Ok(opt) => match opt {
                Some(found) => match String::from_utf8(found) {
                    Ok(s) => match serde_json::from_str::<T>(&s) {
                        Ok(t) => Ok(Some(t)),
                        Err(err) => Err(format!("Failed to deserialize: {:?}", err)),
                    },
                    Err(err) => Err(format!("Failed to deserialize: {:?}", err)),
                },
                None => Ok(None),
            },
            Err(err) => Err(format!("Failed to get from ColumnFamily: {:?}", err)),
        }
    }

    pub fn read_prefix(
        &self,
        cf: &ColumnFamily,
        search_key: &str,
    ) -> Vec<HashMap<String, Vec<u8>>> {
        let mut iter = self.db.raw_iterator_cf(cf);
        iter.seek(search_key);

        let mut result = Vec::new();
        while iter.valid() {
            let key = iter.key();
            let value = iter.value();

            let mut raw = HashMap::new();
            if key == None || value == None {
                continue;
            }

            let result_key = match String::from_utf8(key.unwrap().to_vec()) {
                Ok(s) => s,
                Err(_) => continue,
            };

            if !result_key.starts_with(search_key) {
                break;
            }

            raw.insert(result_key, value.unwrap().to_vec());
            result.push(raw);
            iter.next();
        }

        return result;
    }

    pub fn read_all_by_cf(&self, cf: &ColumnFamily) -> Vec<HashMap<String, String>> {
        let mut iter = self.db.raw_iterator_cf(cf);
        iter.seek_to_first();

        let mut result: Vec<HashMap<String, String>> = Vec::new();
        while iter.valid() {
            if let Some(key) = iter.key() {
                if let Some(val) = iter.value() {
                    match String::from_utf8(key.to_vec()) {
                        Ok(key) => match String::from_utf8(val.to_vec()) {
                            Ok(da) => {
                                let mut raw: HashMap<String, String> = HashMap::new();
                                raw.insert(key, da);
                                result.push(raw);
                            }
                            Err(e) => {
                                error!("{}", e);
                            }
                        },
                        Err(e) => {
                            error!("{}", e);
                        }
                    }
                }
            }
            iter.next();
        }

        return result;
    }

    pub fn read_all(&self) -> HashMap<String, Vec<HashMap<String, String>>> {
        let mut result: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
        for family in column_family_list().iter() {
            let cf = self.get_column_family();
            result.insert(family.to_string(), self.read_all_by_cf(cf));
        }
        return result;
    }

    pub fn delete(&self, cf: &ColumnFamily, key: &str) -> Result<(), RobustMQError> {
        return Ok(self.db.delete_cf(cf, key)?);
    }
}

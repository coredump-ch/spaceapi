extern crate redis;

use std::sync::{Mutex,Arc};

use self::redis::{Client, Commands};

use super::{DataStore, SafeDataStore, DataStoreError};


/// A data store for Redis.
pub struct RedisStore {
    client : Client,
}

/// Implement the DataStore methods for Redis
impl DataStore for RedisStore {

    fn store(&self, key: &str, value: &str) -> Result<(), DataStoreError> {
        let con = try!(self.client.get_connection());

        try!(con.set(key, value));
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<String, DataStoreError> {
        let con = try!(self.client.get_connection());

        Ok(try!(con.get(key)))
    }

    fn delete(&self, key: &str) -> Result<(), DataStoreError> {
        let con = try!(self.client.get_connection());

        Ok(try!(con.del(key)))
    }

    fn make_safe(self) -> SafeDataStore {
        Arc::new(Mutex::new(Box::new(self)))
    }

}

impl RedisStore {
    pub fn new() -> Result<RedisStore, DataStoreError> {
        let redis_client = try!(Client::open("redis://127.0.0.1/"));
        Ok(RedisStore { client: redis_client })
    }
}

#[cfg(test)]
mod test {
    use datastore::DataStore;
    use super::RedisStore;

    #[test]
    fn roundtrip() {
        let rs = RedisStore::new().unwrap();
        rs.store("key", "value");
        let result = rs.retrieve("key").unwrap();
        assert_eq!(result, "value");
        rs.delete("key");
    }

    #[test]
    #[should_panic(expected = "response was nil")]
    fn nonexistant() {
        let rs = RedisStore::new().unwrap();
        rs.retrieve("nonexistant").unwrap();
    }

}

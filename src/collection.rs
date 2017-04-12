use serde_json;
use serde_json::Value;

use KintoConfig;
use error::KintoError;
use request::KintoRequest;
use response::ResponseWrapper;
use resource::Resource;
use bucket::Bucket;
use record::Record;
use utils::unwrap_collection_records;


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectionPermissions {
    #[serde(skip_serializing_if="Option::is_none")]
    pub read: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub write: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none", rename = "record:create")]
    pub create_record: Option<Vec<String>>,
}


#[derive(Debug, Clone)]
pub struct Collection {
    pub data: Option<Value>,
    pub permissions: CollectionPermissions,
    pub bucket: Bucket,
    pub id: Option<String>,
}


impl Collection {
    /// Create a new collection resource.
    pub fn new(bucket: Bucket) -> Self {
        Collection {
            bucket: bucket,
            id: None,
            data: None,
            permissions: CollectionPermissions::default(),
        }
    }

    /// Create a new collection resource.
    pub fn new_by_id<'a>(bucket: Bucket, id: &'a str) -> Self {
        Collection {
            bucket: bucket,
            id: Some(id.to_owned()),
            data: None,
            permissions: CollectionPermissions::default(),
        }
    }

    pub fn record<'a>(&self, id: &'a str) -> Record {
        return Record::new_by_id(self.clone(), id);
    }

    /// Create a new empty record with a generated id.
    pub fn new_record(&self) -> Record {
        return Record::new(self.clone());
    }

    /// List the names of all available records.
    pub fn list_records(&self) -> Result<Vec<Record>, KintoError> {
        let response = try!(try!(self.new_record().list_request()).follow_subrequests());
        return Ok(unwrap_collection_records(response, self.new_record()));
    }

    /// Delete all available records.
    pub fn delete_records(&self) -> Result<(), KintoError> {
        let resource = Record::new(self.clone());
        try!(try!(resource.delete_all_request()).follow_subrequests());
        Ok(())
    }
}


impl Resource for Collection {
    fn resource_path(&self) -> Result<String, KintoError> {
        Ok(format!("{}/collections", try!(self.bucket.record_path())))
    }

    fn unwrap_response(&mut self, wrapper: ResponseWrapper) {
        self.data = Some(wrapper.body["data"].to_owned());
        self.permissions = serde_json::from_value(wrapper.body["permissions"].to_owned())
            .unwrap();
        self.id = Some(wrapper.body["data"]["id"].as_str().unwrap().to_owned());
    }

    fn get_config(&self) -> KintoConfig {
        self.bucket.get_config()
    }

    fn get_id(&self) -> Option<&str> {
        match self.id.as_ref() {
            Some(id) => return Some(id),
            None => (),
        };

        match self.data.as_ref() {
            Some(data) => return data["id"].as_str(),
            None => (),
        };

        return None;
    }

    fn get_timestamp(&self) -> Option<u64> {
        match self.get_data() {
            Some(data) => {
                match data["last_modified"].as_u64() {
                    Some(ts) => ts.into(),
                    None => None,
                }
            }
            None => None,
        }
    }

    fn get_data(&self) -> Option<Value> {
        return self.data.clone();
    }

    fn set_data(&mut self, data: Value) -> Self {
        self.data = data.into();
        return self.clone();
    }

    fn get_permissions(&self) -> Option<Value> {
        serde_json::to_value(&(self.permissions)).unwrap_or_default().into()
    }
}


#[cfg(test)]
mod test_collection {
    use request::{KintoRequest, PluralEndpoint};
    use resource::Resource;
    use record::Record;
    use utils::unwrap_collection_records;
    use utils::tests::{setup_collection, setup_bucket};

    #[test]
    fn test_create_collection() {
        let mut collection = setup_collection();
        collection.data = json!({"good": true}).into();

        collection.create().unwrap();
        let data = collection.data.unwrap().to_owned();

        assert_eq!(data["id"], "meat");
        assert_eq!(data["good"].as_bool().unwrap(), true);
    }

    #[test]
    fn test_create_collection_fails_on_existing() {
        let mut collection = setup_collection();

        // Create
        collection.create().unwrap();

        // Tries to create again
        collection.create().unwrap_err();
    }

    #[test]
    fn test_load_collection() {
        let mut collection = setup_collection();
        collection.set().unwrap();
        let create_data = collection.data.clone().unwrap();

        // Cleanup stored data to make sure load work
        collection.data = json!({}).into();

        collection.load().unwrap();
        let load_data = collection.data.unwrap();


        assert_eq!(create_data, load_data);
    }

    #[test]
    fn test_load_collection_fails_on_not_existing() {
        let mut collection = setup_collection();
        collection.load().unwrap_err();
    }

    #[test]
    fn test_update_collection() {
        let mut collection = setup_collection();

        collection.create().unwrap();
        let create_data = collection.data.clone().unwrap();

        collection.update().unwrap();
        let update_data = collection.data.unwrap();

        assert_eq!(create_data["id"], update_data["id"]);
        assert!(create_data["last_modified"] != update_data["last_modified"]);
    }

    #[test]
    fn test_update_collection_fails_on_not_existing() {
        let client = setup_bucket();
        let mut collection = client.collection("food");
        collection.update().unwrap_err();
    }

    #[test]
    fn test_get_record() {
        let collection = setup_collection();
        let record = collection.record("entrecote");
        assert_eq!(record.get_id().unwrap(), "entrecote");
        assert_eq!(record.data, None);
    }

    #[test]
    fn test_new_record() {
        let mut collection = setup_collection();
        collection.create().unwrap();
        let record = collection.new_record();
        assert_eq!(record.data, None);
        assert_eq!(record.get_id(), None);

    }

    #[test]
    fn test_list_records() {
        let mut collection = setup_collection();
        collection.create().unwrap();
        for _ in 0..10 {
            collection.new_record().create().unwrap();
        }
        let records = collection.list_records();
        assert_eq!(records.unwrap().len(), 10);
    }

    #[test]
    fn test_paginated_records_list() {
        let mut collection = setup_collection();
        collection.create().unwrap();
        for _ in 0..10 {
            collection.new_record().create().unwrap();
        }

        let resource = Record::new(collection.clone());
        let response = resource.list_request()
            .unwrap()
            .limit(3)
            .follow_subrequests()
            .unwrap();
        let records: Vec<Record> = unwrap_collection_records(response,
                                                             collection.new_record());
        assert_eq!(records.len(), 10);
    }

    #[test]
    fn test_delete_records() {
        let mut collection = setup_collection();
        collection.create().unwrap();
        for _ in 0..10 {
            collection.new_record().create().unwrap();
        }
        collection.delete_records().unwrap();
        let records = collection.list_records();
        assert_eq!(records.unwrap().len(), 0);
    }

    #[test]
    fn test_paginated_records_delete() {
        let mut collection = setup_collection();
        collection.create().unwrap();
        for _ in 0..10 {
            collection.new_record().create().unwrap();
        }

        let resource = Record::new(collection.clone());
        resource.delete_all_request()
            .unwrap()
            .limit(5)
            .follow_subrequests()
            .unwrap();
        let records = collection.list_records().unwrap();
        assert_eq!(records.len(), 0);
    }
}

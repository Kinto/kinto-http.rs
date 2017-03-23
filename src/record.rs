use serde_json;
use serde_json::Value;

use KintoClient;
use paths::Paths;
use request::{GetRecord, CreateRecord, UpdateRecord, DeleteRecord};
use response::ResponseWrapper;
use resource::Resource;
use bucket::Bucket;
use collection::Collection;
use utils::extract_ids_from_path;


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordPermissions {
    #[serde(default="Vec::new")]
    pub read: Vec<String>,
    #[serde(default="Vec::new")]
    pub write: Vec<String>,
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Record {
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<RecordPermissions>,

    #[serde(skip_serializing, skip_deserializing)]
    pub client: KintoClient,

    #[serde(skip_serializing, skip_deserializing)]
    pub bucket: Bucket,

    #[serde(skip_serializing, skip_deserializing)]
    pub collection: Collection,
}


impl Record {

    /// Create a new record object without an id.
    pub fn new<'c>(client: KintoClient, collection: Collection) -> Self {
        Record {
            client: client,
            bucket: collection.bucket.clone(),
            collection: collection.clone(),
            data: None,
            permissions: None
        }
    }

    /// Create a new record object with an id.
    pub fn new_by_id<'a>(client: KintoClient, collection: Collection,
                         id: &'a str) -> Self {
        Record {
            client: client,
            bucket: collection.bucket.clone(),
            collection: collection,
            data: json!({"id": id}).into(),
            permissions: None
        }
    }
}


impl Resource for Record {

    fn unwrap_response(&mut self, wrapper: ResponseWrapper){
        *self = wrapper.into()
    }

    fn get_data(&self) -> Option<&Value> {
       self.data.as_ref()
    }

    fn load_request(&self) -> GetRecord {
        GetRecord::new(self.client.clone(),
                       Paths::Record(self.bucket.get_id().unwrap(),
                                     self.collection.get_id().unwrap(),
                                     self.get_id().unwrap()).into())
    }

    fn create_request(&self) -> CreateRecord {
        CreateRecord::new(self.client.clone(),
                          Paths::Records(self.bucket.get_id().unwrap(),
                                        self.collection.get_id().unwrap()).into())
    }

    fn update_request(&self) -> UpdateRecord {
        UpdateRecord::new(self.client.clone(),
                          Paths::Record(self.bucket.get_id().unwrap(),
                                        self.collection.get_id().unwrap(),
                                        self.get_id().unwrap()).into())
    }

    fn delete_request(&self) -> DeleteRecord {
        DeleteRecord::new(self.client.clone(),
                          Paths::Record(self.bucket.get_id().unwrap(),
                                        self.collection.get_id().unwrap(),
                                        self.get_id().unwrap()).into())
    }
}


impl From<ResponseWrapper> for Record {
    fn from(wrapper: ResponseWrapper) -> Self {
        let path_ids = extract_ids_from_path(wrapper.path);

        let bucket_id = path_ids["buckets"].clone().unwrap();
        let collection_id = path_ids["collections"].clone().unwrap();

        let bucket = Bucket::new_by_id(wrapper.client.clone(),
                                       bucket_id.as_str());
        let collection = Collection::new_by_id(wrapper.client.clone(),
                                               bucket.clone(),
                                               collection_id.as_str());
        let record: Record = serde_json::from_value(wrapper.body).unwrap();

        Record {
            client: wrapper.client,
            bucket: bucket,
            collection: collection,
            ..record
        }
    }
}


#[cfg(test)]
mod test_record {
    use resource::Resource;
    use utils::tests::{setup_record, setup_collection};

    #[test]
    fn test_create_record() {
        let mut record = setup_record();
        record.data = json!({"good": true}).into();

        record.create().unwrap();
        let data = record.data.unwrap().to_owned();

        assert_eq!(data["id"], "entrecote");
        assert_eq!(data["good"].as_bool().unwrap(), true);
    }

    #[test]
    fn test_create_record_fails_on_existing() {
        let mut record = setup_record();

        // Create
        record.create().unwrap();

        // Tries to create again
        match record.create() {
            Ok(_) => panic!(""),
            Err(_) => ()
        }
    }

    #[test]
    fn test_load_record() {
        let mut record = setup_record();
        record.set().unwrap();
        let create_data = record.data.clone().unwrap();

        // Cleanup stored data to make sure load work
        record.data = json!({}).into();

        record.load().unwrap();
        let load_data = record.data.unwrap();


        assert_eq!(create_data, load_data);
    }

    #[test]
    fn test_load_record_fails_on_not_existing() {
        let mut record = setup_record();
        match record.load() {
            Ok(_) => panic!(""),
            Err(_) => ()
        }
    }

    #[test]
    fn test_update_record() {
        let mut record = setup_record();

        record.create().unwrap();
        let create_data = record.data.clone().unwrap();

        record.update().unwrap();
        let update_data = record.data.unwrap();

        assert_eq!(create_data["id"], update_data["id"]);
        assert!(create_data["last_modified"] != update_data["last_modified"]);
    }

    #[test]
    fn test_update_record_fails_on_not_existing() {
        let client = setup_collection();
        let mut record = client.record("food");
        match record.update() {
            Ok(_) => panic!(""),
            Err(_) => ()
        }
    }
}

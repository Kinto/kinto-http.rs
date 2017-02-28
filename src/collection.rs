use serde_json;
use serde_json::Value;

use KintoClient;
use error::KintoError;
use paths::Paths;
use request::{GetCollection, DeleteCollection, GetRecord, CreateRecord,
              UpdateRecord, DeleteRecord, KintoRequest};
use response::ResponseWrapper;
use resource::Resource;
use bucket::Bucket;
use record::Record;
use utils::{unwrap_collection_ids, extract_ids_from_path};


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectionPermissions {
    #[serde(default="Vec::new")]
    pub read: Vec<String>,
    #[serde(default="Vec::new")]
    pub write: Vec<String>,
    #[serde(default="Vec::new", rename = "record:create")]
    pub create_record: Vec<String>,
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Collection {
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<CollectionPermissions>,

    #[serde(skip_serializing, skip_deserializing)]
    pub client: KintoClient,

    #[serde(skip_serializing, skip_deserializing)]
    pub bucket: Bucket,

    #[serde(skip_serializing, skip_deserializing)]
    pub id: String,

    #[serde(skip_serializing, skip_deserializing)]
    pub timestamp: Option<u64>,
}


impl Collection {

    /// Create a new collection resource.
    pub fn new<'a>(client: KintoClient, bucket: Bucket, id: &'a str) -> Self {
        Collection {client: client, bucket: bucket, id: id.to_owned(),
                    timestamp: None, data: None, permissions: None}
    }

    pub fn record(self, id: &'static str) -> Record {
        return Record::new(self.client.clone(), self, id);
    }

    /// Create a new empty record with a generated id.
    pub fn new_record(&mut self) -> Result<Record, KintoError> {
        match self.create_record_request().send() {
            Ok(wrapper) => Ok(wrapper.into()),
            Err(value) => return Err(value)
        }
    }

    /// List the names of all available records.
    pub fn list_records(&mut self) -> Result<Vec<String>, KintoError> {
        let response = try!(self.list_records_request().send());
        // XXX: we should follow possible subrequests
        Ok(unwrap_collection_ids(response))
    }

    /// Delete all available records.
    pub fn delete_records(&mut self) -> Result<(), KintoError> {
        try!(self.delete_records_request().send());
        Ok(())
    }

    pub fn list_records_request(&mut self) -> GetCollection {
        GetCollection::new(self.client.clone(),
                           Paths::Records(self.bucket.id.as_str(),
                                          self.id.as_str()).into())
    }

    pub fn delete_records_request(&mut self) -> DeleteCollection {
        DeleteCollection::new(self.client.clone(),
                           Paths::Records(self.bucket.id.as_str(),
                                          self.id.as_str()).into())
    }

    pub fn create_record_request(&mut self) -> CreateRecord {
        CreateRecord::new(self.client.clone(),
                           Paths::Records(self.bucket.id.as_str(),
                                          self.id.as_str()).into())
    }
}


impl Resource for Collection {

    fn unwrap_response(&mut self, wrapper: ResponseWrapper){
        *self = wrapper.into()
    }

    fn get_timestamp(&mut self) -> Option<u64> {
        self.timestamp
    }

    fn load_request(&mut self) -> GetRecord {
        GetRecord::new(self.client.clone(),
                       Paths::Collection(self.bucket.id.as_str(),
                                         self.id.as_str()).into())
    }

    fn update_request(&mut self) -> UpdateRecord {
        UpdateRecord::new(self.client.clone(),
                          Paths::Collection(self.bucket.id.as_str(),
                                            self.id.as_str()).into())
    }

    fn delete_request(&mut self) -> DeleteRecord {
        DeleteRecord::new(self.client.clone(),
                          Paths::Collection(self.bucket.id.as_str(),
                                            self.id.as_str()).into())
    }
}


impl From<ResponseWrapper> for Collection {
    fn from(wrapper: ResponseWrapper) -> Self {

        let path_ids = extract_ids_from_path(wrapper.path);
        let bucket_id = path_ids["buckets"].clone().unwrap();

        let collection: Collection = serde_json::from_str(&wrapper.body).unwrap();
        let data = collection.data.clone().unwrap();

        let timestamp = data["last_modified"].as_u64().unwrap();

        Collection {
            client: wrapper.client.clone(),
            bucket: Bucket::new(wrapper.client, bucket_id.as_str()),
            id: data["id"].as_str().unwrap().to_owned(),
            timestamp: Some(timestamp.into()),
            ..collection
        }
    }
}


#[cfg(test)]
mod test_client {
    use resource::Resource;
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
        match collection.create() {
            Ok(_) => panic!(""),
            Err(_) => ()
        }
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
        match collection.load() {
            Ok(_) => panic!(""),
            Err(_) => ()
        }
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
        match collection.update() {
            Ok(_) => panic!(""),
            Err(_) => ()
        }
    }

    #[test]
    fn test_get_record() {
        let collection = setup_collection();
        let record = collection.record("entrecote");
        assert_eq!(record.id, "entrecote");
        assert!(record.data == None);
    }

    #[test]
    fn test_new_record() {
        let mut collection = setup_collection();
        collection.create().unwrap();
        let record = collection.new_record().unwrap();
        assert!(record.data != None);
        assert_eq!(collection.id.as_str(),
                   collection.data.unwrap()["id"].as_str().unwrap());
    }

    #[test]
    fn test_list_records() {
        let mut collection = setup_collection();
        collection.create().unwrap();
        assert_eq!(collection.list_records().unwrap().len(), 0);
        collection.new_record().unwrap();
        assert_eq!(collection.list_records().unwrap().len(), 1);
    }

    #[test]
    fn test_delete_records() {
        let mut collection = setup_collection();
        collection.create().unwrap();
        collection.new_record().unwrap();
        assert_eq!(collection.list_records().unwrap().len(), 1);
        collection.delete_records().unwrap();
        assert_eq!(collection.list_records().unwrap().len(), 0);
    }

    #[test]
    fn test_list_records_request() {
        let mut collection = setup_collection();
        let request = collection.list_records_request();
        assert_eq!(request.preparer.path, "/buckets/food/collections/meat/records");
    }

    #[test]
    fn test_delete_records_request() {
        let mut collection = setup_collection();
        let request = collection.delete_records_request();
        assert_eq!(request.preparer.path, "/buckets/food/collections/meat/records");
    }

    #[test]
    fn test_create_records_request() {
        let mut collection = setup_collection();
        let request = collection.create_record_request();
        assert_eq!(request.preparer.path, "/buckets/food/collections/meat/records");
    }
}

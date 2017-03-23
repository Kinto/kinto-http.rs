use serde_json;
use serde_json::Value;

use KintoClient;
use error::KintoError;
use paths::Paths;
use request::{GetCollection, DeleteCollection, GetRecord, CreateRecord,
              UpdateRecord, DeleteRecord, KintoRequest};
use response::ResponseWrapper;
use resource::Resource;
use collection::Collection;
use utils::unwrap_collection_ids;


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BucketPermissions {
    #[serde(default="Vec::new")]
    pub read: Vec<String>,
    #[serde(default="Vec::new")]
    pub write: Vec<String>,
    #[serde(default="Vec::new", rename = "collection:create")]
    pub create_collection: Vec<String>,
    #[serde(default="Vec::new", rename="group:create")]
    pub create_group: Vec<String>,
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Bucket {
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<BucketPermissions>,

    #[serde(skip_serializing, skip_deserializing)]
    pub client: KintoClient,

    #[serde(skip_serializing, skip_deserializing)]
    pub id: Option<String>,

    #[serde(skip_serializing, skip_deserializing)]
    pub timestamp: Option<u64>,
}


impl Bucket {
    /// Create a new bucket resource.
    pub fn new<'a>(client: KintoClient, id: &'a str) -> Self {
        Bucket {client: client, id: id.to_owned().into(),
                timestamp: None, data: None, permissions: None}
    }

    pub fn collection<'a>(self, id: &'a str) -> Collection {
        return Collection::new(self.client.clone(), self, id);
    }

    /// Create a new empty collection with a generated id.
    pub fn new_collection(&mut self) -> Result<Collection, KintoError> {
        match self.create_collection_request().send() {
            Ok(wrapper) => Ok(wrapper.into()),
            Err(value) => return Err(value)
        }
    }

    /// List the names of all available collections.
    pub fn list_collections(&mut self) -> Result<Vec<String>, KintoError> {
        let response = try!(self.list_collections_request().send());
        // XXX: we should follow possible subrequests
        Ok(unwrap_collection_ids(response))
    }

    /// Delete all available collections.
    pub fn delete_collections(&mut self) -> Result<(), KintoError> {
        try!(self.delete_collections_request().send());
        Ok(())
    }

    /// Create a custom list collections request.
    pub fn list_collections_request(&mut self) -> GetCollection {
        GetCollection::new(self.client.clone(),
                           Paths::Collections(self.id.as_ref().unwrap()).into())
    }

    /// Create a custom delete collections request.
    pub fn delete_collections_request(&mut self) -> DeleteCollection {
        DeleteCollection::new(self.client.clone(),
                              Paths::Collections(self.id.as_ref().unwrap()).into())
    }

    /// Create a custom create collection request.
    pub fn create_collection_request(&mut self) -> CreateRecord {
        CreateRecord::new(self.client.clone(),
                          Paths::Collections(self.id.as_ref().unwrap()).into())
    }
}


impl Resource for Bucket {
    fn unwrap_response(&mut self, wrapper: ResponseWrapper){
        *self = wrapper.into()
    }

    fn get_id(&mut self) -> Option<String> {
        None
    }

    fn get_timestamp(&mut self) -> Option<u64> {
        self.timestamp
    }

    fn load_request(&mut self) -> GetRecord {
        GetRecord::new(self.client.clone(),
                       Paths::Bucket(self.id.as_ref().unwrap()).into())
    }

    fn create_request(&mut self) -> CreateRecord {
        CreateRecord::new(self.client.clone(),
                          Paths::Buckets().into())
    }

    fn update_request(&mut self) -> UpdateRecord {
        UpdateRecord::new(self.client.clone(),
                          Paths::Bucket(self.id.as_ref().unwrap()).into())
    }

    fn delete_request(&mut self) -> DeleteRecord {
        DeleteRecord::new(self.client.clone(),
                          Paths::Bucket(self.id.as_ref().unwrap()).into())
    }
}


impl From<ResponseWrapper> for Bucket {
    fn from(wrapper: ResponseWrapper) -> Self {

        let bucket: Bucket = serde_json::from_value(wrapper.body).unwrap();
        let data = bucket.data.clone().unwrap();

        let timestamp = data["last_modified"].as_u64().unwrap();

        Bucket {
            client: wrapper.client,
            id: Some(data["id"].as_str().unwrap().to_owned()),
            timestamp: Some(timestamp.into()),
            ..bucket
        }
    }
}


#[cfg(test)]
mod test_bucket {
    use utils::tests::{setup_client, setup_bucket};
    use resource::Resource;

    #[test]
    fn test_create_bucket() {
        let mut bucket = setup_bucket();
        bucket.data = json!({"good": true}).into();

        bucket.create().unwrap();
        let data = bucket.data.unwrap().to_owned();

        assert_eq!(data["id"], "food");
        assert_eq!(data["good"].as_bool().unwrap(), true);
    }

    #[test]
    fn test_create_bucket_fails_on_existing() {
        let mut bucket = setup_bucket();

        // Create
        bucket.create().unwrap();

        // Tries to create again
        match bucket.create() {
            Ok(_) => panic!(""),
            Err(_) => ()
        }
    }

    #[test]
    fn test_load_bucket() {
        let mut bucket = setup_bucket();
        bucket.set().unwrap();
        let create_data = bucket.data.clone().unwrap();

        // Cleanup stored data to make sure load work
        bucket.data = json!({}).into();

        bucket.load().unwrap();
        let load_data = bucket.data.unwrap();


        assert_eq!(create_data, load_data);
    }

    #[test]
    fn test_load_bucket_fails_on_not_existing() {
        let mut bucket = setup_bucket();
        match bucket.load() {
            Ok(_) => panic!(""),
            Err(_) => ()
        }
    }

    #[test]
    fn test_update_bucket() {
        let mut bucket = setup_bucket();

        bucket.create().unwrap();
        let create_data = bucket.data.clone().unwrap();

        bucket.update().unwrap();
        let update_data = bucket.data.unwrap();

        assert_eq!(create_data["id"], update_data["id"]);
        assert!(create_data["last_modified"] != update_data["last_modified"]);
    }

    #[test]
    fn test_update_bucket_fails_on_not_existing() {
        let mut client = setup_client();
        let mut bucket = client.bucket("food");
        match bucket.update() {
            Ok(_) => panic!(""),
            Err(_) => ()
        }
    }

    #[test]
    fn test_get_collection() {
        let bucket = setup_bucket();
        let collection = bucket.collection("meat");
        assert_eq!(collection.id.unwrap(), "meat");
        assert!(collection.data == None);
    }

    #[test]
    fn test_new_collection() {
        let mut bucket = setup_bucket();
        bucket.create().unwrap();
        let collection = bucket.new_collection().unwrap();
        assert!(collection.data != None);
        assert_eq!(collection.id.unwrap().as_str(),
                   collection.data.unwrap()["id"].as_str().unwrap());
    }

    #[test]
    fn test_list_collections() {
        let mut bucket = setup_bucket();
        bucket.create().unwrap();
        assert_eq!(bucket.list_collections().unwrap().len(), 0);
        bucket.new_collection().unwrap();
        assert_eq!(bucket.list_collections().unwrap().len(), 1);
    }

    #[test]
    fn test_delete_collections() {
        let mut bucket = setup_bucket();
        bucket.create().unwrap();
        bucket.new_collection().unwrap();
        assert_eq!(bucket.list_collections().unwrap().len(), 1);
        bucket.delete_collections().unwrap();
        assert_eq!(bucket.list_collections().unwrap().len(), 0);
    }

    #[test]
    fn test_list_collections_request() {
        let mut bucket = setup_bucket();
        let request = bucket.list_collections_request();
        assert_eq!(request.preparer.path, "/buckets/food/collections");
    }

    #[test]
    fn test_delete_collections_request() {
        let mut bucket = setup_bucket();
        let request = bucket.delete_collections_request();
        assert_eq!(request.preparer.path, "/buckets/food/collections");
    }

    #[test]
    fn test_create_collection_request() {
        let mut bucket = setup_bucket();
        let request = bucket.create_collection_request();
        assert_eq!(request.preparer.path, "/buckets/food/collections");
    }
}

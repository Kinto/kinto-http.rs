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
}


impl Bucket {
    /// Create a new bucket resource.
    pub fn new(client: KintoClient) -> Self {
        Bucket {
            client: client,
            data: None,
            permissions: None
        }
    }

    pub fn new_by_id<'a>(client: KintoClient, id: &'a str) -> Self {
        Bucket {
            client: client,
            data: json!({"id": id}).into(),
            permissions: None
        }
    }

    /// Get a collection by id.
    pub fn collection<'a>(self, id: &'a str) -> Collection {
        return Collection::new_by_id(self.client.clone(), self, id);
    }

    /// Get an empty collection.
    pub fn new_collection(&self) -> Collection {
        return Collection::new(self.client.clone(), self.clone());
    }

    /// List the names of all available collections.
    pub fn list_collections(&self) -> Result<Vec<String>, KintoError> {
        let response = try!(self.list_collections_request().send());
        // XXX: we should follow possible subrequests
        Ok(unwrap_collection_ids(response))
    }

    /// Delete all available collections.
    pub fn delete_collections(&self) -> Result<(), KintoError> {
        try!(self.delete_collections_request().send());
        Ok(())
    }

    /// Create a custom list collections request.
    pub fn list_collections_request(&self) -> GetCollection {
        GetCollection::new(self.client.clone(),
                           Paths::Collections(self.get_id().unwrap()).into())
    }

    /// Create a custom delete collections request.
    pub fn delete_collections_request(&self) -> DeleteCollection {
        DeleteCollection::new(self.client.clone(),
                              Paths::Collections(self.get_id().unwrap()).into())
    }

    /// Create a custom create collection request.
    pub fn create_collection_request(&self) -> CreateRecord {
        CreateRecord::new(self.client.clone(),
                          Paths::Collections(self.get_id().unwrap()).into())
    }
}


impl Resource for Bucket {
    fn unwrap_response(&mut self, wrapper: ResponseWrapper){
        *self = wrapper.into()
    }

    fn get_data(&self) -> Option<&Value> {
       self.data.as_ref()
    }

    fn load_request(&self) -> GetRecord {
        GetRecord::new(self.client.clone(),
                       Paths::Bucket(self.get_id().unwrap()).into())
    }

    fn create_request(&self) -> CreateRecord {
        CreateRecord::new(self.client.clone(),
                          Paths::Buckets().into())
    }

    fn update_request(&self) -> UpdateRecord {
        UpdateRecord::new(self.client.clone(),
                          Paths::Bucket(self.get_id().unwrap()).into())
    }

    fn delete_request(&self) -> DeleteRecord {
        DeleteRecord::new(self.client.clone(),
                          Paths::Bucket(self.get_id().unwrap()).into())
    }
}


impl From<ResponseWrapper> for Bucket {
    fn from(wrapper: ResponseWrapper) -> Self {

        let bucket: Bucket = serde_json::from_value(wrapper.body).unwrap();

        Bucket {
            client: wrapper.client,
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
        let client = setup_client();
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
        assert_eq!(collection.get_id().unwrap(), "meat");
        assert!(collection.data != None);
    }

    #[test]
    fn test_new_collection() {
        let bucket = setup_bucket();
        let collection = bucket.new_collection();
        assert_eq!(collection.data, None);
        assert_eq!(collection.get_id(), None);
    }

    #[test]
    fn test_list_collections() {
        let mut bucket = setup_bucket();
        bucket.create().unwrap();
        assert_eq!(bucket.list_collections().unwrap().len(), 0);
        let mut collection = bucket.new_collection();
        collection.set().unwrap();
        assert_eq!(bucket.list_collections().unwrap().len(), 1);
    }

    #[test]
    fn test_delete_collections() {
        let mut bucket = setup_bucket();
        bucket.create().unwrap();
        bucket.new_collection().set().unwrap();
        assert_eq!(bucket.list_collections().unwrap().len(), 1);
        bucket.delete_collections().unwrap();
        assert_eq!(bucket.list_collections().unwrap().len(), 0);
    }

    #[test]
    fn test_list_collections_request() {
        let bucket = setup_bucket();
        let request = bucket.list_collections_request();
        assert_eq!(request.preparer.path, "/buckets/food/collections");
    }

    #[test]
    fn test_delete_collections_request() {
        let bucket = setup_bucket();
        let request = bucket.delete_collections_request();
        assert_eq!(request.preparer.path, "/buckets/food/collections");
    }

    #[test]
    fn test_create_collection_request() {
        let bucket = setup_bucket();
        let request = bucket.create_collection_request();
        assert_eq!(request.preparer.path, "/buckets/food/collections");
    }
}

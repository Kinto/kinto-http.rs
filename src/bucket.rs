use serde_json;
use serde_json::Value;

use KintoConfig;
use error::KintoError;
use request::KintoRequest;
use response::ResponseWrapper;
use resource::Resource;
use collection::Collection;

use utils::unwrap_collection_records;


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BucketPermissions {
    #[serde(skip_serializing_if="Option::is_none")]
    pub read: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub write: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none", rename="collection:create")]
    pub create_collection: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none", rename="group:create")]
    pub create_group: Option<Vec<String>>,
}


#[derive(Debug, Clone)]
pub struct Bucket {
    pub data: Option<Value>,
    pub permissions: BucketPermissions,
    pub id: Option<String>,
    pub config: KintoConfig,
}


impl Bucket {
    /// Create a new bucket resource.
    pub fn new(config: KintoConfig) -> Self {
        Bucket {
            config: config,
            data: None,
            permissions: BucketPermissions::default(),
            id: None,
        }
    }

    pub fn new_by_id(config: KintoConfig, id: &str) -> Self {
        Bucket {
            config: config,
            data: None,
            permissions: BucketPermissions::default(),
            id: Some(id.to_owned()),
        }
    }

    /// Get a collection by id.
    pub fn collection(self, id: &str) -> Collection {
        Collection::new_by_id(self, id)
    }

    /// Get an empty collection.
    pub fn new_collection(&self) -> Collection {
        Collection::new(self.clone())
    }

    /// List the names of all available collections.
    pub fn list_collections(&self) -> Result<Vec<Collection>, KintoError> {
        let response =
            try!(try!(self.new_collection().list_request()).follow_subrequests());
        Ok(unwrap_collection_records(&response, &self.new_collection()))
    }

    /// Delete all available collections.
    pub fn delete_collections(&self) -> Result<(), KintoError> {
        try!(try!(self.new_collection().delete_all_request()).follow_subrequests());
        Ok(())
    }
}


impl Resource for Bucket {
    fn resource_path(&self) -> Result<String, KintoError> {
        Ok("/buckets".to_owned())
    }

    fn unwrap_response(&mut self, wrapper: ResponseWrapper) {
        self.data = Some(wrapper.body["data"].to_owned());
        self.permissions = serde_json::from_value(wrapper.body["permissions"].to_owned())
            .unwrap();
        self.id = Some(wrapper.body["data"]["id"].as_str().unwrap().to_owned());
    }

    fn get_id(&self) -> Option<String> {
        // Try to get id from class
        match self.id {
            Some(ref id) => Some(id.clone()),

            // If none, try to get id from body
            None => {
                match self.data {
                    Some(ref data) => data["id"].as_str().map(|s| s.to_string()),
                    None => None,
                }
            }
        }
    }

    fn get_timestamp(&self) -> Option<u64> {
        match self.get_data() {
            Some(data) => {
                match data["lat_modified"].as_u64() {
                    Some(ts) => ts.into(),
                    None => None,
                }
            }
            None => None,
        }
    }

    fn get_data(&self) -> Option<Value> {
        self.data.clone()
    }

    fn set_data(&mut self, data: Value) -> Self {
        self.data = data.into();
        self.clone()
    }

    fn get_permissions(&self) -> Option<Value> {
        serde_json::to_value(&(self.permissions))
            .unwrap_or_default()
            .into()
    }

    fn get_config(&self) -> KintoConfig {
        self.config.clone()
    }
}


#[cfg(test)]
mod test_bucket_resource {
    use utils::tests::{setup_client, setup_bucket};
    use resource::Resource;
    use bucket::BucketPermissions;

    #[test]
    fn test_set_bucket() {
        let mut bucket = setup_bucket();
        bucket.set().unwrap();
        let data = bucket.data.unwrap();
        assert_eq!(data["id"], "food");
    }

    #[test]
    fn test_set_bucket_without_id() {
        let mut bucket = setup_bucket();
        bucket.id = None;
        bucket.set().unwrap();
        let data = bucket.data.unwrap();
        assert!(data["id"].as_str() != None);
    }

    #[test]
    fn test_create_bucket() {
        let mut bucket = setup_bucket();
        bucket.create().unwrap();
        let data = bucket.data.unwrap();
        assert_eq!(data["id"], "food");
    }

    #[test]
    fn test_create_bucket_without_id() {
        let mut bucket = setup_bucket();
        bucket.id = None;
        bucket.set().unwrap();
        let data = bucket.data.unwrap();
        assert!(data["id"].as_str() != None);
    }

    #[test]
    fn test_load_bucket() {
        let mut bucket = setup_bucket();
        bucket.set().unwrap();
        let create_data = bucket.data.unwrap();

        // Cleanup stored data to make sure load work
        bucket.data = json!({}).into();

        bucket.load().unwrap();
        let load_data = bucket.data.unwrap();

        assert_eq!(create_data, load_data);
    }

    #[test]
    fn test_create_bucket_with_data() {
        let mut bucket = setup_bucket();
        bucket.data = json!({"good": true}).into();
        bucket.create().unwrap();
        let data = bucket.data.unwrap();
        assert_eq!(data["good"].as_bool().unwrap(), true);
    }

    #[test]
    fn test_set_bucket_permissions() {
        let mut bucket = setup_bucket();
        let principals = Some(vec!["bob".to_owned()]);

        bucket.permissions.read = principals.clone();
        bucket.permissions.write = principals.clone();
        bucket.permissions.create_group = principals.clone();
        bucket.permissions.create_collection = principals.clone();

        bucket.set().unwrap();
        bucket.permissions = BucketPermissions::default();

        bucket.load().unwrap();
        let permissions = bucket.permissions;

        assert_eq!(permissions.read.unwrap()[0], "bob");
        assert_eq!(permissions.create_group.unwrap()[0], "bob");
        assert_eq!(permissions.create_collection.unwrap()[0], "bob");
        assert_eq!(permissions.write.unwrap().len(), 2);
    }

    #[test]
    fn test_load_bucket_fails_without_id() {
        let mut bucket = setup_bucket();
        bucket.id = None;
        bucket.load().unwrap_err();
    }

    #[test]
    fn test_create_bucket_fails_on_existing() {
        let mut bucket = setup_bucket();

        // Create
        bucket.create().unwrap();

        // Tries to create again
        bucket.create().unwrap_err();
    }

    #[test]
    fn test_load_bucket_fails_on_not_existing() {
        let mut bucket = setup_bucket();
        bucket.load().unwrap_err();
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
        bucket.update().unwrap_err();
    }
}


#[cfg(test)]
mod test_bucket_class {
    use utils::tests::setup_bucket;
    use resource::Resource;

    #[test]
    fn test_get_collection() {
        let bucket = setup_bucket();
        let collection = bucket.collection("meat");
        assert_eq!(collection.get_id().unwrap(), "meat");
        assert_eq!(collection.data, None);
    }

    #[test]
    fn test_new_collection() {
        let bucket = setup_bucket();
        let collection = bucket.new_collection();
        assert_eq!(collection.data, None);
        assert_eq!(collection.get_id(), None);
    }
}

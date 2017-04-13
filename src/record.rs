use serde_json;
use serde_json::Value;

use KintoConfig;
use error::KintoError;
use response::ResponseWrapper;
use resource::Resource;
use collection::Collection;


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordPermissions {
    #[serde(skip_serializing_if="Option::is_none")]
    pub read: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub write: Option<Vec<String>>,
}


#[derive(Debug, Clone)]
pub struct Record {
    pub data: Option<Value>,
    pub permissions: RecordPermissions,
    pub collection: Collection,
    pub id: Option<String>,
}


impl Record {
    /// Create a new record object without an id.
    pub fn new(collection: Collection) -> Self {
        Record {
            collection: collection.clone(),
            data: None,
            permissions: RecordPermissions::default(),
            id: None,
        }
    }

    /// Create a new record object with an id.
    pub fn new_by_id(collection: Collection, id: &str) -> Self {
        Record {
            collection: collection,
            data: None,
            permissions: RecordPermissions::default(),
            id: Some(id.to_owned()),
        }
    }
}


impl Resource for Record {
    fn resource_path(&self) -> Result<String, KintoError> {
        Ok(format!("{}/records", try!(self.collection.record_path())))
    }

    fn unwrap_response(&mut self, wrapper: ResponseWrapper) {
        self.data = Some(wrapper.body["data"].to_owned());
        self.permissions = serde_json::from_value(wrapper.body["permissions"].to_owned())
            .unwrap();
        self.id = Some(wrapper.body["data"]["id"].as_str().unwrap().to_owned());
    }

    fn get_config(&self) -> KintoConfig {
        self.collection.get_config()
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

    fn get_id(&self) -> Option<String> {
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
        record.create().unwrap_err();
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
        record.load().unwrap_err();
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
        record.update().unwrap_err();
    }
}

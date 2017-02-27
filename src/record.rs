use json::JsonValue;

use KintoClient;
use paths::Paths;
use request::{GetRecord, UpdateRecord, DeleteRecord};
use response::ResponseWrapper;
use resource::Resource;
use bucket::Bucket;
use collection::Collection;
use utils::{extract_ids_from_path};


#[derive(Debug, Clone)]
pub struct Record {
    pub client: KintoClient,
    pub bucket: Bucket,
    pub collection: Collection,
    pub id: String,
    pub timestamp: Option<u64>,
    pub data: Option<JsonValue>,
    pub permissions: Option<JsonValue>,
}


impl Record {

    /// Create a new record resource.
    pub fn new<'a>(client: KintoClient, collection: Collection, id: &'a str) -> Self {
        Record {
            client: client,
            bucket: collection.bucket.clone(),
            collection: collection,
            id: id.to_owned(),
            timestamp: None,
            data: None,
            permissions: None
        }
    }
}


impl Resource for Record {

    fn unwrap_response(&mut self, wrapper: ResponseWrapper){
        *self = wrapper.into()
    }

    fn get_data(&mut self) ->  Option<JsonValue> {
        self.data.clone()
    }

    fn get_permissions(&mut self) ->  Option<JsonValue> {
        self.permissions.clone()
    }

    fn get_timestamp(&mut self) -> Option<u64> {
        self.timestamp
    }

    fn load_request(&mut self) -> GetRecord {
        GetRecord::new(self.client.clone(),
                       Paths::Record(self.bucket.id.as_str(),
                                     self.collection.id.as_str(),
                                     self.id.as_str()).into())
    }

    fn update_request(&mut self) -> UpdateRecord {
        UpdateRecord::new(self.client.clone(),
                          Paths::Record(self.bucket.id.as_str(),
                                        self.collection.id.as_str(),
                                        self.id.as_str()).into())
    }

    fn delete_request(&mut self) -> DeleteRecord {
        DeleteRecord::new(self.client.clone(),
                          Paths::Record(self.bucket.id.as_str(),
                                        self.collection.id.as_str(),
                                        self.id.as_str()).into())
    }
}


impl From<ResponseWrapper> for Record {
    fn from(wrapper: ResponseWrapper) -> Self {
        let timestamp = wrapper.json["data"]["last_modified"].as_number().unwrap();

        let path_ids = extract_ids_from_path(wrapper.path);

        let bucket_id = path_ids["buckets"].clone().unwrap();
        let collection_id = path_ids["collections"].clone().unwrap();

        let bucket = Bucket::new(wrapper.client.clone(),
                                 bucket_id.as_str());
        let collection = Collection::new(wrapper.client.clone(),
                                         bucket.clone(),
                                         collection_id.as_str());;

        Record {
            client: wrapper.client,
            bucket: bucket,
            collection: collection,
            data: wrapper.json["data"].to_owned().into(),
            permissions: wrapper.json["permissions"].to_owned().into(),
            id: wrapper.json["data"]["id"].to_string(),
            timestamp: Some(timestamp.into())
        }
    }
}


impl Into<JsonValue> for Record {
    fn into(self) -> JsonValue {
        let mut obj = JsonValue::new_object();
        match self.data {
            Some(data) => obj["data"] = data,
            None => ()
        }
        match self.permissions {
            Some(perms) => obj["permissions"] = perms,
            None => ()
        }
        return obj;
    }
}


#[cfg(test)]
mod test_record {
    use resource::Resource;
    use utils::tests::{setup_record, setup_collection};

    #[test]
    fn test_create_record() {
        let mut record = setup_record();
        record.data = object!{"good" => true}.into();

        record.create().unwrap();
        let data = record.data.unwrap().to_owned();

        assert_eq!(data["id"], "entrecote");
        assert_eq!(data["good"], true);
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
        record.data = object!{}.into();

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

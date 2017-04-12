use std::convert::Into;


/// Known paths in the kinto server.
pub enum Paths<'a> {
    Batch,
    Buckets,
    Bucket(&'a str),
    Groups(&'a str),
    Group(&'a str, &'a str),
    Collections(&'a str),
    Collection(&'a str, &'a str),
    Records(&'a str, &'a str),
    Record(&'a str, &'a str, &'a str),
}


impl<'a> Into<String> for Paths<'a> {
    fn into(self) -> String {
        match self {
            Paths::Batch => "/batch".to_owned(),
            Paths::Buckets => "/buckets".to_owned(),
            Paths::Bucket(id) => format!("/buckets/{id}", id = id),
            Paths::Groups(bucket_id) => {
                format!("/buckets/{bucket_id}/groups", bucket_id = bucket_id)
            }
            Paths::Group(bucket_id, id) => {
                format!("/buckets/{bucket_id}/groups/{id}",
                        bucket_id = bucket_id,
                        id = id)
            }
            Paths::Collections(bucket_id) => {
                format!("/buckets/{bucket_id}/collections", bucket_id = bucket_id)
            }
            Paths::Collection(bucket_id, id) => {
                format!("/buckets/{bucket_id}/collections/{id}",
                        bucket_id = bucket_id,
                        id = id)
            }
            Paths::Records(bucket_id, collection_id) => {
                format!("/buckets/{bucket_id}/collections/{collection_id}/records",
                        bucket_id = bucket_id,
                        collection_id = collection_id)
            }
            Paths::Record(bucket_id, collection_id, id) => {
                format!("/buckets/{bucket_id}/collections/{collection_id}/records/{id}",
                        bucket_id = bucket_id,
                        collection_id = collection_id,
                        id = id)
            }
        }
    }
}


#[cfg(test)]
mod test_paths {
    use super::Paths;

    #[test]
    fn test_batch_path() {
        let path: String = Paths::Batch.into();
        assert_eq!(path, "/batch");
    }

    #[test]
    fn test_buckets_path() {
        let path: String = Paths::Buckets.into();
        assert_eq!(path, "/buckets");
    }

    #[test]
    fn test_bucket_path() {
        let path: String = Paths::Bucket("food").into();
        assert_eq!(path, "/buckets/food");
    }

    #[test]
    fn test_groups_path() {
        let path: String = Paths::Groups("food").into();
        assert_eq!(path, "/buckets/food/groups");
    }

    #[test]
    fn test_group_path() {
        let path: String = Paths::Group("food", "storage_team").into();
        assert_eq!(path, "/buckets/food/groups/storage_team");
    }

    #[test]
    fn test_collections_path() {
        let path: String = Paths::Collections("food").into();
        assert_eq!(path, "/buckets/food/collections");
    }

    #[test]
    fn test_collection_path() {
        let path: String = Paths::Collection("food", "meat").into();
        assert_eq!(path, "/buckets/food/collections/meat");
    }

    #[test]
    fn test_records_path() {
        let path: String = Paths::Records("food", "meat").into();
        assert_eq!(path, "/buckets/food/collections/meat/records");
    }

    #[test]
    fn test_record_path() {
        let path: String = Paths::Record("food", "meat", "entrecote").into();
        assert_eq!(path, "/buckets/food/collections/meat/records/entrecote");
    }
}

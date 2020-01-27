extern crate rusoto_core;
extern crate rusoto_s3;


use std::collections::HashMap;

use rusoto_core::region::Region::Custom;
use rusoto_s3::{PutObjectRequest, S3Client};

use crate::errors::Result;
use crate::middlewares::requests::request;
use self::rusoto_s3::S3;

pub struct CloudStorage { }

impl CloudStorage {
    pub fn put(b64_file: &String) -> Result<()> {
        let mut request = PutObjectRequest::default();

        request.bucket = "public".to_string();
        request.key = "test.jpeg".to_string();
        request.body = Some(base64::decode(&b64_file)?.into());

        CloudStorage::get_client().put_object(request).sync()?;
        Ok(())
    }

    fn get_client() -> S3Client {
        let region = Custom {
            name: "us-east-1".to_string(),
            endpoint: "http://minio:9000".to_string(),
        };

        S3Client::new(region)
    }
}

pub fn test_request() -> Result<HashMap<String, String>> {
    request(|client| {
        let res: HashMap<String, String> = client.get("https://httpbin.org/ip").send()?.json()?;
        println!("{:#?}", res);

        Ok(res)
    })
}

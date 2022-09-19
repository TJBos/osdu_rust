use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub struct SimpleClient {
    pub storage: StorageService
    // pub search: SearchService
    //pub dataset: DatasetService
    //...
}

impl SimpleClient {
    pub fn new(access_token: &str, base_api_url: &str) -> SimpleClient {
        SimpleClient {
           storage: StorageService::new(base_api_url, access_token)
        }
    }
}

pub struct StorageService {
    pub service_path: String,
    pub http_client: BaseHttpClient
}

impl StorageService {
    pub fn new(base_api_url: &str, access_token: &str) -> StorageService {
        StorageService { service_path: "/api/storage/v2/records".to_string(),
            http_client: BaseHttpClient::new(base_api_url, access_token)
        }
    }
    pub async fn get_record(&self, record_id: &str) -> RecordBase {
      let response = self.http_client.get_request(&self.service_path, Param::Path(record_id.to_string()))
      .await;
      println!{"{}", response.status()};
      response.json::<RecordBase>().await.unwrap()
    }
    // pub async fn store_record() -> StoreRecordResponse {

    // } 

}

pub struct BaseHttpClient {
    pub base_api_url: String,
    pub access_token: String,
    pub data_partition_id: String
}

impl BaseHttpClient {

    pub fn new(base_api_url: &str, access_token: &str) -> BaseHttpClient {
        BaseHttpClient {
            base_api_url: base_api_url.to_string(),
            access_token: access_token.to_string(),
            data_partition_id: "osdu".to_string()
        }
    }

     fn construct_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("AUTHORIZATION", format!("Bearer {}", self.access_token).parse().unwrap());
        headers.insert("CONTENT_TYPE", "application/json".parse().unwrap());
        headers.insert("ACCEPT", "application/json".parse().unwrap());
        headers.insert("data-partition-id", self.data_partition_id.parse().unwrap());
        headers
    }

    async fn get_request(&self, service_path: &str, param: Param) -> reqwest::Response {
        let request_url = format!("{}{}/{}",&self.base_api_url, service_path, get_url_params(param));
        println!("{}", request_url);
        let client = reqwest::Client::new();
        let response = client.get(request_url)
        .headers(self.construct_headers())
        .send()
        .await
        .unwrap();
        response
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RecordBase {
    pub id: String,
    pub kind: String,
    pub data: HashMap<String, Value>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StoreRecordResponse {
    pub record_count: i16,
    pub record_ids: Vec<String>
}

enum Param {
    Path(String),
    Query(HashMap<String, String>)
}

fn get_url_params(param: Param) -> String {
    match param {
        Param::Path(path_param) => return path_param,
        Param::Query(query_params) => {
            let mut url_path = String::new();
            for (key, value) in query_params.into_iter() {
                url_path += &(key + "?" + &value);
            }
            return url_path;
        }

    }
}

// How to represent a "record" in Rust? 

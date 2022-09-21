use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use aws_config;
use aws_sdk_ssm;
use aws_sdk_secretsmanager;
use base64;



pub struct Client {
    pub storage: StorageService
    // pub search: SearchService
    //pub dataset: DatasetService
    //...
}

impl Client {
    // create a new simple osdu client by passing in the access token
    pub fn simple(access_token: &str, base_api_url: &str) -> Client {
        Client {
           storage: StorageService::new(base_api_url, access_token)
        }
    }
    // create a new aws service principal client, it gets a token from the service principal
    pub async fn aws_service_principal(base_api_url: &str, resource_prefix: &str, profile: &str, region: &'static str) -> Client {
        let token_response: TokenResponse = get_svp_token(profile, resource_prefix, region).await;

        Client {
            storage: StorageService::new(base_api_url, &token_response.access_token)
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

// stuff for service principal client

async fn get_svp_token(profile: &str, resource_prefix: &str, region: &'static str) -> TokenResponse {
    let token_url_ssm_path = format!("/osdu/{resource_prefix}/oauth-token-uri");
    let aws_oauth_custom_scope_ssm_path = format!("/osdu/{resource_prefix}/oauth-custom-scope");
    let client_id_ssm_path = format!("/osdu/{resource_prefix}/client-credentials-client-id");
    let client_secret_name = format!("/osdu/{resource_prefix}/client_credentials_secret");
    
    let config = aws_config::from_env()
    .credentials_provider(
        aws_config::profile::ProfileFileCredentialsProvider::builder()
        .profile_name(profile)
        .build()
    )
    .region(region)
    .load()
    .await;

    let client_id = get_parameter(&config, &client_id_ssm_path).await;
    let token_url = get_parameter(&config, &token_url_ssm_path).await;
    let aws_oauth_custom_scope = get_parameter(&config, &aws_oauth_custom_scope_ssm_path).await;
    let client_secret = get_secret(&config, &client_secret_name).await;

    let auth = format!("{client_id}:{client_secret}");
    println!("{}", auth);
    let encoded_auth = base64::encode(auth);

    let token_url = format!("{token_url}?grant_type=client_credentials&client_id={client_id}&scope={aws_oauth_custom_scope}");

    let client = reqwest::Client::new();
    let response = client.post(token_url)
        .header("AUTHORIZATION", format!("Basic {encoded_auth}"))
        .header("CONTENT-TYPE", "application/x-www-form-urlencoded")
        .send()
        .await
        .unwrap();
        println!{"{}", response.status()};
        response.json::<TokenResponse>().await.unwrap()
}

 async fn get_parameter(config: &aws_config::SdkConfig, param_name: &str) -> String {
    let client = aws_sdk_ssm::Client::new(&config);
    let builder = client.get_parameter();
    let resp = builder.name(param_name).send().await.unwrap();
    let parameter = resp.parameter().unwrap().value().unwrap();
    parameter.to_string()
}

async fn get_secret(config: &aws_config::SdkConfig, secret_name: &str) -> String {
    let client = aws_sdk_secretsmanager::Client::new(&config);
    let resp = client.get_secret_value().secret_id(secret_name).send().await.unwrap();
    let secret = resp.secret_string().unwrap();
    let object = serde_json::from_str::<HashMap<String,String>>(secret).unwrap();
    let stringvalue = object.get("client_credentials_client_secret").unwrap();
    stringvalue.to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    access_token: String,
    expires_in: i32
}
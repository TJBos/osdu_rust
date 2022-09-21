extern crate osdu_rust;

#[tokio::main]
async fn main() {

  //   let access_token = "";
  //  let base_url = "https://datagumbo.osdu.dev";
  //  let client = osdu_rust::client::SimpleClient::new(access_token, base_url);
  //  let response = client.storage.get_record("osdu:dataset--File.Generic:Afe1234").await;
  //  println!("{:?}", response);

  //let param = osdu_rust::client::get_parameter("/osdu/osdur3m11/client-credentials-client-id").await;
  //println!("{:?}", param);

  //let secret = osdu_rust::client::get_secret("/osdu/osdur3m11/client_credentials_secret").await;
  //println!("{:?}", secret);

  let base_url = "https://datagumbo.osdu.dev";
  let resource_prefix = "osdur3m11";
  let region = "us-east-2";
  let profile = "osdu-datagumbo";
  let client = osdu_rust::client::Client::aws_service_principal(base_url, resource_prefix, profile, region).await;
  let response = client.storage.get_record("osdu:dataset--File.Generic:Afe1234").await;
  println!("{:?}", response);
}
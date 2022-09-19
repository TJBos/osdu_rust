extern crate osdu_rust;

// tokio let's us use "async" on our main function

#[tokio::main]
async fn main() {

    let access_token = "";
   let base_url = "https://datagumbo.osdu.dev";
   let client = osdu_rust::client::SimpleClient::new(access_token, base_url);
   let response = client.storage.get_record("osdu:dataset--File.Generic:Afe1234").await;
   println!("{:?}", response);

}
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Info {
    enrUri: String,
    listenAddresses: [String; 2]
}
pub(crate) async fn get_node_details() -> Result<(String), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let request = client.request(reqwest::Method::GET, "http://127.0.0.1:21161/debug/v1/info");

    let response = request.send().await?;
    let info : Info = response.json().await?;

    println!("enr_uri is `{}`", info.enrUri);
    println!("listen_addresses is `{:?}`", info.listenAddresses);

    Ok((info.enrUri))
}

pub(crate) async fn subscribe_topic(port: &str) -> Result<(String), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "text/plain".parse()?);

    let data = "[\"/my-app/2/chatroom-1/proto\"]";

    let request = client.request(reqwest::Method::POST, "http://".to_owned() + &*port + "/relay/v1/auto/subscriptions?accept=text/plain&content-type=application/json")
        .headers(headers)
        .body(data);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("subsscribe topic {}", body);
    Ok((body))
}

pub(crate) async fn publish_message(host_port: &str) -> Result<(String), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", "application/json".parse()?);

    let data = r#"{
    "payload": "UmVsYXkgd29ya3MhIQ==",
    "contentTopic": "/my-app/2/chatroom-1/proto",
    "timestamp": 0
    }"#;

    let json: serde_json::Value = serde_json::from_str(&data)?;

    let request = client.request(reqwest::Method::POST, "http://".to_owned() + &*host_port + "/relay/v1/auto/messages")
        .headers(headers)
        .json(&json);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("message publish {}", body);
    Ok((body))
}

pub(crate) async fn confirm_message(host_port: &str) -> Result<(String), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let request = client.request(reqwest::Method::GET, "http://".to_owned() + &*host_port + "/relay/v1/auto/messages/%2Fmyapp%2F2%2Fchatroom-1%2Fproto");

    let response = request.send().await?;
    let body = response.text().await?;
    println!("message on topic : {}", body);
    Ok((body))
}

pub(crate) async fn get_peers() -> Result<(String), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let request = client.request(reqwest::Method::GET, "http://127.0.0.1:21161/admin/v1/peers");

    let response = request.send().await?;
    let body = response.text().await?;

    println!("multiaddr is------ {}", body);
    Ok((body))
}
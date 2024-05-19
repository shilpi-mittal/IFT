use std::{thread};
use thread::sleep;
use crate::{constants};
use crate::docker_utils::{create_network, remove_network, start_docker, stop_docker};
use crate::helpers::{get_node_details, subscribe_topic, get_peers, publish_message};

#[tokio::test]
async fn test_case_2() {
    //create network
    sleep(constants::FIVE_SECS);
    create_network().await;
    sleep(constants::TEN_SECS);

    //starting first docker
    start_docker("docker1", "waku", "127.0.0.1", 21161.into(),
                               21162.into(), 21163.into(), 21164.into(), 21165.into(), "172.18.0.2", "".to_string()).await;
    sleep(constants::FIVE_SECS);

    //Get enr_uri
    let env_uri:String = get_node_details().await.unwrap().into();
    assert_ne!(env_uri.to_string(), "");

    // Register to topic in node 1
    let subscribe_result = subscribe_topic("127.0.0.1:21161").await;
    assert_eq!(subscribe_result.unwrap(), "OK", "subscribe response is not OK");

    //starting second docker
    start_docker("docker2", "waku", "127.0.0.1", 21166.into(),
                               21167.into(), 21168.into(), 21169.into(), 21170.into(), "172.18.0.3", env_uri).await;

    sleep(constants::TEN_SECS);
    sleep(constants::TEN_SECS);


    let peer_result = get_peers().await;
    assert!(peer_result.unwrap().contains("/ip4/172.18.0.3"), "node 1 peer result doesn't contain node 2");

    // Register to topic node 2
    let subscribe_result1 = subscribe_topic("127.0.0.1:21166").await;
    assert_eq!(subscribe_result1.unwrap().to_string(), "OK", "OK not equal");

    // publishing the message on first node
    let publish_result = publish_message("127.0.0.1:21161").await;
    assert_eq!(publish_result.unwrap(), "OK", "publish response not OK");

    // confirming message was consumes on second node
    let confirmation_result = crate::helpers::confirm_message("127.0.0.1:21166").await;
    assert_eq!(confirmation_result.unwrap(), "/myapp/2/chatroom-1/proto", "confirmation response is incorrect");

    stop_docker("docker1").await;
    stop_docker("docker2").await;
    remove_network().await;
}



use std::{thread};
use thread::sleep;
use crate::{constants};
use crate::docker_utils::{start_docker, stop_docker};
use crate::helpers::{confirm_message, get_node_details, publish_message, subscribe_topic};

#[tokio::test]
async fn test_case_1() {
    //starting docker
    start_docker("docker1", "default", "127.0.0.1", 21161.into(),
                               21162.into(), 21163.into(), 21164.into(), 21165.into(),
                               "172.18.0.2", "".to_string()).await;
    sleep(constants::FIVE_SECS);

    //Get enr_uri
    let env_uri :String = get_node_details().await.unwrap().into();
    assert_ne!(env_uri.to_string(), "");

    // Register to topic
    let subscribe_result = subscribe_topic("127.0.0.1:21161").await;
    assert_eq!(subscribe_result.unwrap(), "OK",  "subscribe response is not OK");

    // publishing the message
    let publish_result = publish_message("127.0.0.1:21161").await;
    assert_eq!(publish_result.unwrap(), "OK",  "publish response is not OK");

    // confirming message was published
    let confirmation_result = confirm_message("127.0.0.1:21161").await;
    assert_eq!(confirmation_result.unwrap(), "/myapp/2/chatroom-1/proto",  "confirmation response is incorrect");

    stop_docker("docker1").await;
}
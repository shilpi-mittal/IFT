use std::process::Stdio;
use docker_command::{BaseCommand, Launcher, PortRange, PublishPorts, RunOpt, StopOpt};
use execute::Execute;

pub(crate) async fn start_docker(name: &str, network_name: &str, ip: &str, port1: PortRange, port2: PortRange,
                                 port3: PortRange, port4: PortRange, port5: PortRange, extip: &str, env_uri: String) {
    let output = Launcher::from(BaseCommand::Docker)
        .run(RunOpt {
            image: "wakuorg/nwaku:v0.24.0".into(),
            init: true,
            interactive: true,
            detach: true,
            name: Some(name.into()),
            network: Some(network_name.into()),
            publish: vec![PublishPorts {
                container: port1.clone().into(),
                host: Some(port1.clone().into()),
                ip: Some(ip.into()),
            }, PublishPorts {
                container: port2.clone().into(),
                host: Some(port2.clone().into()),
                ip: Some(ip.into()),
            }, PublishPorts {
                container: port3.clone().into(),
                host: Some(port3.clone().into()),
                ip: Some(ip.into()),
            }, PublishPorts {
                container: port4.clone().into(),
                host: Some(port4.clone().into()),
                ip: Some(ip.into()),
            }, PublishPorts {
                container: port5.clone().into(),
                host: Some(port5.clone().into()),
                ip: Some(ip.into()),
            }],
            args: vec!["--listen-address=0.0.0.0".into(),
                       "--rest=true".into(),
                       "--rest-admin=true".into(),
                       "--websocket-support=true".into(),
                       "--log-level=TRACE".into(),
                       "--rest-relay-cache-capacity=100".into(),
                       ("--websocket-port=".to_string() + &*port3.to_string()).into(),
                       ("--rest-port=".to_string() + &*port1.to_string()).into(),
                       ("--tcp-port=".to_string() + &*port2.to_string()).into(),
                       ("--discv5-udp-port=".to_string() + &*port4.to_string()).into(),
                       "--rest-address=0.0.0.0".into(),
                       ("--nat=extip:".to_string() + &*extip.to_string()).into(),
                       "--peer-exchange=true".into(),
                       "--discv5-discovery=true".into(),
                       "--relay=true".into(),
                       ("--discv5-bootstrap-node=".to_string() + &*env_uri.to_string()).into(),
            ],
            ..Default::default()
        })
        .enable_capture()
        .run();
}

// async fn create_network (docker: Docker)-> Result<(), Box<dyn std::error::Error>> {
//     // let output = Command::new("docker network create --driver bridge --subnet 172.18.0.0/16 --gateway 172.18.0.1 waku").output();
//     // println!("status: {}", output.expect("success").status);
//     // assert!(output.status.success());
//
//     // let docker = Docker::connect_with_unix_defaults().unwrap();
//     // let options = CreateNetworkOptions {
//     //     name: "new_network",
//     //     driver: "bridge",
//     //     ..Default::default()
//     // };
//     //
//     // let network = docker.create_network(options).await.unwrap();
//
//     let ipam_config = IpamConfig {
//         subnet: Some(String::from("172.18.0.0/16")),
//         gateway: Some(String::from("172.18.0.1")),
//         ..Default::default()
//     };
//
//     let create_network_options = CreateNetworkOptions {
//         name: "waku",
//         check_duplicate: true,
//         driver: "bridge",
//         ipam: Ipam {
//             config: Some(vec![ipam_config]),
//             ..Default::default()
//         },
//         ..Default::default()
//     };
//     let result = &docker.create_network(create_network_options).await?;
//     Ok(())
// }

pub(crate) async fn create_network() {
    let mut command1 = execute::command_args!("docker", "network", "create", "--driver", "bridge", "--subnet", "172.18.0.0/16", "--gateway", "172.18.0.1", "waku");
    command1.stdout(Stdio::piped());

    let output = command1.execute_output().unwrap();

    println!("{}", String::from_utf8(output.stdout).unwrap());
}

pub(crate) async fn remove_network() {
    let out = Launcher::from(BaseCommand::Docker)
        .remove_network("waku".into())
        .enable_capture()
        .run();
    println!("Network removed");
}

pub(crate) async fn stop_docker(name: &str) {
    let out = Launcher::from(BaseCommand::Docker)
        .stop(StopOpt {
            containers: vec![name.into()],
            time: Some(123),
        }).enable_capture()
        .run();
    println!("Docker stopped");

    let mut command1 = execute::command_args!("docker", "rm", "-f", name);
    command1.stdout(Stdio::piped());

    let output = command1.execute_output().unwrap();

    println!("{}", String::from_utf8(output.stdout).unwrap());
}

// trying different crate for docker commands
// async fn create_network (docker: Docker)-> Result<(), Box<dyn std::error::Error>> {
//     // let output = Command::new("docker network create --driver bridge --subnet 172.18.0.0/16 --gateway 172.18.0.1 waku").output();
//     // println!("status: {}", output.expect("success").status);
//     // assert!(output.status.success());
//
//     // let docker = Docker::connect_with_unix_defaults().unwrap();
//     // let options = CreateNetworkOptions {
//     //     name: "new_network",
//     //     driver: "bridge",
//     //     ..Default::default()
//     // };
//     //
//     // let network = docker.create_network(options).await.unwrap();
//
//     let ipam_config = IpamConfig {
//         subnet: Some(String::from("172.18.0.0/16")),
//         gateway: Some(String::from("172.18.0.1")),
//         ..Default::default()
//     };
//
//     let create_network_options = CreateNetworkOptions {
//         name: "waku",
//         check_duplicate: true,
//         driver: "bridge",
//         ipam: Ipam {
//             config: Some(vec![ipam_config]),
//             ..Default::default()
//         },
//         ..Default::default()
//     };
//     let result = &docker.create_network(create_network_options).await?;
//     Ok(())
// }



// args: vec!["--subnet 172.18.0.0".into(),
// "--driver bridge".into(),
// "--gateway 172.18.0.1".into()]
// --driver bridge --subnet 172.18.0.0/16 --gateway 172.18.0.1

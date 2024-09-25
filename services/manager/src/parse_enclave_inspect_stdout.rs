use regex::Regex;
use std::error::Error;

use super::{Port, Service};

pub fn parse(stdout: &str) -> Result<Vec<Service>, Box<dyn Error>> {
    let service_regex = Regex::new(r"(?m)^([0-9a-f]+)\s+(\S+)\s+(.*?)\s+(RUNNING|STOPPED)")?;
    let port_regex =
        Regex::new(r"(\w+):\s+(\d+)/(\w+)\s+->\s+(?:http://)?(?:\d+\.\d+\.\d+\.\d+:(\d+))?")?;

    let mut services = Vec::new();
    let mut current_service: Option<Service> = None;

    for line in stdout.lines() {
        if let Some(captures) = service_regex.captures(line) {
            if let Some(service) = current_service {
                services.push(service);
            }

            let mut new_service = Service {
                uuid: captures[1].to_string(),
                name: captures[2].to_string(),
                ports: Vec::new(),
            };

            // Check for ports on the same line as the service
            for port_captures in port_regex.captures_iter(&line) {
                new_service.ports.push(Port {
                    name: port_captures[1].to_string(),
                    internal: port_captures[2].parse()?,
                    external: port_captures[4].parse().unwrap_or(0),
                });
            }

            current_service = Some(new_service);
        } else if let Some(service) = current_service.as_mut() {
            for captures in port_regex.captures_iter(line) {
                service.ports.push(Port {
                    name: captures[1].to_string(),
                    internal: captures[2].parse()?,
                    external: captures[4].parse().unwrap_or(0),
                });
            }
        }
    }

    if let Some(service) = current_service {
        services.push(service);
    }

    Ok(services)
}

// EXAMPLE STDOUT STRING:
//
// Name:            monea-enclave
// UUID:            fbf89d96bb0c
// Status:          RUNNING
// Creation Time:   Tue, 17 Sep 2024 16:16:07 PDT
// Flags:

// ========================================= Files Artifacts =========================================
// UUID           Name
// 1114c1302562   1-lighthouse-geth-0-63-0
// d42bb2b7925a   el_cl_genesis_data
// d6f75b2b96f7   final-genesis-timestamp
// db3a24a77a37   genesis-el-cl-env-file
// 293c5da35df4   genesis_validators_root
// b65e84ce5fdc   jwt_file
// bbf921ece742   keymanager_file
// feefe1e98910   prysm-password
// 7b34bcfa047f   validator-ranges

// ========================================== User Services ==========================================
// UUID           Name                                             Ports                                         Status
// 34ba01fbbca6   cl-1-lighthouse-geth                             http: 4000/tcp -> http://127.0.0.1:61005      RUNNING
//                                                                 metrics: 5054/tcp -> http://127.0.0.1:61006
//                                                                 tcp-discovery: 9000/tcp -> 127.0.0.1:61007
//                                                                 udp-discovery: 9000/udp -> 127.0.0.1:52982
// cb133030f05d   el-1-geth-lighthouse                             engine-rpc: 8551/tcp -> 127.0.0.1:60997       RUNNING
//                                                                 metrics: 9001/tcp -> http://127.0.0.1:60998
//                                                                 rpc: 8545/tcp -> 127.0.0.1:60995
//                                                                 tcp-discovery: 30303/tcp -> 127.0.0.1:60999
//                                                                 udp-discovery: 30303/udp -> 127.0.0.1:55288
//                                                                 ws: 8546/tcp -> 127.0.0.1:60996
// 0f7c3003ba81   validator-key-generation-cl-validator-keystore   <none>                                        RUNNING
// 4944d6050a16   vc-1-geth-lighthouse                             metrics: 8080/tcp -> http://127.0.0.1:61009   RUNNING

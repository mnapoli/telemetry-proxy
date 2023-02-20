use std::thread;
use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::blocking::Client;
use serde_json::{json, Value};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888")
        .expect("Error: could not bind socket");

    println!("Listening on port 8888...");

    loop {
        let mut buf = [0u8; 1500];
        socket.try_clone().expect("Failed to clone socket");
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    let payload_with_null_bytes = String::from_utf8_lossy(&buf);
                    // Remove the null bytes
                    let json_payload = payload_with_null_bytes.trim_matches(char::from(0));

                    let payload: Value = serde_json::from_str(json_payload)
                        .expect("Failed to parse JSON payload");

                    let event_name = match payload["cli"].as_str().expect("Not a string") {
                        "sls" => "Serverless CLI - Command",
                        "vendor/bin/bref" => "Bref CLI - Command",
                        _ => "Unknown CLI - Command",
                    };

                    let data = json!([
                        {
                            "event": event_name,
                            "properties": {
                                "token": "5aa82249a4bf5e4a800ab88b6b725f92",
                                "distinct_id": payload["uid"],
                                "ip": src.ip().to_string(),
                                // Current time in milliseconds
                                "time": time(),
                                "bref_version": payload["v"],
                                "command": payload["c"],
                                // Timestamp of the first local installation
                                "sls_installation_date": payload["install"],
                            },
                        }
                    ]);

                    println!("Received event from {}", src);
                    println!("{}", json_payload);
                    println!("{}/1500", json_payload.len());

                    track_event(data.to_string());
                });
            }
            Err(e) => {
                eprintln!("Error: couldn't receive a datagram: {}", e);
            }
        }
    }
}

fn track_event(buf: String) {
    let client = Client::new();
    // Make a POST HTTP request to the https://api-eu.mixpanel.com/track URL
    let response = client.post("https://api-eu.mixpanel.com/track")
        .header("Content-Type", "application/json")
        .header("Accept", "text/plain")
        .body(buf)
        .send()
        .expect("Failed to send a request");
    let status_code = response.status();
    let response_code = response.text().expect("Unable to read response");
    if status_code != 200 || response_code != "1" {
        eprintln!("Error: failed to send event to Mixpanel");
    }
}

// Returns the current time in milliseconds
fn time() -> u128 {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_millis()
}

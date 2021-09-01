#[macro_use(defer)]
extern crate scopeguard;

use std::process::Child;
use std::thread;
use std::time::Duration;
use utils::binary;
use utils::mock;

mod utils;

#[test]
fn it_forwards_to_the_remote_side() {
    let remote = mock::remote();
    // verify on the remote side
    let remote_thread = thread::spawn(move || {
        let message = "Hello World!";
        let mut buf = [0; 256];
        let bytes = remote.remote.recv(&mut buf).unwrap();
        let payload = &buf[..bytes];

        assert_eq!(bytes, message.as_bytes().len());
        assert_eq!(payload, message.as_bytes());
        println!("foo");
    });

    let proxy = start_udp_proxy();

    // Create a scope guard using `defer!` for the current scope
    defer! {
        proxy.kill();
    }

    // simulating a client
    let message = "Hello World!";
    let client = mock::client();
    let bytes = client.send(message.as_bytes()).unwrap();
    assert_eq!(bytes, message.as_bytes().len());

    remote_thread.join().unwrap();
}

fn start_udp_proxy() -> Child {
    let mut proxy = binary()
        .arg("--verbose")
        .arg("--udp")
        .arg("--server")
        .arg("localhost")
        .arg("--server-port")
        .arg("44310")
        .arg("--port")
        .arg("10443")
        .spawn()
        .unwrap();
    // wait a little to get the proxy bootstrapping
    thread::sleep(Duration::from_millis(500));

    proxy
}

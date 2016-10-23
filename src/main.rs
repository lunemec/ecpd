extern crate nanomsg;
extern crate rustc_serialize;

use std::io::Read;
use nanomsg::{Socket, Protocol};

use rustc_serialize::json;

#[derive(Debug, RustcDecodable)]
struct CopyJob {
    src: String,
    dst: String,
}


impl CopyJob {
    fn new(json: &str) -> CopyJob {
        let job: CopyJob = json::decode(json).unwrap();
        job
    }
}


fn main() {
    println!("ECPD started.");
    let mut input = Socket::new(Protocol::Pull).unwrap();

    input.bind("ipc:///tmp/pipeline.ipc");
    let mut text = String::new();

    // Here we must spawn 1 additional thread that does the actual copying and this main thread
    // listens on the IPC.
    loop {
        input.read_to_string(&mut text);
        println!("Added CopyJob: {:?}", CopyJob::new(&text));

        text.clear();
    }
}

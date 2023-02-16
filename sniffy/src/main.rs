use indicatif::ProgressIterator;
use rayon::prelude::{ParallelBridge, ParallelIterator};

fn main() {
    let ip = std::env::args()
        .nth(1)
        .expect("Please provide the IP address!");

    let _ports = (1..65535)
        .progress()
        .par_bridge()
        .map(|port| {
            let res = std::net::TcpStream::connect_timeout(
                &format!("{}:{}", ip, port).parse().unwrap(),
                std::time::Duration::from_millis(10),
            );

            if let Ok(_) = res {
                println!("{port}");
            }
            port
        })
        .collect::<Vec<i32>>();
}

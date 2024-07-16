use reqwest::blocking::Client;
use std::io::Read;
use std::time::Instant;

fn main() {
    let client = Client::new();
    let download_url = "https://storage.googleapis.com/google-code-archive-downloads/v2/code.google.com/jquery-speedtest/10MB.txt";

    println!("Downloading a: 10 MB testfile");
    let download_speed = perform_download_test(&client, download_url);

    println!("Download speed: {:.2} Mbps", download_speed);
}

fn perform_download_test(client: &Client, url: &str) -> f64 {
    let start = Instant::now();
    let mut response = client.get(url).send().unwrap();
    let mut total_bytes = 0;

    let mut buffer = [0; 8192];
    while let Ok(bytes_read) = response.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        total_bytes += bytes_read;
    }

    let duration = start.elapsed().as_secs_f64();
    let speed = (total_bytes as f64 * 8.0) / (duration * 1_000_000.0); // Mbps
    speed
}

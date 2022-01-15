extern crate argparse;
extern crate reqwest;
extern crate url;

use argparse::{ArgumentParser, Store};
use reqwest::blocking::Client;
use std::fs::File;
use url::Url;
use std::io;

fn fetch(url: String, output: String) {
    let parsed_url = Url::parse(&url).expect("Invalid url");
    let mut target: String = output.clone();
    if target == "None" {
        target = parsed_url.path().split("/").last().unwrap().to_string();
    }
    println!("Download in progress \"{}\"", target);
    let client = Client::new();
    let mut resp = client.get(url).send().unwrap();
    if resp.status().is_success() {
        let out = File::create(target);
        match out {
            Ok(mut f) => {
                io::copy(&mut resp, &mut f).unwrap();
                println!("Downloaded successfully")
            },
            Err(e) => println!("Error: {}", e)
        }
    } else {
        println!("Error while downloading (code: {})", resp.status());
    }
}

fn main() {
    let mut url: String = "None".to_string();
    let mut output: String = "None".to_string();

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Functionality similar to \"wget\" in Linux");
        parser.refer(&mut url).add_argument("url", Store, "URL of the file to download");
        parser.refer(&mut output).add_option(&["-o", "--output"], Store, "Location of downloaded file");
        parser.parse_args_or_exit();
    }

    fetch(url, output);

}

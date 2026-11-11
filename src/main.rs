use clap::Parser;
use reqwest::blocking;

pub mod find;

const BLOCKED_ID: &[&str] = &[
    "This domain has been blocked",
    "Web Page Blcked",
    "The web page you are trying to visit has been blocked in accordance with school policy. Please contact your system administrator if you believe this is an error.",
    "Content Blocked",
    "Certificate Error",
    "This has been blocked by IT.",
];
// these are taken from https://github.com/ZorrentSystems/Website-Block-Checker

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// path
    path: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let path = args.path.unwrap();
    let (links, success) = find::read_f(&path);
    let mut blocked = vec![];
    let mut ublocked = vec![];

    if !success {
        println!("error: {links}");
        return; 
    }

    for url in links.lines() {
        if url.trim().is_empty() {
            continue
        }
        if check(&url) {
            blocked.push(url);
        } else {

            ublocked.push(url);
        }
    }
    
    println!("not blocked:");
    let mut list = String::from("");
    for url in ublocked {
        list.push_str(&format!("\n{url}"));
    }
    println!("{list}");

}


fn check(urls: &str) -> bool {
    let url = urls.trim();
    if !urls.is_empty() {
        match blocking::get(url) {
            Ok(fb) => {
                let body = fb.text().unwrap_or_default();
                for id in BLOCKED_ID {
                    if body.contains(id) {
                        return true;
                    }
                }
            }
            Err(_) => {
                return true;
            }
        }
    }
    return false;
}

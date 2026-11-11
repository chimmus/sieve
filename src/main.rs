use clap::Parser;
use std::{fs, time::Duration};
use reqwest::blocking::Client;

const BLOCKED_ID: &[&str] = &[
    "This domain has been blocked",
    "Web Page Blcked",
    "The web page you are trying to visit has been blocked in accordance with school policy. Please contact your system administrator if you believe this is an error.",
    "Content Blocked",
    "Certificate Error",
    "This has been blocked by IT.",
];
// these are taken from https://github.com/ZorrentSystems/Website-Block-Checker

fn main() {}

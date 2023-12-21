/*
     SPDX-FileCopyrightText: 2023 Kurt Hindenburg <kurt.hindenburg@gmail.com>

     SPDX-License-Identifier: MIT
*/

// Find latest "pfSense Plus Software" version from their website and
//  compare to installed version (text file)

// <li class="toctree-l4"><a class="reference internal" href="2-5-2.html">2.5.2</a></li>

use std::fs;

fn get_website_version() -> String {
    let response = reqwest::blocking::get("https://docs.netgate.com/pfsense/en/latest/releases/index.html");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let version_selector = scraper::Selector::parse("li.toctree-l4>a").unwrap();
    let mut vec = Vec::new();
    document.select(&version_selector).for_each(|x| vec.push(x.inner_html()));
    vec.sort();
    vec.reverse();
    //println!("{}", vec.first().unwrap());

    return vec.first().unwrap().to_string();
}

fn get_installed_version() -> String {
    let file_path = "/Users/kurthindenburg/.pfsense.version.txt";

    //println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("unable to read file");

    return contents.trim().to_string();
}

fn main() {
    let web_version = get_website_version();
    //let web_version = String::from("23.09.01");
    //println!("{}", web_version);

    let installed_version = get_installed_version();
    //println!("{}", installed_version);

    if web_version > installed_version {
        println!("UPDATE NOW current version {}; installed version {}", web_version, installed_version);
    } else {
        println!("No need to update current version {}; installed version {}", web_version, installed_version);

    }
}

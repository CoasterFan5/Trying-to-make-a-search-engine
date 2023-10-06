use std::{fs, error::Error};

fn main() {
    println!("Hello, world!");

    let weblist = fs::read_to_string("./weblist.txt");
    if weblist.is_err() {
        
        let error = weblist.unwrap_err();
        println!("{error}");
        return;
    }

    let web_list_string = weblist.unwrap();
    

    let web_sites: Vec<&str> = web_list_string.split("\n").collect();
    println!("{web_sites:?}");

    for site in web_sites.iter()  {
        println!("Site: {site}");
    }
}

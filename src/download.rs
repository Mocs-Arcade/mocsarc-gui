use std::{fs::{File, self}, io::{Write, Cursor}, path::{Path, PathBuf}};

use curl::easy::{Easy2, List, Handler, WriteError, Easy};
struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn download () {

    let mut versions_json = json::parse(&fs::read_to_string("releases/releases.json").unwrap()).unwrap();

    let mut user_agent_header = List::new();
    user_agent_header.append("User-Agent: Mocs-Arcade").unwrap();

    let mut org_curl = Easy2::new(Collector(Vec::new()));
    org_curl.get(true).unwrap();
    org_curl.url("https://api.github.com/orgs/Mocs-Arcade/repos").unwrap();
    org_curl.http_headers(user_agent_header).unwrap();
    org_curl.perform().unwrap();

    let org_contents = org_curl.get_ref();
    let org_data : String = String::from_utf8_lossy(&org_contents.0).to_string();
    let org_json = json::parse(&org_data).unwrap();

    for x in 0..org_json.len() {
        if !org_json[x]["name"].eq(".github") && !org_json[x]["name"].eq("mocsarc-gui") {
            let s = org_json[x]["releases_url"].to_string().replace("{/id}", "/latest");

            let mut user_agent_header = List::new();
            user_agent_header.append("User-Agent: Mocs-Arcade").unwrap();

            let mut release_curl = Easy2::new(Collector(Vec::new()));
            release_curl.get(true).unwrap();
            release_curl.url(&s).unwrap();
            release_curl.http_headers(user_agent_header).unwrap();
            release_curl.perform().unwrap();

            let release_contents = release_curl.get_ref();
            let release_data : String = String::from_utf8_lossy(&release_contents.0).to_string();
            let release_json = json::parse(&release_data).unwrap();

            let repo_name = format!("{}",org_json[x]["name"]);
            let filepath = format!("{}.zip",&repo_name);

            if !versions_json[&repo_name].eq(&release_json["tag_name"]) || !Path::new(&format!("releases/{}",filepath)).exists() {
                let mut dst = Vec::new();
                let mut download_curl = Easy::new();
                download_curl.url(&release_json["assets"][0]["browser_download_url"].to_string()).unwrap();
                let _redirect = download_curl.follow_location(true);

                if Path::new(&format!("releases/{}",filepath)).exists() {
                    println!("Removing {} in releases", filepath);
                    fs::remove_file(format!("releases/{}",filepath)).unwrap();
                }

                {
                    let mut transfer = download_curl.transfer();
                    transfer.write_function(|data| {
                        dst.extend_from_slice(data);
                        Ok(data.len())
                    }).unwrap();
                    transfer.perform().unwrap();
                }
                {
                    println!("Downloaded {}",filepath);
                    let mut file = File::create(format!("releases/{}",filepath)).unwrap();
                    file.write_all(dst.as_slice()).unwrap();
                }

                let zip = fs::read(format!("releases/{}",filepath)).unwrap();

                if PathBuf::from(format!("games/{}",&repo_name)).exists() {
                    fs::remove_dir_all(PathBuf::from(format!("games/{}",&repo_name))).unwrap();
                }
                fs::create_dir(format!("games/{}",&repo_name)).unwrap();

                zip_extract::extract(Cursor::new(zip), &PathBuf::from(format!("games/{}",&repo_name)), true).unwrap();

                versions_json.remove(&repo_name);
                versions_json.insert(&repo_name, format!("{}",&release_json["tag_name"])).unwrap();
            } else {
                println!("Latest release is already downloaded for {} {}", repo_name, &release_json["tag_name"])
            }
        }
    }

    let mut new_json = File::create("releases/releases.json").unwrap();
    new_json.write_all(json::stringify(versions_json).as_bytes()).unwrap();

}
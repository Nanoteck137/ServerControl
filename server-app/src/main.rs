#![allow(dead_code)]

use sysinfo::SystemExt;
use sysinfo::System;
use serde::{ Deserialize };

mod system_info;

#[derive(Debug)]
enum Error {
    NoObjectWithId{ status_code: reqwest::StatusCode, id: String },
    FailedToUpdate(reqwest::StatusCode),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Debug)]
struct ApiCreatedResponse {
    #[serde(rename = "objectId")]
    object_id: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

struct ServerObject {
    object_id: String,
}

impl ServerObject {
    fn update_data(&mut self, data: String) -> Result<()> {
        // TODO(patrik): Should this client be cached?
        let client = reqwest::blocking::Client::new();

        let id = &self.object_id;

        // TODO(patrik): This URL should change to enviroment variable
        let url =
            format!("http://localhost:3000/parse/classes/clients/{}", id);
        // TODO(patrik): Map this error (remove unwrap)
        // TODO(patrik): X-Parse-Application-Id should be env variable
        let res = client.put(url)
            .header("X-Parse-Application-Id", "servercontrol")
            .header("Content-Type", "application/json")
            .body(data)
            .send()
            .unwrap();

        if res.status() != reqwest::StatusCode::OK {
            return Err(Error::FailedToUpdate(res.status()));
        }

        Ok(())
    }
}

fn obtain_object() -> Result<ServerObject> {
    let id = std::fs::read_to_string("id.txt").unwrap();

    let client = reqwest::blocking::Client::new();

    // TODO(patrik): This URL should change to enviroment variable
    let url = format!("http://localhost:3000/parse/classes/clients/{}", id);
    // TODO(patrik): Map this error (remove unwrap)
    let res = client.get(url)
        .header("X-Parse-Application-Id", "servercontrol")
        .send()
        .unwrap();
    if res.status() != reqwest::StatusCode::OK {
        return Err(Error::NoObjectWithId {
            status_code: res.status(),
            id: id
        });
    }

    Ok(ServerObject {
        object_id: id
    })
}

fn main() {
    let mut obj = obtain_object().unwrap();

    loop {
        println!("Update");
        let system_info = system_info::get_system_info();
        let data = serde_json::to_string(&system_info).unwrap();

        obj.update_data(data).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    /*
    let client = reqwest::blocking::Client::new();

    let res = client.post("http://localhost:3000/parse/classes/clients")
        .header("X-Parse-Application-Id", "servercontrol")
        .header("Content-Type", "application/json")
        .body(s)
        .send()
        .unwrap();

    let res = res.text().unwrap();
    let res = serde_json::from_str::<ApiCreatedResponse>(&res);
    println!("Res: {:?}", res);
    */
}

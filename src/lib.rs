// To get a token, you need to log in to the 
// converter website and go to your profile profile.
// Link -> https://conversiontools.io/profile
// Author: WinsomeQuill (https://github.com/WinsomeQuill)
// GitHub: https://github.com/WinsomeQuill/ConversionTools-Rust

pub mod api {
    use std::{io, collections::HashMap, fs::File};
    use reqwest;
    use reqwest::blocking::{Client, multipart};
    use serde_json::{Value, json};

    pub fn get_tasks(url: &str, token: &str) -> String {
        let furl = format!("{}/tasks", &url);
        let client = Client::new();

        let resp = client
            .get(&furl)
            .bearer_auth(&token)
            .header("Content-Type", "application/json")
            .header("User-Agent", "conversion_tools_rust_reqwest")
            .send()
            .unwrap();

        resp.text().unwrap()
    }

    pub fn get_task(url: &str, token: &str, task_id: &str) -> String {
        let furl = format!("{}/tasks/{}", &url, &task_id);
        let client = Client::new();

        let resp = client
            .get(&furl)
            .bearer_auth(&token)
            .header("Content-Type", "application/json")
            .header("User-Agent", "conversion_tools_rust_reqwest")
            .send()
            .unwrap();

        resp.text().unwrap()
    }

    pub fn upload_file(url: &str, token: &str, path: &str) -> String {
        let furl = format!("{}/files", &url);
        let form = multipart::Form::new()
            .file("file=@", &path).unwrap();

            
        let client = Client::new();
        let res = client
            .post(&furl)
            .bearer_auth(&token)
            .header("Content-Type", "multipart/form-data")
            .header("User-Agent", "conversion_tools_rust_reqwest")
            .multipart(form)
            .send()
            .unwrap();

        res.text().unwrap()
    }

    pub fn create_task(url: &str, token: &str, type_conv: &str, file_id: &str, args: &HashMap<&str, &str>) -> String {
        let mut json_task: serde_json::Value = json!({});

        json_task["type"] = Value::String(type_conv.to_string());

        if &file_id.len() != &0 {
            json_task["file_id"] = Value::String(file_id.to_string());
        }

        if args.len() != 0 {
            for i in args.iter() {
                json_task["options"][i.0] = Value::String(i.1.to_string());
            }
        }

        let furl = format!("{}/tasks", &url);
        let client = Client::new();

        let resp = client
            .post(&furl)
            .bearer_auth(&token)
            .header("Content-Type", "application/json")
            .header("User-Agent", "conversion_tools_rust_reqwest")
            .json(&json_task)
            .send()
            .unwrap();

        resp.text().unwrap()
    }

    pub fn download_file(url: &str, token: &str, file_id: &str, output_path: &str) {
        let furl = format!("{}/files/{}", &url, &file_id);
        let client = Client::new();

        let mut resp = client
            .get(&furl)
            .bearer_auth(&token)
            .header("Content-Type", "application/json")
            .header("User-Agent", "conversion_tools_rust_reqwest")
            .send()
            .unwrap();

        let mut out = File::create(&output_path).unwrap();
        io::copy(&mut resp, &mut out).unwrap();
    }
}

// To get a token, you need to log in to the 
// converter website and go to your profile profile.
// Link -> https://conversiontools.io/profile
// Author: WinsomeQuill (https://github.com/WinsomeQuill)
// GitHub: https://github.com/WinsomeQuill/ConversionTools-Rust

pub mod api {
    use std::{io, collections::HashMap, fs::File};
    use reqwest;
    use reqwest::blocking::multipart::Form;
    use reqwest::blocking::{Client, multipart, Response};
    use serde_json::{Value, from_str, json};

    pub fn get_tasks(url: &str, token: &str) -> Value {
        let furl: String = format!("{}/tasks", &url);
        let client: Client = Client::new();

        let resp: Value = match client 
            .get(&furl)
            .bearer_auth(&token)
            .header("Content-Type", "application/json")
            .header("User-Agent", "conversion_tools_rust_reqwest")
            .send() {
                Ok(response) => from_str(&response.text().unwrap()).unwrap(),
                Err(_) => json!({}),
            };
        resp
    }

    pub fn get_task(url: &str, token: &str, task_id: &str) -> Value {
        let furl: String = format!("{}/tasks/{}", &url, &task_id);
        let client: Client = Client::new();

        let resp: Value = match client
            .get(&furl)
            .bearer_auth(&token)
            .header("Content-Type", "application/json")
            .header("User-Agent", "conversion_tools_rust_reqwest")
            .send() {
                Ok(response) => from_str(&response.text().unwrap()).unwrap(),
                Err(_) => json!({}),
            };
        resp
    }

    pub fn upload_file(url: &str, token: &str, path: &str) -> Value {
        let furl: String = format!("{}/files", &url);
        let form: Form = multipart::Form::new()
            .file("file=@", &path).unwrap();

            
        let client: Client = Client::new();
        let res: Value = match client
            .post(&furl)
            .bearer_auth(&token)
            .header("Content-Type", "multipart/form-data")
            .header("User-Agent", "conversion_tools_rust_reqwest")
            .multipart(form)
            .send() {
                Ok(response) => from_str(&response.text().unwrap()).unwrap(),
                Err(_) => json!({}),
            };
        res
    }

    pub fn create_task(url: &str, token: &str, type_conv: &str, file_id: &str, args: &HashMap<&str, &str>) -> Value {
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

        let furl: String = format!("{}/tasks", &url);
        let client: Client = Client::new();

        let resp: Value = match client
            .post(&furl)
            .bearer_auth(&token)
            .header("Content-Type", "application/json")
            .header("User-Agent", "conversion_tools_rust_reqwest")
            .json(&json_task)
            .send() {
                Ok(response) => from_str(&response.text().unwrap()).unwrap(),
                Err(_) => json!({}),
            };
        resp
    }

    pub fn download_file(url: &str, token: &str, file_id: &str, output_path: &str) {
        let furl: String = format!("{}/files/{}", &url, &file_id);
        let client: Client = Client::new();

        let mut resp: Response = client
            .get(&furl)
            .bearer_auth(&token)
            .header("Content-Type", "application/json")
            .header("User-Agent", "conversion_tools_rust_reqwest")
            .send()
            .unwrap();

        let mut out: File = File::create(&output_path).unwrap();
        io::copy(&mut resp, &mut out).unwrap();
    }
}

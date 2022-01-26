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

    impl Api {
        pub fn new(token: String, url: String) -> Api {
            Api { token: token, url: url }
        }

        pub fn get_tasks(&self) -> Value {
            let url: String = format!("{}/tasks", &self.url);
            let client: Client = Client::new();
    
            let resp: Value = match client 
                .get(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .send() {
                    Ok(response) => from_str(&response.text().unwrap()).unwrap(),
                    Err(_) => json!({}),
                };
            resp
        }

        pub fn get_task(&self, task_id: &str) -> Value {
            let url: String = format!("{}/tasks/{}", &self.url, &task_id);
            let client: Client = Client::new();
    
            let resp: Value = match client
                .get(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .send() {
                    Ok(response) => from_str(&response.text().unwrap()).unwrap(),
                    Err(_) => json!({}),
                };
            resp
        }

        pub fn upload_file(&self, path: &str) -> Value {
            let url: String = format!("{}/files", &self.url);
            let form: Form = multipart::Form::new()
                .file("file=@", &path).unwrap();
    
                
            let client: Client = Client::new();
            let res: Value = match client
                .post(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "multipart/form-data")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .multipart(form)
                .send() {
                    Ok(response) => from_str(&response.text().unwrap()).unwrap(),
                    Err(_) => json!({}),
                };
            res
        }

        pub fn create_task(&self, type_conv: &str, file_id: &str, args: &HashMap<&str, &str>) -> Value {
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
    
            let url: String = format!("{}/tasks", &self.url);
            let client: Client = Client::new();
    
            let resp: Value = match client
                .post(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .json(&json_task)
                .send() {
                    Ok(response) => from_str(&response.text().unwrap()).unwrap(),
                    Err(_) => json!({}),
                };
            resp
        }

        pub fn download_file(&self, file_id: &str, output_path: &str) {
            let url: String = format!("{}/files/{}", &self.url, &file_id);
            let client: Client = Client::new();
    
            let mut resp: Response = client
                .get(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .send()
                .unwrap();
    
            let mut out: File = File::create(&output_path).unwrap();
            io::copy(&mut resp, &mut out).unwrap();
        }
    }

    pub struct Api {
        token: String,
        url: String
    }
}

// To get a token, you need to log in to the 
// converter website and go to your profile profile.
// Link -> https://conversiontools.io/profile
// Author: WinsomeQuill (https://github.com/WinsomeQuill)
// GitHub: https://github.com/WinsomeQuill/ConversionTools-Rust

pub mod api {
    use std::{collections::HashMap, fs::File};
    use std::io::Write;
    use reqwest;
    use reqwest::{
        Error,
        Client,
        multipart::Form
    };
    use reqwest::multipart::Part;
    use serde_json::{Value, json};
    use crate::models::models::api::*;

    pub struct Api {
        token: String,
        url: String
    }

    impl Api {
        pub async fn new(token: String, url: String) -> Api {
            Api { token, url }
        }

        pub async fn get_tasks(&self) -> Result<ResultApi<Tasks>, Error> {
            let url = format!("{}/tasks", &self.url);
            let client = Client::new();
            let resp = client
                .get(url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .send()
                .await?;

            let status_code = resp.status().as_u16();
            let json = resp.text().await?;

            let result = match serde_json::from_str::<Tasks>(&json) {
                Ok(o) => ResultApi { status_code, json, result: Some(o) },
                Err(_) => ResultApi { status_code, json, result: None },
            };

            Ok(result)
        }

        pub async fn get_task(&self, task_id: &str) -> Result<ResultApi<Task>, Error> {
            let url: String = format!("{}/tasks/{}", &self.url, &task_id);
            let client: Client = Client::new();
            let resp = client
                .get(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .send()
                .await?;

            let status_code = resp.status().as_u16();
            let json = resp.text().await?;

            let result = match serde_json::from_str::<Task>(&json) {
                Ok(o) => ResultApi { status_code, json, result: Some(o) },
                Err(_) => ResultApi { status_code, json, result: None }
            };

            Ok(result)
        }

        pub async fn upload_file(&self, path: &str, file_name: String) -> Result<ResultApi<UploadFile>, Error> {
            let url: String = format!("{}/files", &self.url);

            let file = std::fs::read(path).unwrap();
            let file_part = Part::bytes(file)
                .file_name(file_name);
            let form = Form::new()
                .part("file=@", file_part);
                
            let client: Client = Client::new();
            let resp = client
                .post(&url)
                .bearer_auth(&self.token)
                //.header("Content-Type", "multipart/form-data")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .multipart(form)
                .send()
                .await?;

            let status_code = resp.status().as_u16();
            let json = resp.text().await?;
            let result = match serde_json::from_str::<UploadFile>(&json) {
                Ok(o) => ResultApi { status_code, json, result: Some(o) },
                Err(_) => ResultApi { status_code, json, result: None }
            };

            Ok(result)
        }

        pub async fn create_task(&self, type_conv: &str, file_id: &str, args: &HashMap<&str, &str>) -> Result<ResultApi<CreateTask>, Error> {
            let mut json_task = json!({
                "type": type_conv
            });
    
            if !file_id.is_empty() {
                json_task["file_id"] = Value::String(file_id.to_string());
            }
    
            if !args.is_empty() {
                for (k, v) in args.iter() {
                    json_task["options"][k] = Value::String(v.to_string());
                }
            }
    
            let url: String = format!("{}/tasks", &self.url);
            let client: Client = Client::new();
    
            let resp = client
                .post(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .json(&json_task)
                .send()
                .await?;

            let status_code = resp.status().as_u16();
            let json = resp.text().await?;

            let result = match serde_json::from_str::<CreateTask>(&json) {
                Ok(o) => ResultApi { status_code, json, result: Some(o) },
                Err(_) => ResultApi { status_code, json, result: None }
            };

            Ok(result)
        }

        pub async fn download_file(&self, file_id: &str, output_path: &str) -> Result<(), std::io::Error> {
            let url: String = format!("{}/files/{}", &self.url, &file_id);
            let client: Client = Client::new();
    
            let resp = client
                .get(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .send()
                .await
                .expect("Failed request!")
                .bytes()
                .await
                .expect("Failed request convert to bytes!")
                .to_vec();

            File::create(output_path)?.write_all(&resp)
        }
    }
}

pub mod models;

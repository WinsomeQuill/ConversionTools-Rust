// To get a token, you need to log in to the 
// converter website and go to your profile profile.
// Link -> https://conversiontools.io/profile
// Author: WinsomeQuill (https://github.com/WinsomeQuill)
// GitHub: https://github.com/WinsomeQuill/ConversionTools-Rust

pub mod api {
    use std::{collections::HashMap, fs::File};
    use reqwest;
    use reqwest::Error;
    use reqwest::blocking::{Client, multipart, Response, multipart::Form};
    use serde_json::{Value, json};
    pub mod models;
    use models::models::{
        tasks::*, upload_file::*, 
        task::*, create_task::*
    };
    
    #[derive(Debug)]
    pub struct Api {
        token: String,
        url: String
    }

    impl Api {
        pub fn new(token: String, url: String) -> Api {
            Api { token, url }
        }

        pub fn get_tasks(&self) -> Result<TasksResult, Error> {
            let url: String = format!("{}/tasks", &self.url);
            let client: Client = Client::new();
    
            let resp = match client 
                .get(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .send() {
                    Ok(response) => {
                        let status_code = response.status().as_u16();
                        let json = response.text().unwrap();
                        let result = match serde_json::from_str::<Tasks>(&json) {
                            Ok(o) => TasksResult { status_code, json, result: Some(o) },
                            Err(_) => TasksResult { status_code, json, result: None },
                        };
                        Ok(result)
                    },
                    Err(e) => Err(e),
                };
            resp
        }

        pub fn get_task(&self, task_id: &str) -> Result<TaskResult, Error> {
            let url: String = format!("{}/tasks/{}", &self.url, &task_id);
            let client: Client = Client::new();
    
            let resp: Result<TaskResult, Error> = match client
                .get(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .send() {
                    Ok(response) => {
                        let status_code = response.status().as_u16();
                        let json = response.text().unwrap();
                        let result: TaskResult = match serde_json::from_str::<Task>(&json) {
                            Ok(o) => TaskResult { status_code, json, result: Some(o) },
                            Err(_) => TaskResult { status_code, json, result: None }
                        };
                        Ok(result)
                    },
                    Err(e) => Err(e),
                };
            resp
        }

        pub fn upload_file(&self, path: &str) -> Result<UploadFileResult, Error> {
            let url: String = format!("{}/files", &self.url);
            let form: Form = multipart::Form::new()
                .file("file=@", &path).unwrap();
                
            let client: Client = Client::new();
            let res: Result<UploadFileResult, Error> = match client
                .post(&url)
                .bearer_auth(&self.token)
                //.header("Content-Type", "multipart/form-data")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .multipart(form)
                .send() {
                    Ok(response) => {
                        let status_code = response.status().as_u16();
                        let json = response.text().unwrap();
                        let result: UploadFileResult = match serde_json::from_str::<UploadFile>(&json) {
                            Ok(o) => UploadFileResult { status_code, json, result: Some(o) },
                            Err(_) => UploadFileResult { status_code, json, result: None }
                        };
                        Ok(result)
                    },
                    Err(e) => Err(e),
                };
            res
        }

        pub fn create_task(&self, type_conv: &str, file_id: &str, args: &HashMap<&str, &str>) -> Result<CreateTaskResult, Error> {
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
    
            let resp: Result<CreateTaskResult, Error> = match client
                .post(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .json(&json_task)
                .send() {
                    Ok(response) => {
                        let status_code = response.status().as_u16();
                        let json = response.text().unwrap();
                        let result: CreateTaskResult = match serde_json::from_str::<CreateTask>(&json) {
                            Ok(o) => CreateTaskResult { status_code, json, result: Some(o) },
                            Err(_) => CreateTaskResult { status_code, json, result: None }
                        };
                        Ok(result)
                    },
                    Err(e) => Err(e),
                };
            resp
        }

        pub fn download_file(&self, file_id: &str, output_path: &str) -> Result<(), std::io::Error> {
            let url: String = format!("{}/files/{}", &self.url, &file_id);
            let client: Client = Client::new();
    
            let resp: Result<Response, Error> = match client
                .get(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .send() {
                    Ok(response) => Ok(response),
                    Err(e) => Err(e),
                };

            let mut out: File = File::create(&output_path).unwrap();
            match std::io::copy(&mut resp.unwrap(), &mut out) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
    }
}

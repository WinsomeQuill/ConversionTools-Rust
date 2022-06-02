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

        pub fn get_tasks(&self) -> Result<Result<Tasks, Box<String>>, Error> {
            let url: String = format!("{}/tasks", &self.url);
            let client: Client = Client::new();
    
            let resp: Result<Result<Tasks, Box<String>>, Error> = match client 
                .get(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .send() {
                    Ok(response) => {
                        let t = Box::new(response.text().unwrap());
                        let result: Result<Tasks, Box<String>> = match serde_json::from_str::<Tasks>(&t) {
                            Ok(o) => Ok(o),
                            Err(_) => Err(t),
                        };
                        Ok(result)
                    },
                    Err(e) => Err(e),
                };
            resp
        }

        pub fn get_task(&self, task_id: &str) -> Result<Result<Task, Box<String>>, Error> {
            let url: String = format!("{}/tasks/{}", &self.url, &task_id);
            let client: Client = Client::new();
    
            let resp: Result<Result<Task, Box<String>>, Error> = match client
                .get(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .send() {
                    Ok(response) => {
                        let t = Box::new(response.text().unwrap());
                        let result: Result<Task, Box<String>> = match serde_json::from_str::<Task>(&t) {
                            Ok(o) => Ok(o),
                            Err(_) => Err(t)
                        };
                        Ok(result)
                    },
                    Err(e) => Err(e),
                };
            resp
        }

        pub fn upload_file(&self, path: &str) -> Result<Result<UploadFile, Box<String>>, Error> {
            let url: String = format!("{}/files", &self.url);
            let form: Form = multipart::Form::new()
                .file("file=@", &path).unwrap();
                
            let client: Client = Client::new();
            let res: Result<Result<UploadFile, Box<String>>, Error> = match client
                .post(&url)
                .bearer_auth(&self.token)
                //.header("Content-Type", "multipart/form-data")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .multipart(form)
                .send() {
                    Ok(response) => {
                        let t = Box::new(response.text().unwrap());
                        let result: Result<UploadFile, Box<String>> = match serde_json::from_str::<UploadFile>(&t) {
                            Ok(o) => Ok(o),
                            Err(_) => Err(t)
                        };
                        Ok(result)
                    },
                    Err(e) => Err(e),
                };
            res
        }

        pub fn create_task(&self, type_conv: &str, file_id: &str, args: &HashMap<&str, &str>) -> Result<Result<CreateTask, Box<String>>, Error> {
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
    
            let resp: Result<Result<CreateTask, Box<String>>, Error> = match client
                .post(&url)
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("User-Agent", "conversion_tools_rust_reqwest")
                .json(&json_task)
                .send() {
                    Ok(response) => {
                        let t = Box::new(response.text().unwrap());
                        let result: Result<CreateTask, Box<String>> = match serde_json::from_str::<CreateTask>(&t) {
                            Ok(o) => Ok(o),
                            Err(_) => Err(t),
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

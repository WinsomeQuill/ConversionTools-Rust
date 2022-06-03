# ConversionTools Rust
This Conversion Tools API Rust client allows you to use the site API and convert files faster and more conveniently.

Site [**Conversion Tools**](https://conversiontools.io/) allows you to convert files from one format to another.

To use the library, you will need a **API-Token**, which you can find in [**your profile**](https://conversiontools.io/profile).

# How to use
You can find examples of using all functions in the [**ConversionTools-Rust-Examples**](https://github.com/WinsomeQuill/ConversionTools-Rust-Examples/blob/main/src/main.rs) repository 

Get All Tasks
---
```Rust
use conversion_tools_api::api::Api;
use conversion_tools_api::api::models::models::tasks::TasksResult;

let url: &str = "api url";
let object: Api = Api::new(String::from("Your Token"), &url);
let tasks: Result<TasksResult, Error> = object.get_tasks();
let variable = result.unwrap().result.unwrap(); // get result struct
println!("{}", result.unwrap().json); // print json
println!("{}", result.unwrap().status_code); //print http code
```

Upload File
---
```Rust
use conversion_tools_api::api::Api;
use conversion_tools_api::api::models::models::upload_file::UploadFileResult;

let url: &str = "api url";
let object: Api = Api::new(String::from("Your Token"), &url);
let result: Result<UploadFileResult, Error> = object.upload_file(&"path");
let variable = result.unwrap().result.unwrap(); // get result struct
println!("{}", result.unwrap().json); // print json
println!("{}", result.unwrap().status_code); //print http code
```

Create task (start converting)
---
```Rust
use std::collections::HashMap;
use conversion_tools_api::api::Api;
use conversion_tools_api::api::models::models::create_task::CreateTaskResult;

let mut args: HashMap<&str, &str> = HashMap::new();
args.insert("orientation", "Portrait");

let type_convert: &str = "convert.jpg_to_pdf";

let url: &str = "api url";
let object: Api = Api::new(String::from("Your Token"), &url);
let result: Result<CreateTaskResult, Error> = object.create_task(&type_convert, &"file_id", &args);
let variable = result.unwrap().result.unwrap(); // get result struct
println!("{}", result.unwrap().json); // print json
println!("{}", result.unwrap().status_code); //print http code
```

Get Task
---
```Rust
use conversion_tools_api::api::Api;
use conversion_tools_api::api::models::models::task::TaskResult;

let url: &str = "api url";
let object: Api = Api::new(String::from("Your Token"), &url);
let result: Result<TaskResult, Error> = object.get_task(&"task_id");
let variable = result.unwrap().result.unwrap(); // get result struct
println!("{}", result.unwrap().json); // print json
println!("{}", result.unwrap().status_code); //print http code
```

Download File
---
```Rust
use conversion_tools_api::api::Api;

let object: Api = Api::new(String::from("Your Token"), String::from("Url"));
let result: Result<(), Error> = object.download_file(&"file_id", "output_path");
```

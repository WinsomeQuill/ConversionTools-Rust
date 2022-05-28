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
use conversion_tools_api::api::models::models::tasks::Tasks;

let url: &str = "api url";
let object: Api = Api::new(String::from("Your Token"), &url);
let tasks: Result<Tasks, Error> = object.get_tasks();
println!("{:?}", tasks.unwrap()); //print struct
```

Upload File
---
```Rust
use conversion_tools_api::api::Api;
use conversion_tools_api::api::models::models::upload_file::UploadFile;

let url: &str = "api url";
let object: Api = Api::new(String::from("Your Token"), &url);
let result: Result<UploadFile, Error> = object.upload_file(&"path");
println!("{:?}", result.unwrap()); //print struct
```

Create task (start converting)
---
```Rust
use std::collections::HashMap;
use conversion_tools_api::api::Api;
use conversion_tools_api::api::models::models::create_task::CreateTask;

let mut args: HashMap<&str, &str> = HashMap::new();
args.insert("orientation", "Portrait");

let type_convert: &str = "convert.jpg_to_pdf";

let url: &str = "api url";
let object: Api = Api::new(String::from("Your Token"), &url);
let result: Result<CreateTask, Error> = object.create_task(&type_convert, &"file_id", &args);
println!("{:?}", result.unwrap()); //print struct
```

Get Task
---
```Rust
use conversion_tools_api::api::Api;
use conversion_tools_api::api::models::models::task::Task;

let url: &str = "api url";
let object: Api = Api::new(String::from("Your Token"), &url);
let result: Result<Task, Error> = object.get_task(&"task_id");
println!("{:?}", result.unwrap()); //print struct
```

Download File
---
```Rust
use conversion_tools_api::api::Api;

let object: Api = Api::new(String::from("Your Token"), String::from("Url"));
let result: Result<(), Error> = object.download_file(&"file_id", "output_path");
```

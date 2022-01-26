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

let object: Api = Api::new(String::from("Your Token"), String::from("Url"));
let tasks: Value = object.get_tasks();
println!("{}", tasks); //print json response
```

Upload File
---
```Rust
use conversion_tools_api::api::Api;

let object: Api = Api::new(String::from("Your Token"), String::from("Url"));
let result: Value = object.upload_file(&"path");
println!("{}", result); //print json response
```

Create task (start converting)
---
```Rust
use std::collections::HashMap;
use conversion_tools_api::api::Api;

let mut args: HashMap<&str, &str> = HashMap::new();
args.insert("orientation", "Portrait");

let type_convert: &str = "convert.jpg_to_pdf";
let object: Api = Api::new(String::from("Your Token"), String::from("Url"));
let result: Value = object.create_task(&type_convert, &"file_id", &args);
println!("{}", result); //print json response
```

Get Task
---
```Rust
use conversion_tools_api::api::Api;

let object: Api = Api::new(String::from("Your Token"), String::from("Url"));
let result: Value = object.get_task(&"task_id");
println!("{}", result); //print json response
```

Download File
---
```Rust
use conversion_tools_api::api::Api;

let object: Api = Api::new(String::from("Your Token"), String::from("Url"));
object.download_file(&"file_id", "output_path");
```

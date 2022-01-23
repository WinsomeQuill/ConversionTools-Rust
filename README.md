# ConversionTools Rust
This Conversion Tools API Rust client allows you to use the site API and convert files faster and more conveniently. 

# How to use
You can find examples of using all functions in the [ConversionTools-Rust-Examples](https://github.com/WinsomeQuill/ConversionTools-Rust-Examples/blob/main/src/main.rs) repository 

Get All Tasks
---
```Rust
use conversion_tools_api::api::get_tasks;

let tasks: String = get_tasks("api_url", "your_token");
println!("{}", tasks); //print json response
```

Upload File
---
```Rust
use conversion_tools_api::api::upload_file;

let result: String = upload_file("api_url", "your_token", &"path");
println!("{}", result); //print json response
```

Create task (start converting)
---
```Rust
use std::collections::HashMap;
use conversion_tools_api::api::create_task;

let mut args: HashMap<&str, &str> = HashMap::new();
args.insert("orientation", "Portrait");

let type_convert: &str = "convert.jpg_to_pdf";
let result: String = create_task("api_url", "your_token", &type_convert, &"file_id", &args);
println!("{}", result); //print json response
```

Get Task
---
```Rust
use conversion_tools_api::api::get_task;

let result: String = get_task("api_url", "your_token", &"task_id");
println!("{}", result); //print json response
```

Download File
---
```Rust
use conversion_tools_api::api::download_file;

download_file("api_url", "your_token", &"file_id", "output_path");
```

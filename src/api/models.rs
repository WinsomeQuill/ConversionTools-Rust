pub mod models {
    pub mod upload_file {
        use serde::{Deserialize, Serialize};

        pub struct UploadFileResult {
            pub status_code: u16,
            pub json: String,
            pub result: Option<UploadFile>
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub struct UploadFile {
            pub error: Option<String>,
            pub file_id: String
        }
    }
    pub mod tasks {
        use serde::{Deserialize, Serialize};

        pub struct TasksResult {
            pub status_code: u16,
            pub json: String,
            pub result: Option<Tasks>
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub struct Tasks {
            pub error: Option<String>,
            pub data: Vec<Data>
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub struct FileSource {
            pub id: String,
            pub name: String,
            pub size: i32,
            pub exists: bool
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub struct  FileResult {
            pub id: String,
            pub name: String,
            pub size: i32,
            pub exists: bool
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub struct Data {
            pub id: String,
            #[serde(alias = "type")]
            pub convertor_type: String,
            pub status: String,
            pub error: Option<String>,
            #[serde(alias = "dateCreated")]
            pub date_create: String,
            #[serde(alias = "dateFinished")]
            pub date_finished: String,
            #[serde(alias = "conversionProgress")]
            pub conversion_progress: u8,
            #[serde(alias = "fileSource")]
            pub file_source: FileSource,
            #[serde(alias = "fileResult")]
            pub file_result: FileResult,
        }
    }
    pub mod task {
        use serde::{Deserialize, Serialize};

        pub struct TaskResult {
            pub status_code: u16,
            pub json: String,
            pub result: Option<Task>
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub struct Task {
            pub error: Option<String>,
            pub status: String,
            pub file_id: Option<String>,
            #[serde(alias = "conversionProgress")]
            pub conversion_progress: u8,
            #[serde(alias = "conversionResult")]
            pub conversion_result: Option<ConversionResult>
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub struct ConversionResult {
            pub message: Option<String>
        }
    }
    pub mod create_task {
        use serde::{Deserialize, Serialize};

        pub struct CreateTaskResult {
            pub status_code: u16,
            pub json: String,
            pub result: Option<CreateTask>
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub struct CreateTask {
            pub error: Option<String>,
            pub task_id: String
        }
    }
}
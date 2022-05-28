pub mod models {
    pub mod upload_file {
        use serde::Deserialize;
        #[derive(Debug, Deserialize)]
        pub struct UploadFile {
            pub error: Option<String>,
            pub file_id: String
        }
    }
    pub mod tasks {
        use serde::Deserialize;
        #[derive(Debug, Deserialize)]
        pub struct Tasks {
            pub error: Option<String>,
            pub data: Vec<Data>
        }

        #[derive(Debug, Deserialize)]
        pub struct FileSource {
            pub id: String,
            pub name: String,
            pub size: i32,
            pub exists: bool
        }

        #[derive(Debug, Deserialize)]
        pub struct  FileResult {
            pub id: String,
            pub name: String,
            pub size: i32,
            pub exists: bool
        }

        #[derive(Debug, Deserialize)]
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
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct Task {
            pub error: Option<String>,
            pub status: String,
            pub file_id: Option<String>,
            #[serde(alias = "conversionProgress")]
            pub conversion_progress: u8,
            #[serde(alias = "conversionResult")]
            pub conversion_result: ConversionResult
        }

        #[derive(Debug, Deserialize)]
        pub struct ConversionResult {
            pub message: Option<String>
        }
    }
    pub mod create_task {
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct CreateTask {
            pub error: Option<String>,
            pub task_id: String
        }
    }
}
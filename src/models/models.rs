pub mod api {
    use serde::{Deserialize, Serialize};

    #[derive(Debug)]
    pub struct ResultApi<T> {
        pub status_code: u16,
        pub json: String,
        pub result: Option<T>
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct UploadFile {
        pub error: Option<String>,
        pub file_id: String
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
        pub r#type: String,
        pub status: String,
        pub error: Option<String>,
        pub url: String,
        #[serde(alias = "dateCreated")]
        pub date_create: String,
        #[serde(alias = "dateFinished")]
        pub date_finished: String,
        #[serde(alias = "conversionProgress")]
        pub conversion_progress: u8,
        #[serde(alias = "fileSource")]
        pub file_source: Option<FileSource>,
        #[serde(alias = "fileResult")]
        pub file_result: FileResult,
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

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CreateTask {
        pub error: Option<String>,
        pub task_id: String
    }
}
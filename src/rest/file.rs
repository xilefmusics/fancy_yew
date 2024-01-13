use gloo::file::File;
use gloo::net::http::Request;
use serde::de::DeserializeOwned;

pub struct FileUploader {}

impl FileUploader {
    pub async fn upload<T: DeserializeOwned>(endpoint: &str, file: &File) -> T {
        let bytes = gloo::file::futures::read_as_bytes(&file).await.unwrap();
        let uint8arr = unsafe { js_sys::Uint8Array::view(&bytes) };
        let js_value = wasm_bindgen::JsValue::from(uint8arr);
        Request::post(endpoint)
            .header("Content-Type", &file.raw_mime_type())
            .body(js_value)
            .unwrap()
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}

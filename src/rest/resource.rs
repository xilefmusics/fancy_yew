use gloo::net::http::Request;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use web_sys::{window, Blob, HtmlAnchorElement, Url};
use yew::{Callback, UseStateHandle};

#[derive(PartialEq)]
pub struct Resource<T: PartialEq> {
    name: String,
    endpoint: String,
    data: Vec<T>,
}

impl<T: PartialEq> Deref for Resource<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Serialize + DeserializeOwned + Default + 'static + Clone + PartialEq> Resource<T> {
    pub fn new(endpoint: String, name: String, data: Vec<T>) -> Self {
        Self {
            endpoint,
            name,
            data,
        }
    }

    pub async fn reload_async(&self) -> Self {
        let data = Request::get(&self.endpoint)
            .send()
            .await
            .unwrap()
            .json::<Vec<T>>()
            .await
            .unwrap();
        Self {
            endpoint: self.endpoint.clone(),
            name: self.name.clone(),
            data,
        }
    }

    pub fn reload(handle: UseStateHandle<Resource<T>>) {
        wasm_bindgen_futures::spawn_local(async move { handle.set(handle.reload_async().await) });
    }

    pub fn reload_closure<I>(handle: UseStateHandle<Resource<T>>) -> impl Fn(I) {
        move |_: I| Self::reload(handle.clone())
    }

    pub fn reload_callback<I: 'static>(handle: UseStateHandle<Resource<T>>) -> Callback<I> {
        Callback::from(Self::add_default_closure(handle))
    }

    pub async fn add_default_async(&self) -> Self {
        Request::post(&self.endpoint)
            .json(&vec![T::default()])
            .unwrap()
            .send()
            .await
            .unwrap();
        self.reload_async().await
    }

    pub fn add_default(handle: UseStateHandle<Resource<T>>) {
        wasm_bindgen_futures::spawn_local(
            async move { handle.set(handle.add_default_async().await) },
        );
    }

    pub fn add_default_closure<I>(handle: UseStateHandle<Resource<T>>) -> impl Fn(I) {
        move |_: I| Self::add_default(handle.clone())
    }

    pub fn add_default_callback<I: 'static>(handle: UseStateHandle<Resource<T>>) -> Callback<I> {
        Callback::from(Self::add_default_closure(handle))
    }

    pub fn export(&self) {
        let data = serde_json::to_string_pretty(&self.data).unwrap();
        let array = js_sys::Array::new();
        array.push(&JsValue::from_str(&data));

        let blob = Blob::new_with_u8_array_sequence_and_options(
            &array,
            web_sys::BlobPropertyBag::new().type_("text/json"),
        )
        .unwrap();
        let url = Url::create_object_url_with_blob(&blob).unwrap();
        let document = window().unwrap().document().unwrap();
        let anchor = document
            .create_element("a")
            .unwrap()
            .dyn_into::<HtmlAnchorElement>()
            .unwrap();
        anchor.set_href(&url);
        anchor.set_download(&format!("{}.json", self.name));
        anchor.click();
        Url::revoke_object_url(&url).unwrap();
    }

    pub fn export_closure<I>(handle: UseStateHandle<Resource<T>>) -> impl Fn(I) {
        move |_: I| (*handle).export()
    }

    pub fn export_callback<I: 'static>(handle: UseStateHandle<Resource<T>>) -> Callback<I> {
        Callback::from(Self::export_closure(handle))
    }

    pub async fn delete_async(&self, r: T) -> Self {
        Request::delete(&self.endpoint)
            .json(&vec![r])
            .unwrap()
            .send()
            .await
            .unwrap();
        self.reload_async().await
    }

    pub fn delete(r: T, handle: UseStateHandle<Resource<T>>) {
        wasm_bindgen_futures::spawn_local(async move { handle.set(handle.delete_async(r).await) });
    }

    pub fn delete_closure<I>(r: T, handle: UseStateHandle<Resource<T>>) -> impl Fn(I) {
        move |_: I| Self::delete(r.clone(), handle.clone())
    }

    pub fn delete_callback<I: 'static>(r: T, handle: UseStateHandle<Resource<T>>) -> Callback<I> {
        Callback::from(Self::delete_closure(r, handle))
    }
}

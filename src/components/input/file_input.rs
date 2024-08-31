use gloo::file::File;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub bind_handle: UseStateHandle<Vec<File>>,
    #[prop_or_default]
    pub callback: Option<Callback<Vec<File>>>,
}

#[function_component(FileInput)]
pub fn file_input(props: &Props) -> Html {
    let onchange = {
        let bind_handle = props.bind_handle.clone();
        let callback = props.callback.clone();
        move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(File::from)
                    .collect::<Vec<File>>();
                if let Some(callback) = callback.clone() {
                    callback.emit(files.clone());
                }
                bind_handle.set(files);
            }
        }
    };

    html! {
        <input
            type="file"
            onchange={onchange}
        />
    }
}

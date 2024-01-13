use super::FileInput;
use gloo::file::File;
use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub bind_handle: UseStateHandle<Vec<String>>,
}

#[function_component]
pub fn RemoteFileInput(props: &Props) -> Html {
    let files_handle = use_state(|| Vec::new());
    let bind_handle = props.bind_handle.clone();

    let add_callback = {
        let list_handle = bind_handle.clone();
        Callback::from(move |files: Vec<File>| {
            let mut list = (*list_handle).clone();
            for file in files {
                list.push(file.name());
            }
            list_handle.set(list);
        })
    };

    let list = (*bind_handle).clone().into_iter()
        .map(|file_name| {
            let ondelete = {
                let list_handle = bind_handle.clone();
                let file_name = file_name.clone();
                move |_: MouseEvent| {
                    let mut list = (*list_handle).clone();
                    list.retain(|item| item != &file_name);
                    list_handle.set(list);
                }
            };
            html! {
                <tr>
                    <td><span class="list-item">{file_name}</span></td>
                    <td><button onclick={ondelete}><span class="material-symbols-outlined icon">{"delete"}</span></button></td>
                </tr>
            }
        })
        .collect::<Html>();

    html! {
        <table class={Style::new(include_str!("remote_file_input.css")).expect("Unwrapping CSS should work!")}>
            {list}
            <tr>
                <FileInput
                    bind_handle={files_handle}
                    callback={add_callback}
                />
            </tr>
        </table>
    }
}

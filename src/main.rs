use fancy_yew::components::{
    input::RemoteFileInput, input::StringNumberMap, Chart as ChartJs, ConfigBuilder,
    Editor as EditorComponent, SyntaxParser,
};
use fancy_yew::layouts::{NavItemBuilder, Navable, VerticalLayout as Layout};

use std::collections::HashMap;

use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(FullScreen)]
fn full_screen() -> Html {
    html! {
        <h1>{"FullScreen"}</h1>
    }
}

#[function_component(Editor)]
fn editor() -> Html {
    let content = r#"{title: Du hast einen Plan}
{artist: Felix Rollbühler}
{key: D}
{section: Intro}
[D A/C# Bm G]
{section: Verse 1}
[D]Manchmal frag ich [G]mich, muss denn das so [D]sein,
denn ich weiß es [Bm]nicht, mein Verstand ist zu [A]klein.
Im [D]Gebet frag ich [G]dich und ich weiß, du hörst mir [D]zu,
darum frag ich [Em7]dich, was ist dein Plan für [A]mich?
{section: Interlude 1}
[D G D Em7 A]
{section: Chorus}
Du [D]hast einen Plan, du [A]hast einen Plan,
du [Bm]hast einen Plan, mit [G]mir. (2x)
{section: Verse 2}
[D]Herr hilf mir [G]versteh`n, zu hören, wenn du [D]sprichst,
deine Antwort [Bm]kommt, dessen bin ich mir [A]gewiss.
[D]Herr hilf mir zu [G]seh`n, was du mir zeigen [D]willst,
was wir jetzt nicht [Em7]verstehn gibt später einen [A]Sinn.
{section: Chorus}
{section: Interlude 2}
[Bm A/C# Bm G Bm A/C# Bm A]
{section: Bridge}
Ich [Bm]werde warten Herr, [G]warten Herr,
[Em]warten Herr, bis du [A]sprichst.
Ich werd` [Bm]vertrauen Herr, [G]vertrauen Herr,
ver[Em]trauen Herr, auf deinen [A]Plan. (2x)"#
        .to_string();

    let log_content = Callback::from(move |content: String| log!(content));
    let onautoformat = Callback::from(move |content: String| content.replace("{", "\n{"));

    let syntax_parser = SyntaxParser::builder()
        .transition("default", "{", "meta-begin", Some("default"), 1)
        .transition("meta-begin", "{", "meta-begin", None, 0)
        .transition("meta-begin", ":", "meta-middle", None, 1)
        .transition("meta-begin", "}", "meta-end", None, 1)
        .transition("meta-begin", "", "meta-key", Some("meta-surround"), 1)
        .transition("meta-key", "title:", "meta-middle", Some("meta-key"), 1)
        .transition("meta-key", "artist:", "meta-middle", Some("meta-key"), 1)
        .transition("meta-key", "key:", "meta-middle", Some("meta-key"), 1)
        .transition("meta-key", "section:", "meta-middle", Some("meta-key"), 1)
        .transition("meta-key", ":", "meta-middle", Some("meta-key-error"), 1)
        .transition("meta-key", "}", "meta-end", Some("meta-key"), 1)
        .transition("meta-middle", ":", "meta-middle", None, 0)
        .transition("meta-middle", "}", "meta-end", None, 1)
        .transition("meta-middle", "", "meta-value", Some("meta-surround"), 1)
        .transition("meta-value", "}", "meta-end", Some("meta-value"), 1)
        .transition("meta-end", "}", "default", Some("meta-surround"), 0)
        .transition("default", "[", "chord", Some("default"), 1)
        .transition("chord", "[", "chord", None, 0)
        .transition("chord", "]", "default", Some("chord"), 0)
        .label_style("meta-surround", "font-weight", "bold")
        .label_style("meta-key", "color", "#cc241d")
        .label_style("meta-key-error", "text-decoration", "underline")
        .label_style("meta-key-error", "text-decoration-color", "#cc241d")
        .label_style("meta-value", "color", "#98971a")
        .label_style("chord", "color", "#d79921")
        .build()
        .expect("static parser should build");

    html! {
        <EditorComponent
            content={content}
            onsave={log_content.clone()}
            ondiscard={log_content.clone()}
            ondelete={log_content}
            onautoformat={onautoformat}
            syntax_parser={syntax_parser}
        />
    }
}

#[function_component(Chart)]
fn chart() -> Html {
    let chart_config = ConfigBuilder::bar()
        .labels(&vec!["Jan", "Feb", "Mar", "Apr", "Mai", "Jun"])
        .dataset(&vec![1., 6., 3., 4., 2., 5.])
        .dataset_label("First Data")
        .dataset_stack(0)
        .dataset_border_color_rgba(0, 255, 0, 0.4)
        .dataset_border_width(1)
        .dataset_background_color_rgba(0, 255, 0, 0.2)
        .dataset(&vec![6., 3., 4., 2., 5., 1.])
        .dataset_label("Second Data")
        .dataset_stack(0)
        .dataset_border_color_rgba(0, 0, 255, 0.5)
        .dataset_border_width(1)
        .dataset_background_color_rgba(0, 0, 255, 0.2)
        .dataset(&vec![3., 4., 2., 5., 1., 6.7])
        .dataset_label("Third Data")
        .dataset_border_color_rgba(255, 0, 0, 0.45)
        .dataset_border_width(1)
        .dataset_background_color_rgba(255, 0, 0, 0.2)
        .build()
        .unwrap();
    html! {
        <ChartJs config={chart_config}/>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let mut map: HashMap<String, f64> = HashMap::new();
    map.insert("test".into(), 1999.);
    let bind_handle = use_state(|| map);
    let bind_handle2 = use_state(|| vec!["file1.txt".into(), "file2.txt".into()]);
    let on_show_success = {
        |_: MouseEvent| {
            fancy_yew::toast_notifications::show_success("Error", "This is a success");
        }
    };
    let on_show_error = {
        |_: MouseEvent| {
            fancy_yew::toast_notifications::show_error("Error", "This is an error");
        }
    };
    let on_show_warning = {
        |_: MouseEvent| {
            fancy_yew::toast_notifications::show_warning("Error", "This is a warning");
        }
    };
    let on_show_info = {
        |_: MouseEvent| {
            fancy_yew::toast_notifications::show_info("Error", "This is an info");
        }
    };
    html! {
        <div style="width: 30vw;">
            <StringNumberMap
                bind_handle={bind_handle}
                min=0.
                max=10000.
                options={vec!{"A".into(), "B".into()}}
                />
            <RemoteFileInput<String>
                bind_handle={bind_handle2}
                endpoint="/api/blobs"
            />
            <span onclick={on_show_success}>{"Show Success"}</span>
            <span onclick={on_show_error}>{"Show Error"}</span>
            <span onclick={on_show_warning}>{"Show Warning"}</span>
            <span onclick={on_show_info}>{"Show Info"}</span>
        </div>
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/account")]
    Account,
    #[at("/home")]
    Home,
    #[at("/chart")]
    Chart,
    #[at("/editor")]
    Editor,
    #[at("/fullscreen")]
    FullScreen,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Navable for Route {
    fn route_items() -> Vec<Self> {
        vec![Route::Home, Route::Chart, Route::Editor, Route::FullScreen]
    }

    fn to_nav_item(self) -> NavItemBuilder<'static> {
        match self {
            Route::Home => NavItemBuilder::new()
                .path("/home")
                .icon("home")
                .text("Home")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Home)
                }))
                .index(),
            Route::Account => NavItemBuilder::new()
                .path("/account")
                .icon("account_circle")
                .text("Account")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Account)
                })),
            Route::Chart => NavItemBuilder::new()
                .path("/chart")
                .icon("bar_chart")
                .text("Chart")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Chart)
                })),
            Route::Editor => NavItemBuilder::new()
                .path("/editor")
                .icon("edit")
                .text("Editor")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Editor)
                })),
            Route::FullScreen => NavItemBuilder::new()
                .path("/fullscreen")
                .icon("fullscreen")
                .text("FullScreen")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::FullScreen)
                })),
            _ => NavItemBuilder::new(),
        }
    }

    fn render(route: Route) -> Html {
        html! {
            <Layout<Route>
                nav_routes={Route::route_items()}
                account_route={Route::Account}
                fullscreen={match route {
                    Route::FullScreen => true,
                    _ => false,
                }}
            >{
                match route {
                    Route::Index => html! { <Home /> },
                    Route::Home => html! { <Home /> },
                    Route::Chart => html! { <Chart /> },
                    Route::Editor => html! { <Editor /> },
                    Route::FullScreen => html! { <FullScreen /> },
                    Route::Account => html! { <h1>{"Account"}</h1> },
                    Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
        }}
            </Layout<Route>>
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Route::render} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

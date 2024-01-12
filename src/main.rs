use fancy_yew::components::{
    input::StringNumberMap, ChartJs, ConfigBuilder, DefaultLayout, NavItemBuilder, Navable,
};
use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
fn Chart() -> Html {
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

#[function_component]
fn Home() -> Html {
    let mut map: HashMap<String, f64> = HashMap::new();
    map.insert("test".into(), 1999.);
    let bind_handle = use_state(|| map);
    html! {
        <div style="width: 30vw;">
            <StringNumberMap
                bind_handle={bind_handle}
                min=0.
                max=10000.
                options={vec!{"A".into(), "B".into()}}
                />
        </div>
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/home")]
    Home,
    #[at("/contact")]
    Contact,
    #[at("/chart")]
    Chart,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {}

impl Navable for Route {
    fn route_items() -> Vec<Self> {
        vec![Route::Home, Route::Contact, Route::Chart]
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
            Route::Contact => NavItemBuilder::new()
                .path("/contact")
                .icon("contact_page")
                .text("Contact")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Contact)
                })),
            Route::Chart => NavItemBuilder::new()
                .path("/chart")
                .icon("bar_chart")
                .text("Chart")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Chart)
                })),
            _ => NavItemBuilder::new(),
        }
    }

    fn render(route: Route) -> Html {
        html! {
            <DefaultLayout<Route> nav_routes={Route::route_items()}>{
                match route {
                    Route::Index => html! { <h1>{ "Home" }</h1> },
                    Route::Home => html! { <Home />},
                    Route::Contact => html! { <h1>{ "Contact" }</h1> },
                    Route::Chart => html! { <Chart /> },
                    Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
        }}
            </DefaultLayout<Route>>
        }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Route::render} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

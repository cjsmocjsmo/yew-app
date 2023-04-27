use yew_router::prelude::*;
use yew::prelude::*;
// use yew::{function_component, html, Html};
// use stylist::yew::styled_component;
// use stylist::yew::use_style;
// use yew_svg;

mod comps;

mod actionpage;



#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/action")]
    Action,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
fn CatAction() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Action));

    html!{

        <h1 {onclick} style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Action"}</h1>
    }
}

#[function_component]
fn MovieCatagories() -> Html {

    html! {
        <>
            <div style="display:flex;flex-direction:row;flex-wrap:wrap;justify-content:center;align-items:center;margin-bottom:60px;">
                <CatAction />
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Arnold"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Bruce Willis"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Cartoons"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Comedy"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Drama"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Documentary"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Fantasy"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Godzilla"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Harry Potter"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Indiana Jones"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"James Bond"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"John Wayne"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"John Wick"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Jurassic Park"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Kings Men"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Men In Black"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Misc"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Nicolas Cage"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Pirates"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Riddick"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Star Wars"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Star Trek"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Super Heroes"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"SciFi"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Tom Cruize"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Transformers"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"Tremors"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"The Rock"}</h1>
                <h1 style="font-size:1.5em;padding:10px;margin:10px;color:white">{"XMen"}</h1>
            </div>
        </>
    }
}



// #[function_component]
// fn ActionPage() -> Html {
//     let navigator = use_navigator().unwrap();

//     let onclick = Callback::from(move |_| navigator.push(&Route::Home));
//     html!(
//         <>
//             <h1>{"Action Page"}</h1>
//             <button {onclick}>{"Home"}</button>
//         </>
//     )
// }

#[function_component]
fn MainPage() -> Html {
    // let navigator = use_navigator().unwrap();

    // let onclick = Callback::from(move |_: MouseEvent | navigator.push(&Route::Action));
    html!(
        <>
            <main style="height:100vh;width:900px;margin-right:auto;margin-left:auto;">

                <comps::home_page_comps::MyTitle />

                <div style="display:flex;flex-direction:row;justify-content:center;align-items:center;">
                    <comps::home_page_comps::MovieSVG />
                </div>

                <hr style="width:75%;background-color:white;" />

                

                <MovieCatagories />

                <comps::home_page_comps::PlayerControls />

                <comps::home_page_comps::ImageGroupTwo />

            </main>
        </>
    )
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{ <MainPage /> },
        Route::Action => html!{ <actionpage::ActionPage />},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {

        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter> 
    
       

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

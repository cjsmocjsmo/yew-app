use yew_router::prelude::*;
use yew::prelude::*;
use stylist::yew::use_style;

#[function_component]
fn CatFantasy() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::TVFantazy));

    let starwars_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={starwars_h1}>{"Fantasy"}</h1>
    }
}

#[function_component]
fn CatStarWars() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::TVStarWars));

    let starwars_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={starwars_h1}>{"Star Wars"}</h1>
    }
}

#[function_component]
fn CatStarTrek() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::TVStarTrek));

    let starwars_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={starwars_h1}>{"Star Trek"}</h1>
    }
}

#[function_component]
fn CatSciFi() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::TVSciFi));

    let starwars_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={starwars_h1}>{"SciFi"}</h1>
    }
}

#[function_component]
fn CatMCU() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::MCU));

    let starwars_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={starwars_h1}>{"MCU"}</h1>
    }
}

#[function_component]
fn CatWestern() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Western));

    let starwars_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={starwars_h1}>{"Western"}</h1>
    }
}

#[function_component]
pub fn TVShowsPage() -> Html {
    let main_style = use_style!("
        height: 100vh;
        width: 900px;
        margin-right: auto;
        margin-left: auto;
    ");

    let hr_style = use_style!("
        width: 75%;
        background-color: white;
    ");

    html!(
        <>
            <main class={ main_style }>

                <crate::comps::mainpage_comps::MyTitle />

                <crate::comps::mainpage_comps::MovieSVG />

                <hr class={ hr_style } />

                <CatFantasy />

                <CatStarTrek />

                <CatStarWars />

                <CatSciFi />

                <CatMCU />

                <CatWestern />


            </main>
        </>
    )
}
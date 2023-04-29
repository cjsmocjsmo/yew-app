use stylist::yew::use_style;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn ImageGroupB() -> Html {
    let img_div = use_style!(
        "
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        margin-top: 5em;
        margin-bottom: 60px;
    "
    );

    html!(
        <>
            <div class={ img_div }>
                <crate::images::nineteen23_img_comp::Nineteen23ImgComp />
            </div>
        </>
    )
}

#[function_component]
fn WBCom() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::TVShows));
    let western_button = use_style!("
        padding: 8px;
        font-size: 1.125em; 
        background-color: #ebb917;
    ");

    html!(
        <>
            <button {onclick} class={western_button}>{"TVShows"}</button>
        </>
    )
}

#[function_component]
pub fn WesternPage() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Home));

    let western_style = use_style!(
        "
        text-align: center;
        color: #ebb917;
    "
    );

    let western_button = use_style!(
        "
        padding:8px;
        font-size: 1.125em;
        background-color: #ebb917;
        
    "
    );

    let western_btn_group = use_style!(
        "
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: space-evenly;
        align-items: center;
        margin-top: 5em;
        margin-bottom: 60px;
    "
    );

    html!(
        <>
            <h1 class={western_style}>{"Westeren Page"}</h1>
            <div class={western_btn_group}>
                <button {onclick} class={western_button}>{"Movies"}</button>
                <WBCom />
            </div>


            <ImageGroupB />
        </>
    )
}

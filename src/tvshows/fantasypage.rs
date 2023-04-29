use yew_router::prelude::*;
use yew::prelude::*;
use stylist::yew::use_style;

#[function_component]
pub fn ImageGroupB() -> Html {
    let img_div = use_style!("
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: space-evenly;
        align-items: center;
        margin-bottom: 60px;
    ");

    html!(
        <>
            <div class={ img_div }>
                <crate::images::house_of_the_dragon_img_comp::HouseOfTheDragonImgComp />
                <crate::images::the_lord_of_the_rings_img_comp::TheLordOfTheRingsImgComp />
                <crate::images::wheel_of_time_img_comp::WheelOfTimeImgComp />
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
pub fn TVFantasyPage() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Home));
    
    let btn_group = use_style!(
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
            <h1 style="text-align:center;color: #ebb917;">{"Fantasy Page"}</h1>
            <div class={btn_group}>
                <button {onclick} style="padding:8px;background-color: #ebb917;font-size: 1.125em;">{"Movies"}</button>
                <WBCom />
            </div>    
            <ImageGroupB />
            
        </>
    )
}
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

    let img_container = use_style!("
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: 210px;
        height: 400px;
        background-color: black;
        border: 3px solid black;
        border-radius: 8px;
    ");

    let season_style = use_style!("
        color:blue;
        font-size: 1.75em;
        text-decoration: underline;
    ");

    html!(
        <>
            <div class={ img_div }>
                <div class={img_container.clone()}>
                    <crate::images::house_of_the_dragon_img_comp::HouseOfTheDragonImgComp />
                    <p class={season_style.clone()}>{"1"}</p>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::the_lord_of_the_rings_img_comp::TheLordOfTheRingsImgComp />
                    <p class={season_style.clone()}>{"1"}</p>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::wheel_of_time_img_comp::WheelOfTimeImgComp />
                    <p class={season_style.clone()}>{"1"}</p>
                </div>
                
               
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
        margin-top: 1em;
        margin-bottom: 1em;
    "
    );

    let button_style = use_style!("
        padding: 8px;
        background-color: #ebb917;
        font-size: 1.125em;
    ");
    
    html!(
        <>
            <div class={btn_group}>
                <button class={button_style.clone()}>{"Music"}</button>
                <button {onclick} class={button_style.clone()}>{"Movies"}</button>
                <WBCom />
            </div>
            <h1 style="text-align:center;color:#ebb917;">{"Fantasy"}</h1>
            <ImageGroupB />
        </>
    )
}
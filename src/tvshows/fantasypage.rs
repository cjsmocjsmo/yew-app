use yew::prelude::*;
use stylist::yew::use_style;

#[function_component]
pub fn ThumbnailGroup() -> Html {
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
pub fn TVFantasyPage() -> Html {
       
    let h1_style = use_style!("
        text-align: center;
        color: #ebb917;
    ");
    
    html!(
        <>
            <crate::comps::mainpage_comps::MyHeader />
            <h1 class={h1_style}>{"Fantasy"}</h1>
            <ThumbnailGroup />
        </>
    )
}
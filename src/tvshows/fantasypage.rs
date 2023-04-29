use yew_router::prelude::*;
use yew::prelude::*;
use stylist::yew::use_style;

#[function_component]
pub fn ImageGroupB() -> Html {
    let img_div = use_style!("
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        margin-bottom: 60px;
    ");

    html!(
        <>
            <div class={ img_div }>
                <crate::images::the_lord_of_the_rings_img_comp::TheLordOfTheRingsImgComp />
                <crate::images::wheel_of_time_img_comp::WheelOfTimeImgComp />
                <crate::images::nineteen23_img_comp::Nineteen23ImgComp />
                <crate::images::prehistoric_planet_img_comp::PrehistoricPlanetImgComp />
                
                <crate::images::wheel_of_time_img_comp::WheelOfTimeImgComp />
                <crate::images::nineteen23_img_comp::Nineteen23ImgComp />
                <crate::images::prehistoric_planet_img_comp::PrehistoricPlanetImgComp />
                <crate::images::wheel_of_time_img_comp::WheelOfTimeImgComp />
                <crate::images::nineteen23_img_comp::Nineteen23ImgComp />
                <crate::images::prehistoric_planet_img_comp::PrehistoricPlanetImgComp />
                
                <crate::images::wheel_of_time_img_comp::WheelOfTimeImgComp />
                <crate::images::nineteen23_img_comp::Nineteen23ImgComp />
                <crate::images::prehistoric_planet_img_comp::PrehistoricPlanetImgComp />
            </div>
        </>
    )
}

#[function_component]
pub fn TVFantasyPage() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Home));
    html!(
        <>
            <h1 style="text-align:center;color:white">{"Fantasy Page"}</h1>
            <button {onclick} style="padding:8px;">{"Home"}</button>
            <ImageGroupB />
            
        </>
    )
}
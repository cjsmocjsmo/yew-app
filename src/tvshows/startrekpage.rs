use yew_router::prelude::*;
use yew::prelude::*;
use stylist::yew::use_style;

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

    let season_div = use_style!("
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: space-evenly;
        align-items: center;
        width:100%;
    ");

    html!(
        <>
            <div class={ img_div }>
                <div class={img_container.clone()}>
                    <crate::images::discovery_img_comp::DiscoveryImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                        <p class={season_style.clone()}>{"3"}</p>
                        <p class={season_style.clone()}>{"4"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::enterprise_img_comp::EnterpriseImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                        <p class={season_style.clone()}>{"3"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::lowerdecks_img_comp::LowerDecksImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                        <p class={season_style.clone()}>{"3"}</p>
                    </div>
                </div>

                <div class={img_container.clone()}>
                    <crate::images::nextgeneration_img_comp::NextGenerationImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                        <p class={season_style.clone()}>{"3"}</p>
                        <p class={season_style.clone()}>{"4"}</p>
                        <p class={season_style.clone()}>{"5"}</p>
                        <p class={season_style.clone()}>{"6"}</p>
                        <p class={season_style.clone()}>{"7"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::picard_img_comp::PicardImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                        <p class={season_style.clone()}>{"3"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::prodigy_img_comp::ProdigyImgComp />
                    <p class={season_style.clone()}>{"1"}</p>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::strange_new_worlds_img_comp::StrangeNewWorldsImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::sttv_img_comp::STTVImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                        <p class={season_style.clone()}>{"3"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::voyager_img_comp::VoyagerImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                        <p class={season_style.clone()}>{"3"}</p>
                        <p class={season_style.clone()}>{"4"}</p>
                        <p class={season_style.clone()}>{"5"}</p>
                        <p class={season_style.clone()}>{"6"}</p>
                        <p class={season_style.clone()}>{"7"}</p>
                    </div>
                </div>
            </div>
        </>
    )
}

#[function_component]
pub fn TVStarTrekPage() -> Html {
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

    html!(
        <>
            
            <div class={btn_group}>
                <button style="padding:8px;background-color: #ebb917;font-size: 1.125em;">{"Music"}</button>
                <button {onclick} style="padding:8px;background-color: #ebb917;font-size: 1.125em;">{"Movies"}</button>
                <WBCom />
            </div>
            <h1 style="text-align:center;color:#ebb917">{"Star Trek"}</h1>
            <ImageGroupB />

        </>
    )
}
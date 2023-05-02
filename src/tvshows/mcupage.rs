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
                    <crate::images::falcon_and_the_winter_soldier_img_comp::FalconAndTheWinterSoldierImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::hawkeye_img_comp::HawkeyeImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::i_am_groot_img_comp::IAmGrootImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::loki_img_comp::LokiImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::moonknight_img_comp::MoonKnightImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::ms_marvel_img_comp::MsMarvelImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::she_hulk_img_comp::SheHulkImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::wandavision_img_comp::WandaVisionImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>
            </div>
        </>
    )
}

#[function_component]
pub fn MCUPage() -> Html {
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

    let h1_style = use_style!("
        text-align: center;
        color: #ebb917;
    ");

    html!(
        <>
            <div class={btn_group}>
                <crate::comps::mainpage_comps::MusicSVG />
                <crate::comps::mainpage_comps::MovieSVG />
                <crate::comps::mainpage_comps::TVShowsSVG />
            </div>
            <h1 class={h1_style}>{"MCU"}</h1>
            <ImageGroupB />
        </>
    )
}
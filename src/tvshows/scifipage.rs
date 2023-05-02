// use yew_router::prelude::*;
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
                    <crate::images::alienworlds_img_comp::AlienWorldsImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::altered_carbon_img_comp::AlteredCarbonImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::cowboy_bebop_img_comp::CowboyBebopImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>
                <div class={img_container.clone()}>
                    <crate::images::for_all_man_kind_img_comp::ForAllManKindImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                        <p class={season_style.clone()}>{"3"}</p>
                    </div>
                </div>

                <div class={img_container.clone()}>
                    <crate::images::foundation_img_comp::FoundationImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>
                
                <div class={img_container.clone()}>
                    <crate::images::halo_img_comp::HaloImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>

                <div class={img_container.clone()}>
                    <crate::images::lost_in_space_img_comp::LostInSpaceImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                        <p class={season_style.clone()}>{"3"}</p>
                    </div>
                </div>
                
                <div class={img_container.clone()}>
                    <crate::images::last_of_use_img_comp::TheLastOfUsImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>


                <div class={img_container.clone()}>
                    <crate::images::night_sky_img_comp::NightSkyImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>
                
                <div class={img_container.clone()}>
                    <crate::images::orville_img_comp::OrvilleImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                        <p class={season_style.clone()}>{"3"}</p>
                    </div>
                </div>
                
                
                <div class={img_container.clone()}>
                    <crate::images::raised_by_wolves_img_comp::RaisedByWolvesImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                    </div>
                </div>
            </div>
        </>
    )
}

#[function_component]
pub fn TVSciFiPage() -> Html {
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
            <h1 class={h1_style}>{"SciFi"}</h1>
            <ImageGroupB />
        </>
    )
}
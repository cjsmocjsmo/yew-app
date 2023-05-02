use stylist::yew::use_style;
use yew::prelude::*;
// use yew_router::prelude::*;

#[function_component]
pub fn StarWarsImgGroup() -> Html {
    let img_div = use_style!(
        "
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        margin-bottom: 60px;
    "
    );

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
                    <crate::images::andor_img_comp::AndorImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>

                <div class={img_container.clone()}>
                    <crate::images::bad_batch_img_comp::TheBadBatchImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                    </div>
                </div>

                <div class={img_container.clone()}>
                    <crate::images::book_of_boba_fett_img_comp::BookOfBobaFettImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>
                </div>

                <div class={img_container.clone()}>
                    <crate::images::mandalorian_img_comp::MandalorianImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                        <p class={season_style.clone()}>{"3"}</p>
                    </div>
                </div>

                <div class={img_container.clone()}>
                    <crate::images::obi_wan_kenobi_img_comp::ObiWanKenobiImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                        <p class={season_style.clone()}>{"2"}</p>
                    </div>
                </div>

                <div class={img_container.clone()}>
                    <crate::images::visions_img_comp::VisionsImgComp />
                    <div class={season_div.clone()}>
                        <p class={season_style.clone()}>{"1"}</p>
                    </div>   
                </div>

                <div class={img_container.clone()}>
                    <crate::images::tales_of_the_jedi_img_comp::TalesOfTheJediImgComp />
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
pub fn TVStarWarsPage() -> Html {
    let h1_style = use_style!(
        "
        text-align: center;
        color: #ebb917;
    "
    );

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
                <crate::comps::mainpage_comps::MusicSVG />
                <crate::comps::mainpage_comps::MovieSVG />
                <crate::comps::mainpage_comps::TVShowsSVG />
            </div>
            <h1 class={h1_style}>{"Star Wars"}</h1>
            <StarWarsImgGroup />

        </>
    )
}

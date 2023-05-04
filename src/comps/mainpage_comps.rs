use yew_router::prelude::*;
use yew::prelude::*;
use stylist::yew::use_style;

#[function_component]
pub fn MyTitle() -> Html {
    let h1style = use_style!("
        font-size: 3em;
        padding: 20px;
        color: #ca25d9;
        text-align: center;
    ");

    html! {<h1 class={h1style} >{ "UTV" }</h1>}
}

#[function_component]
pub fn MyHeader() -> Html {
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

    html!{
        <div class={btn_group}>
            <crate::comps::mainpage_comps::MusicSVG />
            <crate::comps::mainpage_comps::MovieSVG />
            <crate::comps::mainpage_comps::TVShowsSVG />
        </div>
    }
}

#[function_component]
pub fn MovieSVG() -> Html {
    let svg_div = use_style!("
        display:flex;
        flex-direction:row;
        justify-content:center;
        align-items:center;
    ");
    // let asvg = use_style!("
    //     margin-bottom: 3em;
    //     margin-left: auto;
    //     margin-right: auto;
    // ");
    let svgstyle = use_style!("
        fill: #ebb917;
        width: 4em;
        text-align: center;
        margin-bottom: 3em;
    ");

    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Home));

    html! {
        <div class={svg_div}>
            
                <svg {onclick} class={svgstyle} viewBox="0 0 16 16">
                    <path d="M0 1a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H1a1 1 0 0 1-1-1V1zm4 0v6h8V1H4zm8 8H4v6h8V9zM1 1v2h2V1H1zm2 3H1v2h2V4zM1 7v2h2V7H1zm2 3H1v2h2v-2zm-2 3v2h2v-2H1zM15 1h-2v2h2V1zm-2 3v2h2V4h-2zm2 3h-2v2h2V7zm-2 3v2h2v-2h-2zm2 3h-2v2h2v-2z"/>
                </svg>
            
        </div>

    }
}

#[function_component]
pub fn TVShowsSVG() -> Html {
    let svg_div = use_style!("
        display:flex;
        flex-direction:row;
        justify-content:center;
        align-items:center;
    ");

    let svgstyle = use_style!("
        fill: #ebb917;
        width: 4em;
        text-align: center;
        margin-bottom: 3em;
    ");

    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::TVShows));

    html! {
        <div {onclick} class={svg_div}>
                <svg 
                    class={svgstyle}
                    viewBox="0 0 640 512">
                    <path d="M64 64V352H576V64H64zM0 0H64 576h64V64 352v64H576 64 0V352 64 0zM128 448H512h32v64H512 128 96V448h32z"/>
                </svg>
        </div>

    }
}

#[function_component]
pub fn MusicSVG() -> Html {
    let svg_div = use_style!("
        display:flex;
        flex-direction:row;
        justify-content:center;
        align-items:center;
    ");

    let svgstyle = use_style!("
        fill: #ebb917;
        width: 4em;
        text-align: center;
        margin-bottom: 3em;
    ");

    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Music));

    html! {
        <div {onclick} class={svg_div}>
            <svg 
                class={svgstyle}
                viewBox="0 0 16 16">
                <path d="M6 13c0 1.105-1.12 2-2.5 2S1 14.105 1 13c0-1.104 1.12-2 2.5-2s2.5.896 2.5 2zm9-2c0 1.105-1.12 2-2.5 2s-2.5-.895-2.5-2 1.12-2 2.5-2 2.5.895 2.5 2z"/>
                <path fill-rule="evenodd" d="M14 11V2h1v9h-1zM6 3v10H5V3h1z"/>
                <path d="M5 2.905a1 1 0 0 1 .9-.995l8-.8a1 1 0 0 1 1.1.995V3L5 4V2.905z"/>
            </svg>
        </div>
    }
}

// #[function_component]
// fn ImageGroupOne() -> Html {
//     let igo = use_style!("
//         display: flex;
//         flex-direction: row;
//         flex-wrap: wrap;
//         justify-content: center;
//         align-items: center;
//         margin-bottom: 30px;
//     ");

//     let image_css = use_style!("
//         display: block;
//         margin-top: 10px;
//         margin-right: auto;
//         margin-left: auto;
//         width: 200px;
//         border-radius: 8px;
//     ");

//     let image_css2 = use_style!("
//         display: block;
//         margin-top: 10px;
//         margin-right: auto;
//         margin-left: auto;
//         width: 200px;
//         border-radius: 8px;
//     ");

//     html! {
//         <div class={ igo }>
//             <img class={ image_css }
//                 src="https://www.nasa.gov/sites/default/files/styles/full_width_feature/public/thumbnails/image/pia22486-main.jpg"
//                 alt="video thumbnail" />

//             <img class={ image_css2 }
//                 src="https://www.nasa.gov/sites/default/files/styles/full_width_feature/public/thumbnails/image/pia22486-main.jpg"
//                 alt="video thumbnail" />

//         </div>
//     }
// }



#[function_component]
pub fn PlayerControls() -> Html {
    let player_control_div = use_style!("
        display: flex;
        flex-direction: row;
        flex-grow: 1;
        width: 100%;
        justify-content: center;
        align-items: center;
    ");

    let button_div = use_style!("
        margin: 4em;
    ");

    // let btn_style = use_style!("
    //     background-color: black;
    //     color: black;
    // ");

    let svg_style = use_style!("
        fill: #ebb917;
        background-color: black;
        border-radius: 8px;
        width: 2.75em;
        padding: 1em;
    ");

    html!(
        <div class={player_control_div}>
            <div class={button_div.clone()}>
                <svg id="previousBtn"
                    class={svg_style.clone()}
                    viewBox="0 0 320 512">
                    <path d="M267.5 440.6c9.5 7.9 22.8 9.7 34.1 4.4s18.4-16.6 18.4-29V96c0-12.4-7.2-23.7-18.4-29s-24.5-3.6-34.1 4.4l-192 160L64 241V96c0-17.7-14.3-32-32-32S0 78.3 0 96V416c0 17.7 14.3 32 32 32s32-14.3 32-32V271l11.5 9.6 192 160z"></path>
                </svg>
            </div>

            <div class={button_div.clone()}>
                <svg id="Stop"
                    class={svg_style.clone()}
                    viewBox="0 0 320 512">
                    <path d="M48 64C21.5 64 0 85.5 0 112V400c0 26.5 21.5 48 48 48H80c26.5 0 48-21.5 48-48V112c0-26.5-21.5-48-48-48H48zm192 0c-26.5 0-48 21.5-48 48V400c0 26.5 21.5 48 48 48h32c26.5 0 48-21.5 48-48V112c0-26.5-21.5-48-48-48H240z"/>
                </svg>
            </div>

            <div class={button_div.clone()}>
                <svg id="Next"
                    class={svg_style.clone()}
                    viewBox="0 0 320 512">
                    <path d="M52.5 440.6c-9.5 7.9-22.8 9.7-34.1 4.4S0 428.4 0 416V96C0 83.6 7.2 72.3 18.4 67s24.5-3.6 34.1 4.4l192 160L256 241V96c0-17.7 14.3-32 32-32s32 14.3 32 32V416c0 17.7-14.3 32-32 32s-32-14.3-32-32V271l-11.5 9.6-192 160z"/>
                </svg>
            </div>
        </div>
    )
}

#[function_component]
pub fn ImageGroupTwo() -> Html {
    // let b64_style = use_style!("
    //     width: 250px;
    //     border-radius: 8px;
    // ");

    html!(
        <>
            <div style="display:flex;flex-direction:row;flex-wrap:wrap;justify-content:center;align-items:center;margin-bottom:60px;">

                <img style="display:block;margin-top:10px;margin-right:auto;margin-left:auto;width:200px;border-radius:8px;"
                src="https://www.nasa.gov/sites/default/files/styles/full_width_feature/public/thumbnails/image/pia22486-main.jpg" alt="video thumbnail" />

                <img style="display:block;margin-top:10px;margin-right:auto;margin-left:auto;width:200px;border-radius:8px;"
                src="https://www.nasa.gov/sites/default/files/styles/full_width_feature/public/thumbnails/image/pia22486-main.jpg" alt="video thumbnail" />

                <img style="display:block;margin-top:10px;margin-right:auto;margin-left:auto;width:200px;border-radius:8px;"
                src="https://www.nasa.gov/sites/default/files/styles/full_width_feature/public/thumbnails/image/pia22486-main.jpg" alt="video thumbnail" />

                
            </div>
        </>
    )
}

#[function_component]
fn CatAction() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Action));

    let action_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{

        <h1 {onclick} class={action_h1}>{"Action"}</h1>
    }
}

#[function_component]
fn CatArnold() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Arnold));

    let arnold_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{

        <h1 {onclick} class={arnold_h1}>{"Arnold"}</h1>
    }
}

#[function_component]
fn CatBruceWillis() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::BruceWillis));

    html!{

        <h1 {onclick} style="font-size:1.5em;padding:10px;margin:10px;color:blue">{"Bruce Willis"}</h1>
    }
}

#[function_component]
fn CatCartoons() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Cartoons));

    html!{

        <h1 {onclick} style="font-size:1.5em;padding:10px;margin:10px;color:blue">{"Cartoons"}</h1>
    }
}

#[function_component]
fn CatComedy() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Comedy));

    html!{

        <h1 {onclick} style="font-size:1.5em;padding:10px;margin:10px;color:blue">{"Comedy"}</h1>
    }
}

#[function_component]
fn CatDrama() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Drama));

    html!{

        <h1 {onclick} style="font-size:1.5em;padding:10px;margin:10px;color:blue">{"Drama"}</h1>
    }
}

#[function_component]
fn CatDocumentary() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Documentary));

    html!{

        <h1 {onclick} style="font-size:1.5em;padding:10px;margin:10px;color:blue">{"Documentary"}</h1>
    }
}

#[function_component]
fn CatFantasy() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Fantasy));

    html!{

        <h1 {onclick} style="font-size:1.5em;padding:10px;margin:10px;color:blue">{"Fantasy"}</h1>
    }
}

#[function_component]
fn CatGodzilla() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Godzilla));

    html!{

        <h1 {onclick} style="font-size:1.5em;padding:10px;margin:10px;color:blue">{"Godzilla"}</h1>
    }
}

#[function_component]
fn CatHarryPotter() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::HarryPotter));

    html!{

        <h1 {onclick} style="font-size:1.5em;padding:10px;margin:10px;color:blue">{"Harry Potter"}</h1>
    }
}

#[function_component]
fn CatIndianaJones() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::IndianaJones));

    html!{

        <h1 {onclick} style="font-size:1.5em;padding:10px;margin:10px;color:blue">{"Indiana Jones"}</h1>
    }
}

#[function_component]
fn CatJamesBond() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::JamesBond));

    let jamesbond_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{

        <h1 {onclick} class={jamesbond_h1}>{"James Bond"}</h1>
    }
}

#[function_component]
fn CatJohnWayne() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::JohnWayne));

    let johnwayne_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{

        <h1 {onclick} class={johnwayne_h1}>{"John Wayne"}</h1>
    }
}

#[function_component]
fn CatJohnWick() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::JohnWick));

    let johnwayne_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{

        <h1 {onclick} class={johnwayne_h1}>{"John Wick"}</h1>
    }
}

#[function_component]
fn CatJurassicPark() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::JurassicPark));

    let johnwayne_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{

        <h1 {onclick} class={johnwayne_h1}>{"Jurassic Park"}</h1>
    }
}

#[function_component]
fn CatKingsMen() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::KingsMen));

    let kingsmen_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={kingsmen_h1}>{"Kings Men"}</h1>
    }
}

#[function_component]
fn CatMenInBlack() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::MenInBlack));

    let kingsmen_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={kingsmen_h1}>{"Men In Black"}</h1>
    }
}

#[function_component]
fn CatMisc() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Misc));

    let misc_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={misc_h1}>{"Misc"}</h1>
    }
}

#[function_component]
fn CatNicolasCage() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::NicolasCage));

    let misc_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={misc_h1}>{"Nicolas Cage"}</h1>
    }
}

#[function_component]
fn CatPirates() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Pirates));

    let misc_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={misc_h1}>{"Pirates"}</h1>
    }
}

#[function_component]
fn CatRiddick() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Riddick));

    let riddick_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={riddick_h1}>{"Riddick"}</h1>
    }
}

#[function_component]
fn CatSciFi() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::SciFi));

    let riddick_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={riddick_h1}>{"SciFi"}</h1>
    }
}

#[function_component]
fn CatStarTrek() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::StarTrek));

    let startrek_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={startrek_h1}>{"Star Trek"}</h1>
    }
}

#[function_component]
fn CatStarWars() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::StarWars));

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
fn CatSuperHeroes() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::SuperHeroes));

    let starwars_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={starwars_h1}>{"Super Heroes"}</h1>
    }
}

#[function_component]
fn CatTremors() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Tremors));

    let starwars_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={starwars_h1}>{"Tremors"}</h1>
    }
}

#[function_component]
fn CatTheRock() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::TheRock));

    let starwars_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={starwars_h1}>{"The Rock"}</h1>
    }
}

#[function_component]
fn CatTomCruize() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::TomCruize));

    let starwars_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={starwars_h1}>{"Tom Cruize"}</h1>
    }
}

#[function_component]
fn CatTransformers() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Transformers));

    let starwars_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={starwars_h1}>{"Transformers"}</h1>
    }
}

#[function_component]
fn CatXMen() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::XMen));

    let starwars_h1 = use_style!("
        font-size: 1.5em;
        padding: 10px;
        margin: 10px;
        color: blue;
    ");

    html!{
        <h1 {onclick} class={starwars_h1}>{"XMen"}</h1>
    }
}

#[function_component]
pub fn MovieCatagories() -> Html {
    let mov_cat_div = use_style!("
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        margin-bottom: 60px;
    ");

    html! {
        <>
            <div class={mov_cat_div}>
                <CatAction />
                <CatArnold />
                <CatBruceWillis />
                <CatCartoons />
                <CatComedy />
                <CatDrama />
                <CatDocumentary />
                <CatFantasy />
                <CatGodzilla />
                <CatHarryPotter />
                <CatIndianaJones />
                <CatJamesBond />
                <CatJohnWayne />
                <CatJohnWick />
                <CatJurassicPark />
                <CatKingsMen />
                <CatMenInBlack />
                <CatMisc />
                <CatNicolasCage />
                <CatPirates />
                <CatRiddick />
                <CatStarWars />
                <CatStarTrek />
                <CatSuperHeroes />
                <CatSciFi />
                <CatTomCruize />
                <CatTransformers />
                <CatTremors />
                <CatTheRock />
                <CatXMen />
            </div>
        </>
    }
}

// #[function_component]
// fn MyFooter() -> Html {
//     let footer_style = use_style!("height:75px;background-color:purple;");

//     let footer_p_style =
//         use_style!("padding-top:20px;text-align:center;color:white;font-size:16px;");

//     html! {
//         <footer class={ footer_style }>
//             <p class={ footer_p_style }>{ "FeO2" }</p>
//         </footer>
//     }
// }
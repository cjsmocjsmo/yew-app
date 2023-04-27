// use yew_router::prelude::*;
use yew::prelude::*;
// use yew::{function_component, html, Html};
// use stylist::yew::styled_component;
use stylist::yew::use_style;
// use yew_svg;

#[function_component]
pub fn MyTitle() -> Html {
    let h1style = use_style!("font-size:3em;padding:20px;color:#ca25d9;text-align:center;");

    html! {<h1 class={h1style} >{ "MTV" }</h1>}
}

#[function_component]
pub fn MovieSVG() -> Html {
    let asvg = use_style!("margin-bottom: 3em;margin-left: auto; margin-right:auto;");
    let svgstyle = use_style!("fill: white; width: 4em; text-align: center;");

    html! {
        <a class={asvg} href={"/"}>
            <svg
                class={svgstyle}
                viewBox="0 0 640 512">
                <path d="M64 64V352H576V64H64zM0 0H64 576h64V64 352v64H576 64 0V352 64 0zM128 448H512h32v64H512 128 96V448h32z"/>
            </svg>
        </a>

    }
}

#[function_component]
fn ImageGroupOne() -> Html {
    let igo = use_style!(
        "display:flex;flex-direction:row;flex-wrap:wrap;
        justify-content:center;align-items:center;margin-bottom:30px;"
    );

    let image_css = use_style!(
        "display:block;margin-top:10px;margin-right:auto;
        margin-left:auto;width:200px;border-radius:8px;"
    );

    let image_css2 = use_style!(
        "display:block;margin-top:10px;margin-right:auto;
         margin-left:auto;width:200px;border-radius:8px;"
    );

    html! {
        <div class={ igo }>
            <img class={ image_css }
                src="https://www.nasa.gov/sites/default/files/styles/full_width_feature/public/thumbnails/image/pia22486-main.jpg"
                alt="video thumbnail" />

            <img class={ image_css2 }
                src="https://www.nasa.gov/sites/default/files/styles/full_width_feature/public/thumbnails/image/pia22486-main.jpg"
                alt="video thumbnail" />

        </div>
    }
}

#[function_component]
fn MyFooter() -> Html {
    let footer_style = use_style!("height:75px;background-color:purple;");

    let footer_p_style =
        use_style!("padding-top:20px;text-align:center;color:white;font-size:16px;");

    html! {
        <footer class={ footer_style }>
            <p class={ footer_p_style }>{ "FeO2" }</p>
        </footer>
    }
}

#[function_component]
pub fn PlayerControls() -> Html {
    let player_control_div = use_style!(
        "display:flex;flex-direction:row;flex-grow:1;width:100%;justify-content:center;align-items:center;"
    );

    let button_div = use_style!("margin:4em;");

    let svg_style = use_style!("fill: white;background-color:#023b05;width: 2.75em;padding:1em;");
    html!(
        <div class={player_control_div}>
            <div class={button_div.clone()}>
                <button id="Previous">
                    <svg
                        class={svg_style.clone()}
                        viewBox="0 0 320 512">
                        <path d="M267.5 440.6c9.5 7.9 22.8 9.7 34.1 4.4s18.4-16.6 18.4-29V96c0-12.4-7.2-23.7-18.4-29s-24.5-3.6-34.1 4.4l-192 160L64 241V96c0-17.7-14.3-32-32-32S0 78.3 0 96V416c0 17.7 14.3 32 32 32s32-14.3 32-32V271l11.5 9.6 192 160z"></path>
                    </svg>
                </button>
            </div>

            <div class={button_div.clone()}>
                <button id="Stop">
                    <svg
                        class={svg_style.clone()}
                        viewBox="0 0 320 512">
                        <path d="M48 64C21.5 64 0 85.5 0 112V400c0 26.5 21.5 48 48 48H80c26.5 0 48-21.5 48-48V112c0-26.5-21.5-48-48-48H48zm192 0c-26.5 0-48 21.5-48 48V400c0 26.5 21.5 48 48 48h32c26.5 0 48-21.5 48-48V112c0-26.5-21.5-48-48-48H240z"/>
                    </svg>
                </button>
            </div>

            <div class={button_div.clone()}>
                <button id="Next">
                    <svg
                        class={svg_style.clone()}
                        viewBox="0 0 320 512">
                        <path d="M52.5 440.6c-9.5 7.9-22.8 9.7-34.1 4.4S0 428.4 0 416V96C0 83.6 7.2 72.3 18.4 67s24.5-3.6 34.1 4.4l192 160L256 241V96c0-17.7 14.3-32 32-32s32 14.3 32 32V416c0 17.7-14.3 32-32 32s-32-14.3-32-32V271l-11.5 9.6-192 160z"/>
                    </svg>
                </button>
            </div>
        </div>
    )
}

#[function_component]
pub fn ImageGroupTwo() -> Html {
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
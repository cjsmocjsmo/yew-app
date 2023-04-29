
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
    let img_style = use_style!("
        display: block;
        margin-top: 10px;
        margin-right: auto;
        margin-left: auto;
        width: 200px;
        border-radius: 8px;
    ");
    
    let img_src = String::from(
        "https://www.nasa.gov/sites/default/files/styles/full_width_feature/public/thumbnails/image/pia22486-main.jpg"
    );

    html!(
        <>
            <div class={ img_div }>

                <img class={img_style.clone()} src={img_src.clone()} alt="video thumbnail" />

                <img class={img_style.clone()} src={img_src.clone()} alt="video thumbnail" />

                <img class={img_style.clone()} src={img_src.clone()} alt="video thumbnail" />

            </div>
        </>
    )
}


#[function_component]
pub fn JohnWaynePage() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Home));
    html!(
        <>
            <h1 style="text-align:center;color:white">{"John Wayne Page"}</h1>
            <button {onclick} style="padding:8px;">{"Home"}</button>
            <ImageGroupB />

        </>
    )
}
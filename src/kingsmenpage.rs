
use yew_router::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn ImageGroupB() -> Html {
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
pub fn KingsMenPage() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Home));
    html!(
        <>
            <h1 style="text-align:center;color:white">{"Kings Men Page"}</h1>
            <button {onclick} style="padding:8px;">{"Home"}</button>
            <ImageGroupB />

        </>
    )
}
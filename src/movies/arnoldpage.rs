use yew_router::prelude::*;
use yew::prelude::*;




#[function_component]
pub fn ArnoldPage() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Home));
    html!(
        <>
            <h1 style="color:white">{"Arnold Page"}</h1>
            <button {onclick}>{"Home"}</button>
            <crate::comps::mainpage_comps::ImageGroupTwo />
            <crate::comps::mainpage_comps::PlayerControls />
        </>
    )
}
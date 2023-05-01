use yew_router::prelude::*;
use yew::prelude::*;
use stylist::yew::use_style;

#[function_component]
pub fn MusicMainPage() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&crate::Route::Home));
    
    let cat_btn_grp = use_style!("
        display: flex;
        flex-direction: row;
        flex-wrap: nowrap;
        justify-content: space-evenly;
        align-items: center;
    ");
    
    html!(
        <>

            <div class={cat_btn_grp}>
                <crate::comps::mainpage_comps::MovieSVG />
                <crate::comps::mainpage_comps::TVShowsSVG />
            </div>
            <h1 style="text-align:center;color:#ebb917">{"MUSIC MAIN Page"}</h1>
            <button {onclick}>{"Home"}</button>
            
        </>
    )
}
use stylist::yew::use_style;
use yew::prelude::*;

#[function_component]
pub fn ProdigyImgComp() -> Html {
    let b64_style = use_style!("
        width: 200px;
    ");

    html!(
        <>
        <img 
            class={b64_style}
            src="data:image/avif;base64, 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
            6tqHsUUZgYR9ycyA" 
            />
        </>
    )
}

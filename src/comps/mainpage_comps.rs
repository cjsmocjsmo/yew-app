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
        fill: white;
        width: 4em;
        text-align: center;
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
    // let asvg = use_style!("
    //     margin-bottom: 3em;
    //     margin-left: auto;
    //     margin-right: auto;
    // ");
    let svgstyle = use_style!("
        fill: white;
        width: 4em;
        text-align: center;
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
fn ImageGroupOne() -> Html {
    let igo = use_style!("
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        margin-bottom: 30px;
    ");

    let image_css = use_style!("
        display: block;
        margin-top: 10px;
        margin-right: auto;
        margin-left: auto;
        width: 200px;
        border-radius: 8px;
    ");

    let image_css2 = use_style!("
        display: block;
        margin-top: 10px;
        margin-right: auto;
        margin-left: auto;
        width: 200px;
        border-radius: 8px;
    ");

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

    let svg_style = use_style!("
        fill: white;
        background-color: #023b05;
        width: 2.75em;
        padding: 1em;
    ");

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
    let b64_style = use_style!("
        width: 250px;
        border-radius: 8px;
    ");

    html!(
        <>
            <div style="display:flex;flex-direction:row;flex-wrap:wrap;justify-content:center;align-items:center;margin-bottom:60px;">

                <img style="display:block;margin-top:10px;margin-right:auto;margin-left:auto;width:200px;border-radius:8px;"
                src="https://www.nasa.gov/sites/default/files/styles/full_width_feature/public/thumbnails/image/pia22486-main.jpg" alt="video thumbnail" />

                <img style="display:block;margin-top:10px;margin-right:auto;margin-left:auto;width:200px;border-radius:8px;"
                src="https://www.nasa.gov/sites/default/files/styles/full_width_feature/public/thumbnails/image/pia22486-main.jpg" alt="video thumbnail" />

                <img style="display:block;margin-top:10px;margin-right:auto;margin-left:auto;width:200px;border-radius:8px;"
                src="https://www.nasa.gov/sites/default/files/styles/full_width_feature/public/thumbnails/image/pia22486-main.jpg" alt="video thumbnail" />

                <img id="prehistoric_planet" class={b64_style} src="
                    data:image/avif;base64, AAAAHGZ0eXBhdmlmAAAAAGF2aWZtaWYxbWlhZgAAAOptZXRhAAAAAAAAACFoZGxyAAAAAA
                    AAAABwaWN0AAAAAAAAAAAAAAAAAAAAAA5waXRtAAAAAAABAAAAImlsb2MAAAAAREAAAQAB
                    AAAAAAEOAAEAAAAAAABQkgAAACNpaW5mAAAAAAABAAAAFWluZmUCAAAAAAEAAGF2MDEAAA
                    AAamlwcnAAAABLaXBjbwAAABNjb2xybmNseAACAAIABoAAAAAMYXYxQ4EEDAAAAAAUaXNw
                    ZQAAAAAAAAIVAAADIAAAABBwaXhpAAAAAAMICAgAAAAXaXBtYQAAAAAAAAABAAEEgYIDhA
                    AAUJptZGF0EgAKBxkmYUx9oQgyg6EBEfwCSUDhMUgEREQARL2rZkISjfQC3yd92YKH5DyR
                    E9YqvONiIh/VSGheiyzGLjyO5r8/gCKNTsoyEu5Lb1BUuDpHZVAE0gN6Nu9AafXySlIL0G
                    CJsDpampK7Cnki5G6BSsPpXS9psYLyqvdL8DRQtMjAJskjdFKhK8qaL0YcI+Kdn2z5pjIJ
                    b48KOL9n2lfdUjbqq3pPIVy7/JGKs/evSUxxltcWnVE768HeKdW6q8GMTrpfSRB85FDLIG
                    HDrdQn29e+AKPmd89R0R/PrLH7WVUUCavxGLxay/6qknaesHW4PPOccFPmONYWsB1/3IS7
                    wqVLdaMePVZULga/EzoF3Dgbo70jJV3H4QJG1p4AS+B49CkREzalSHpJcZE52OG9ifwaMy
                    rgw8dqbkPgnL9A3fCs4XLvs1cjvgzavV3sq2Y02KPyBkKSEDQiNLLWWxZfYaIrRF6mgztd
                    BTRSIZ6IsC9Y5v0JlzDtE1+txb1KTh+Kmb4vP1fv4YI0VQGvjLjhKPWkkTVvcyrgEZsQQK
                    MVlDjMYEbEh8tYTXGF1ITdBJoXJmIA6hQRTR3TogFwDRLIrqpclRFoeWW2ZggeHrGuc62b
                    12BdI/Nnu/CR8c3aOUdoO6iAEvj9OM8JYXhkhAShHIc7g+sUiy80n3QERWokH2LTnDIedJ
                    2kCuVNM4YeiCONlM871igeuMSKmQsbAeryMeUlnUITp+3yVHp1gpFauOmBlFdA8WVQ1GYd
                    aQBehbqk1yWCh4Z5v37K2ZQn+Sk4sRPWNnxFfkEO9D/QZPgkBjTJqTOR9V5bzkeOxMbZc1
                    NIc9C+4tuQiYxIcWFlnf10xRUz3vdHbp5+t0vW0N+pc/T6qlsydisUu4wuL1uSRhkJstgc
                    lNPDevdJM/CJJLLXP0FRx7rbyqPwUX/kaxaz7N08/5HMgn9n5HLOY570xM6iUGjihi8cur
                    RN9bAJbxekWt5ydktOmY2kw8v6cGPgkoCrxE6nhtWUKCEeq8r7tDN6j5x2tRCygmn++VmP
                    44WLAY8FfuFyrylrS8BIb4Lsyt50ZnuFonPdJgP1lKMVXfLC+aKOqgVLV0jaGsIUVm5mj/
                    3TUsTTD5j1qrJh57UKBiWpuvfK+Wp1vqbq/zbVMa9nmGhTZxtQtUDeJw1pVXkDqGfEThyz
                    wjxnA2MFA78rOGLCmaED7Wpneu/L1haumxmDO8nLuZXtLUIkmgpg9l7UdPio1ocqIrZzmu
                    kqIyQz52DnDpbN/H7lSDLCg94uPhz7T88O6shlNNqqtd0HWNUPTFwClZ7LFhu9UfxPQPta
                    TAqgNE6NtijgunZe/VWyeiOaGGZdyIi/NR91kbGakXyYQvbhJZpiX6Xo6naGNWJF4K87qI
                    yAlYJa+Ctyda7ulVqtXNrcyGOKtwI5J0uPZ7Kmn56MxV7BJlKj9gh3Ttds4IKvyp3L9vNb
                    zdl7oSSD4EvBnvUHn00KeufMHtuXV1tSmHYO+L+m7QFd+69jDdmgkSs62z8svMEJ9ydRaH
                    GS120WIKhPTajvK2EuYcvMKu+J9/KSO4vm2UenG8QkfltzvFJF1qd2AIYsC+A6D6tBOzpA
                    84meftBn2l5NSVOmzgCUGgcQt0UxneeGZDEg8pjIwYiBeBzWKqBSM5qnKYmEuBm8dCRk6I
                    fJlZUToCcziH4qK9fllz71sXiYPMOqO8me66LaqKbLzjRqbUHCzSZgvLcFAGXoUocK1dd7
                    KB3+KPjWTdzyp0SQaE7kt+AyNXA1FpmZtdtN0b+KWhC0hyRyExD9/sa9B3Ky8Jx7LDhiLl
                    O5Cz9G5D+1SN4DHmdk512LTXum+9SRFzcmlu5WitxH66isIsmj5CoqmxGrjEXAjCWfGMDo
                    H69dyBXg3OdXkXmRp8MAcrhLutESsbznJjLDS44SWrrLlJAGkkkIBVCWym2djrvpLfaA/9
                    N1d2x1RTYTKcyealkvqvTiazXXwbYPhr314M1XiJ9bPffftWAXCEKVcV3ZAsGLPkzb6dae
                    5wrFcdzdyDDbexRqzZ6fLXE2INQNDxpnZmRp1p1u/giEo0T8v2lZAuaWoHCmbc6XSEfnTH
                    F0CKRhrQpWbXS/VvwRYsGj5yophnCgQwiCMVTfYboKbmyqb1bo5uG2XXkVFi+IqZ5RIum1
                    SyN94aDyEDh7JnqNsBECou5LcL+HqmRqTEow+TnTzoYtVxOUC9caESCsAEKm6+j1A/gnwX
                    laQX66GH28jBI9xkoLxRmNIhLocw7dTdU1Wkpz2e65fd3LIvL21W5Azn2gut6/OuC+F5SD
                    zFtRvEU2LTNkpGRZTWn+bzVL+LDj0KnMQQxahmlF4PpZambudF9kWDhNwybuh355q9Govr
                    PyFJcN4MMkViiDI9Gixvc04b+OpfHbcZlGh8zdErxXN4wGdjNb9YWdRMxGK17RNC3vzrD8
                    hLySrdV/oQ3no7yNSvolz/LrqiSzWWQd5r7ALtxEop60DrzwQMjXY5+OseO6yxiKylcOYP
                    ftOFLC/AsN5buv80g4A+TeOB7h8NGbKy2/MA334a+5DryCI2i05NuvRhr0E2I/h1BpIvDX
                    0HYpu/1iMapCniX8ElZmCPgig2d7XEBdedbPGolxC5gw1t3P3vvkxFajepL3r6KKfKN+NF
                    mNzlel/N8+UUPcXESvFXFkhjY/5OUrl28zVMGNxHXW6UpPxxKI9s6/E5tx6h3Jl/cqt9PQ
                    xFI6CzAPjHMkhT9gPYlIqPlgeq5fwu/rpI9vupqquufYmFLOlQpWQo0ghg9ql1AuUJnjhU
                    ttoKvNLUYvcSO2u8qJ6mvt6RnREgp1ebndUDXdGC1RmIKJfaqdxRSh8agKBesKwfFp+gi+
                    XxbabruDyZKTQ/BUn2ywr5T0BziKJwKg3GKKYrAdtXIQzX2MV+fiDRH8/jGCavbl4nBcnq
                    qDNUeA1KI9k5KMc5hOx6UGWJ47hyKYVyRdjCkcVVmT0djdMZ4UelzVGYleTqrpblv+kZk9
                    JA7hduAOqQvPzchQe3HUlEZkqv5hmSasKpu7v7EDQYLjbOGMy71GO+KZXfOghVWmTDRi9y
                    i/EOZOC0y1uDa3pFXosle7aMw/YjrQo2wm9mcY4xMXFFa6K+Vc3uyxuXLfzbpUvHPoPeWJ
                    RVbBPwh1dp5GYIz7pXmWxmUwdh4H7FkDio4R9hdo23eLJbuSS9li9loWqtFpdxHLGwh7HV
                    hABlOFx2LoXJ5+ZZpjlnY9nrA1ZYkcLO3NzsHmnFgjARHzlI3pkU9ZFqkTggP7xfle6fax
                    wSIqmUpXl6v9iLJzodthOXe6QJeMXqemB5KiZsJGkW+jL5ptptStZ2Ov7FoDkQczvHYZPI
                    AEPYLHohADkYAZ+3bpn8713kd5qAYp4yVuFHO65Ijcy38LZ7a8Dbhch2JQJx2B1IUmVqNI
                    iav/VJ1ReaST7UCJm26QgufwW1CXT/y0tSdBArXSw6YgDVdlHMOoDoSEiTmb3hSS4yYV+w
                    8UodGp/Sy5LwTucB/NLhruVyfyM1F4T0VbIo1acX0f0GjqdVG3T3g2MhrX5E3gl3tkxkqm
                    Ah/VFTx+ib/rKfXtv5TY8jJhj18c3GPPli/qj2so6n5eIhCnHwY/IlQJWrbjKm6wxxne6y
                    l98jxoiUvQJeyuK/elNvWpIMs/kJfsUDdKl5rXyyrNXOk7ajn13a6Ah+21Fm66kGS49zJ2
                    GpsoKkML7SEoDQr4NWtr75+jY1dZOjYn0TRev+xu/XuAuJmELwEzsPBKS4R/Klas19h5C0
                    Y6dLlnVeUNTZXxwOQim+BIX0hYN0VAuGeSSNg3ErWL08dbxTwTtB9vRfdUdy6qtGne5kBQ
                    SQldbQ9b3gyGMMDiMwjzltiLiJLhzYzOKEjsmwrQ05oJzpFNJfAzh2QE8tlxmkKFXiliMI
                    ELXp3nRQiGMlb/tK8oatmQxj2g4E2td/H5EgvOzGY13ijY3sjyO5IoN7wiNer2bDWILYpX
                    2hYCdCeAXCNPnDsCdtC6qBud5EfLyuE84o14YwyRSCBnZbhF/SLi+vXrBsV2QPpJnZOtoE
                    FG33EXvpXgyzFpiGIT/kGoLKTbgbk1JRTU3x0iH/mAA9x8kDuZgESoVwUnZmluuASmiU/F
                    zZ0klXodjz7yvU7AJh/7g2LB1Q0grYQoDvOPD1cUcwpbEz65VEXUAVzOF6mWonOXbxlvCT
                    f63xgXHaBlp4lP6zmRpM2wRGj5478sWmjnofeJ85wir2MuGz7wFdX8q/ayOMdRbatK4gDF
                    A2nqaR1uzk4XXz1TWlCs/X+QmDlsl6mGo10x4exHgoZ6bj5oC2axm4V1F2O/+ssuJ2we3w
                    ByJpjj5fGMtYGD3C49jpbxK4+5PHa16GeCiqjOZS8zCW4cr9dNlB6KcmzoAFi5vGFFf/iV
                    hN6IphLVRZTYsQnmj4Tb1RIQxXJ7Hiw2SFt8BDbV5UUdwNxEwwTGB4yax627NNzgtFlO/3
                    rsMs7GRSUo6ThXyMsnIWytb4Hx9mddZyFWxz2A4jSI3JpU0/iM9jqAPvWlbrkaZQsTcBmM
                    Kn8isQ/ahEvdeQUO1vbv75aiIJd1MEq5RRO+DCYFvHi7Wazgy0gVWO3mJQ+HHfktlfUrx8
                    e5NdHhCF2OtPXO+QfC8t01iEnsBs6pzI28pbko7rKWD4VZApvU1m6I970sgXTppwJNYCOU
                    zQhB8DTg0CU8Qbkgu9CfxSXdFZSRJWPDVCuPSsM2IQRQ/HSkdhz2HfGMKqOukoAJZZDK9J
                    MfjUV35O5F1R1mFf3M47o57khbrFcztMlixKEBbYplCM0P4ue1LNe8XPuU5ymYXWsrMLIL
                    Ehk64NKsCsQi/TC1ZIwvo1mgBU5YsUH6QLyCh6L2iphoJXT4a8voXRVfqxSOPaZBcKUwN6
                    WqbrO7muqxj8ayE+9j1X6qff50enhDOINKZHVcEVMWYI720wjthfGTsjtw8SyGd4d+MPms
                    36UDPpK28swdM6Xb11HgOOT2+e8R+R/2Bz/ZCieAQFiTt//K/ydRT7hmSHh2bUOYEpBfmn
                    vioWaSe+3gACHlpKXToaoMr1HogfU+KQpa/osceNLP5xWaxZ9LafSr7sGVq3NVV1DpW8hG
                    yDSuMOjUjC/3eBEKoF2hqlzV8bZA2P1LS9yz5weSq7PPDD8vK10LrKX1EfdxKqJpXVb+pT
                    H/1DPFO34KJgWLPFLumQBLGqLDru8NqfaF5KJXNSLax5jJ/XRNBhBqzQZGPKMPlrsmvlwL
                    68thpkxftg5afI78h0F20dATVOlEF5g0llKY5XnDOsDtzkwsQ4PJDKWc3OjvAli/W0IvPd
                    SNrLD3aOHHvzG8YDK17JAcpgvq/8vrGG8STJw6utUwU2vFCf9Ad5D9AEc+5wtzVooW23/n
                    CKhWYO7nRHgDLeQ9R6ZltZH+JqSt0aA1FurT8tdXpNWNk+bL/r0Id0WbgvK+jhLOFlnvaB
                    X9rD7Df0iH3rT7PgW8A3BNHg19owXMwTOZlcpD1sxUkpAAZgJijQfoe/pMhzBTdGndSO5I
                    VzriPakZAK7mAMEVI/JcEsJ73nDWi+AYm/13D1QlAsuRQcqbGS62qtWycoSWgzkzbJdusa
                    xmr+r8dfwRkuxB/lhMkMSo0juXAPFR3TwQpFN+23jK5teF7zVjgIs1bUTB4zRxpNJctPie
                    ffqD5EDWJkLK6DrzJjxyEWqnxc0KqlIODHKSPgdQ84XrMsW+wiiP4EtRM9EeuIJa8P6ig/
                    U/xxg1m/fEGJKzeczjwVuvL4mqIQ1Ru0SOz+t/2t7KT097pIOsio37+DkDOn4Z+NBHcUTg
                    VCjizooCXFqBElfpXQ/z1CqKnFlUjpUAsPHeMveJmSdEaVeBezt2ClnUM4XmXdzGvAvhrD
                    iAsOIT57JmmsNqOTnyykhE/3mVx5Ju3uE9mSyP3nXbf3oLpQ64vnNIlEK6r+qWrTnalAul
                    4lOQw0y0qX9ZOvIqyOJheHQFeSN38WYENQsoOOgzl0y/qNarECVVW24adsCCBs5O6dxCvZ
                    RVGeppuQcOpadJM2MnRqbupsoeex5+JHD0VzoNVvWKjK+61LG7BEpUbQcz7muQWsocjuLJ
                    PjUw3ExLPwhth1FaU5e7M7fTuMC1C3mpH0U23EGY03b1u/8giKZ7NKblHtkZ1D2uMdcaaJ
                    vMe2RLnbtWV5p2Mx1iAyjGq3t3NahCDGz9Boc/5ETvdIV71rermPag1B7+R+NH5q+B3sXg
                    xBDnii74YpkBQQaoOlIW8rn6fM16+gjKrszLBXbVU3TjqYD75O+OoifCAhwo0FZvizq9xJ
                    GgKX36SiJerxHvSENbOjVqtEhhNuHe4rn/b3MHlwnnnmwjs+d4TH/yiD9CJhNQWHc9tdZY
                    mSLp2bK5GkMmHpxLxQd8m2tqOziEXGHB3XUhgI09gnuKWFlFV9fUZj8UNT16AxA7UrOatj
                    XCOUonzmSCBYsSajN6RVKzTER70Xrl12Kz7ogSCf+OocXEDJfNciebzSmGtyJPemedTtrJ
                    nr9erKQIfDsgA14IwvfbwajMziUciuk7LMJTmWAIjt2E0Wq2jsHsDqazqGIVjoUGYE4qZK
                    9C1oW8tFMvsNLFJAHOCZwqMo4P5PsMbfDP2qZ/c/LnepAd6jxXACHl/5Lp9TvHv1kvj6ad
                    ufi2D/SFH3qO5guTGYo7/bZnI965E69y1abqxwKwtXbO1BSUhsAe3PtDjk7WVxj6WB7pPH
                    bgX0wVI3N/EnnHZWFjmrkz7NnMj1+AtwEByjLrhqTYlXSeZX+N5XseD5WsfOrX+5bkzGXA
                    GHeyvMRYVL7YgGCp3f8EL/AC2deNvWmaRB5BShYWROHdKeoH1eZoNrWEqIHTpPEAT2qRDL
                    NlHVP8Q7gEuleu6W2J+e4w+5NEn30QbG3IjHgwffYJp8psGciYZVJdMzTCk+fLfpBdLKMh
                    wEfAaDQv04sBjGxvbYMDEUqWj9RYNT7gWj3jgK8I/F8XtQxtpljk8kL5loCDGfBFUmCQ3p
                    +5f+zvMGiNQPhkpq2UV9qre3Es4vPYlE9NnfMlR1d24ZrsFD8DxyfKNPCGXDTfNzP0hVVo
                    mlNMIOx+rkCVADh9abxsQY8zt/amdEfin7rrHGS40s1mw15vWoz+AevWBLiYb9YTPhyclt
                    Fjosg3v7XcUDr+sDddSD+qflUSHRN6xHSH0IXFFULUo81Xrf/uXMffmN0YYAm7sEXOxWea
                    vwQaU+RelWPt1QfhJSHbQPiZLoiQPrPIsaa0ICyceAf7rPzvjATpeobmmsNK3PIlKXZDdc
                    cJN5lNvptCgd06NjESXPzzPZS+heeLPapGkL0owVGqbGv41yqBayPiTZ2kDwcRfsxqmOes
                    F3W4WxisH3POcNY4nrCC4AgjI1hVAe3kEJ/a54olLhyoHOFsNgHS94p0w9EcpHOh2XBSoh
                    3F6c4yEAHiPk0162dDOCZA19nDSBTXUOGLq4uJD1QiqtiAPosMXAqVDpKglnjKCrLJ90Og
                    cNYR7ppcBOb3RczyjaRctk1j8Lcm8+e/plBUsBek4s/vQgu37u96WvwyNlINz6XaquS+g7
                    4vMOacxpwkgKUrgaLIBJMwpQZCUFBGQxaq/C9hEQ3F8G/eIfyodulTecdqY4ymMsnpu8hP
                    ondxdii3voF/APA+cS4QEycP2qOTjqSZP/Nr/rfUkiqJHERvTq0PuEjZbRt6xiQ8/3inw9
                    qIC6qqlx2grJrkr71j/f0LumuWfWMK/PEV0L4xSiPLcC53PnNUskN7i5XO7z2Mdatn+iZG
                    6QsgtJgkcWRCcXstdAmZlRr/OddY65RzTvjJHWh2HqXSvjMlYlatKLrboWtgxSWuce8iBP
                    QsEuPabjvq2goJpqSi4pZcYkevqyCpoKAMf9lX46A3Y5lWaXZaedLktcQpNcBRqThU59pB
                    gTwTfQLDivNjNlX0MM8Nmioc2KT7sPC9KL8Dn53cfP6/G8VTLUMEX5TjDt+KcugC1uKScY
                    tDtUpvbB5RtGMdibgQKj0soQFzvVvL+IDrwEVmcz3nO8IQhpks4ETq3QCuOLHbSuxC4eBW
                    5p/H4tGxDn12zeGmsHb58ifeafFzz5YNTMgtwxa4+6EwzmL8ewJ31q0EyinlQTu/gtCrC6
                    o07LiPmdatf9wbwHC9U+DthSyjVBE/MNKuQmj/DzHSizTY7htlkBhKCRYFrapaT3Q1mU8w
                    UsQgfbLazrmBVx7xTvAwk//lk1cI/xxJduiMyrrAuxm5icEWvkFsRqBQgFFLI9CfbXPpEx
                    qYlUVfODziK8GZns7mk/ghzH1/Yjh+00f1EElDi1lA8Q9V4IGPbKrXoTgiOUDEkYYy37+L
                    T0Tm0MO6ooVO4JvbT5F8UW40cOqHkcKN7634PtQ8Enh065Jk9WY8gaLxfpvLKjfCT+VZL1
                    1ba+Pj3MD8UV8MuPgR2FIK6yn/oUAJZ0+PMdPQmNo/rsJRxK/xYwlSnIVJoA+3AJz1wOVE
                    j0el6zh2k6es38o5a8+TBAD9Vz+KjbriId2uM7xJZ7SlpbfRVfEplibuexAmSVKFn3lFYJ
                    zihLmo6X6BFEpiTNWjAAr7esNYUEtl4ka9z5XvBNmg0qXZ13I6dBsys5bPoI3tcW3u3LA6
                    Ucw76WI6BD0KazdyOw1bP+Q8Y/kPPKSCfk3OLZTL6JPcx1MsfI1zF8JJ4mC/AZIEHmKDuY
                    jn8O2KbGJk1nQP2gEc0mo9YUX3JNke8vshfwoshDhOjPau6UCe2A8BjVZ9VHXOIgZcagaY
                    iDgNUlwymVzunjLNEJT+nc6+EB05RVO7yop8/MEZ53Tpj7llwtuQv7h5u/wT6bS/1kYLBp
                    7+EmeZYwMri7ffFvS5X2mc8XxQm750YICoQlBDN7ImO6dp7yBbZBc5T1Cs+rWRGNMOf9kV
                    5v3bsU5NVCIVvMGnKjYI+X3UbNmUTgsTGwvwt/Fi3g70J/UfO/cMBB3KoGVCf1mzXW+P56
                    ggSnAvHXT2chnsLBMySlmb9NBzSx2VOarupOkcLCxlAjwFsxDGo3PGF1ypu2BBYqh3FpFt
                    gT3Xb/JnapssJxp7z6dfrv31kbqY5UIKPnLWYdiOBG7+bl87C3RgWEbBFgasbF6lun2noN
                    d9QHle3KAjoEFwFk9QGCDbgHFhpRXwtLn4G/3LG6uras05y+oyqwyIS9qlb3M3wVBkvF4z
                    Ljo/xhpqaGjkUtljrmYhf8fm9NhLOeiomu3ToQ/BYUSfOdL1lT41FRfpPYh79D6yx9XKrE
                    ruWVzFH+qCbfJNK+uafI0s+rIjidBvNx3S3Zk1JcHNiegcHa2TJ3E++ikbT0m9haTUgMCg
                    ZHcNN525DbqXPGjDKQhCnxJZPFGkgkFhOW5yROTc428epWao6IwTzc1TphrhWEM799rsPl
                    /rrWuPe1SNlNfkMQyMPf769QWcOr9jIkJx+SJTqkq4X7ngYEHdB61exKfawpOKJzPkTmMa
                    xVBhbPSXTMLMmJpAl6m75sj5vXg7udfd5SZVuSzut0/tGLNoN1Q5OUP9H4tyld3tTjMKmB
                    Xe8XeHrbMRO3eZQhVvhKvXpvasFaxpOdqWuDhXylmfKZBeVb5J030nMKydWmbVa7w6f8Oa
                    gBUmBhf3h1tzq9DQTAofpTT7HQU7ZTj/WFRyQ5RICDqWUyTtzJHmHuXqrwR8wzEiIJ3m7/
                    3iTcxfrvtagDRDWm6KrUl1er+76NHbE7nsSHUF+/T4DKQ9+m50JF/3STXDW5XDpdaqTZJj
                    9cohda02f3ve2YVBzTeuOYwBFQ/Zx9rqoDBBZAnRQY4V4wuHEILscJ6zlc+jBXlPZ7cqsq
                    kmNgpqSMaeX4OLGIDWhxk9dYS5NTso4lFsMoP5qh50DoBy/ecztdWdFoof9NxGSgKC9ndM
                    djOk1Qeji6d8CWteNC/llo+FBl6Ep8ZiGsNte/kVQR10cAA+KB4fCMwRU8yXQTAb6+dvjR
                    5Wxq2B92YMoibEuEqy9N5NYg6WdX1E2vVKktctRtgUn8FSISF2HithUAAVP31lI5ebgAgx
                    kHc8WHbxQIvPi1MHqyjMBTsqNmbQU/R7G9tecGWltK0ZdjYUcI0JReqV9B3JRqnnfcZ58L
                    Hpw4i3SF8xwkdsAWINPP5w74OdT3exkbrKfLeeF7ct6YDBkQfSrlKeO1yJ/DWE6gju4Pkw
                    Ej7ROOVXgUebNX5gSbzmfCEFpDajRvxFOZeGSbNPlvlBniZ81wYlb68bEFRTvyMRpbEkiz
                    TXltkBOpFVx4UbmLV/GloPhKrpl5ZeHb4Zna9gc3TxwRyAKwRPiQhRt+Ri5GQI8gkIGO5S
                    K+i4K42qgxbyOwmm5M8vIF6fZxYc5Yo42z866HFqdZOZwFv2Z2jTstGc7keSEdiv5QTSGJ
                    1ATr+eOuokzykvVDyUzKyiBNzctUnUpuiJ7aeeydse+Lp7FAFPBrG11HWM+dIi/Pv0Eflo
                    9uEzXNeK/vWg6Vh94C71dqwt77T6S/S2n5GJlMuf0NQJLWHWA/APaP3EfHo/WaXjfRILAT
                    pXpMEviZHunpKXQYfA/w24ftAlART/bnsxiI0yfZULXWLWO4BsvgiKHPPWqEn4oJH66eLA
                    if0WjdKCWJxGyHjvuHCavuOj14Sm20j04WAO8ZuKFGwZohhFDQZOxi9fIAwQQC8zTLr0YL
                    6A+nUM6XWngXDr8Vfpq88Ks937ijfQwcGwwa0J3ZWT2RLvhgdERuFFMXHmuZWxzxkS32ct
                    +eOfvNFcSovNEEjshcsvse8zpBNe3yR9IlTAh3HvYaj06tBJec4PeL0BtDJKXwmZAgpFkV
                    03uEqg/5JINRkCDIRSLKZW5GVKeZUKbrgTIY95Htfpb6/23iN/4HtB5S4N0jqHi4z9PhcN
                    yZ8QRiQTSMVNq/H8ds28jcqxT3sAkQOnPROtVeCnhu5c4EbtP1sBlDxSaPc8Tst0/ygBDZ
                    inxF9i6Qc+Of1nZxMYA3eUun+qp6xZkEb0duPAS+sc8nWAfB5kpcOzrsfYcEJTHnDR05dx
                    2v/JCiGyMlidq7e0yQaUygEJmKYdXOOaL0tWKY2BTkJkeDoElZ4jvtBRGe9UA8194cmASn
                    okAKvI7+EF0CUepAxPAT/gbQszEgfC1Tcsac6okKKFHZJDE9tehxg5aDsN2JXfp/bgn2QV
                    Kq/algALF7Hozk0lyzQ1sB57Xj0bTyFdsXRhCAm/G0Wn9xRLBmakkbSgtkyxA89S5Nudf3
                    HiTia4GNjeALda7X6YanEvm8GqFQtPw4jY04sgMSaStb9HMug0+DRTiNeyKCrS+F/b2/ZL
                    PxAM4pqa509NhD/VJt2IpGoDMx6fWt3fZIaSMcfCWWOvEu8YUOUNgp5OYJfskdY2NrCUGa
                    EjLyUDGM7ZOzkLJjErE64gSsGoneMO6b/uis20xpYqeD6KPQqeMtAjBMS5WtwZ03rFx/hW
                    sXNOBU9cLjb7PEHkjbr9ZLtIfIb5+9aOXJdyB8RiYNcklLGZYBngswNfu9TaMyokIqkqre
                    IoDqpYoBkYNlLLD1PuNqwborhGb+2LA4zgWsE24mv5RxBvU2OtqNODslStsftIbXYtiMgJ
                    Cvrpg6Z4HJnbmU7EnUQ5GSWxdNpUVck7UTjCRsqQOfE+4yHpKaibuO1BQOVPLTtOJrifDX
                    lJQNvd7mHC9IBBEJ1Or/JHgFn5GTVvwC00Wfir1FGJpOi09fgnjjmXMY3s17HqHbyNAmDg
                    yVXRni7tTqYTL+H3gV9w74z4zqCjzsYIe2m1Rudvofz67IQHH2faR85rtY1YyhNvVxHs2W
                    Hcor0dl8Feeko7da1BagtU3681ixtnZBqkKaV0u36ZNiHeesR5EH3/sxUXZ7Jvi41QaY3t
                    SmHXCH25KrvR8R6U7k+73GpMGRTKftitopmkgwy6pWmRJaDiURWcMQaQ8O3IBw3fNIspeP
                    fkFnkFI8pnAllCKSPJlzYf7Ym1UouRECRr3IsePKchSphDkEgYt9eJkD4SoBx84l1zBnAE
                    4p4qXZTiPsxFU/uiJwkw8HATXAplPVBzrStoutaFRPmadQi193igEVhbgjVqfNhpVUGoh/
                    bVsFjRigmlwhOQj9q4TFVTwe8f/BUqSkf75iSXitrWuUTpPsr9Eh01nAz7a0lDZOgxQYpf
                    vMjL3Anwg7velaPrGaFkHRD0HF9kn1KEKtQHYdAlUYx9X0IJR5yALXNGzt1OnIWkCsvw6C
                    cbmsQBLr7Zgl3R7T0/2c4HRHWECPWDu2+bqHhSihDVd0FO3Wckkarcx9VvL8coM4u9Am5P
                    iUjy7QTow0j/BPKT1ouGcv7My19Ai0huuoFawqLjTUJInugDK0bY1Kmvg6dszwZjEXuIXR
                    OiaQXTtDOceb6UcKcfJ3PhMIF09X0UsBkgMqaiST/aIaumX++M72ET+XgkQKjXpblh9Xnz
                    ZTLIYYXudYUyBnqw2++Fvu5Uvg1reWvnnMMk45h06bC5AEJEBbZ0btF6eB/JdDf1DymWBT
                    BPHREgUQX5mxbrpAWNZXxdYmzj1H/GdChyPSXmT20o68lpTz+Vc5SWG3m2envTX/aKQfTe
                    qrZBO6zi8rDfQvvam12gihlUgNZIaQTI1pIVc1B5GAb/4uBamxo1RoZ7sEEB6KCUG6lT4p
                    zKVzMDfi+gvUpeS4eVKPCpkChYXQ9vTDlJWvlpuA6Mgp9aG3iR6C3TChkskPOJ3nhdy/jk
                    ALY0lOkrVj/iv2AnJuwFMma9BnnaVgCrozRen0VOtoLT34Gp76Rjr8LfBRQjv0WzRSZbZj
                    bFDnwmpPsToqwYZbxUK6gIIrkJV2l2mLFdLgEP+Abj9i/jLqcieirE1NH3nAesd5Xpt0Cn
                    +yN8NKw3uo6mxdJ9vONDF02aTOoAW2ncK38zMTcznZ2ONyGzq52GWws3MgMY9ROXjoO8LO
                    FOG5ud+QX/ELhegH75th/AR9MuIjAACJF2V7YY5qOP4urO/eTzWPu6rZZgA7kLCBZPotB5
                    A2MzHPmpOMII5xJWsdg0V44FYD8GYnRAbhqc7BS4O+3PDKiqqg/6LCIvbXOZRwG72GLnyz
                    HPMxMeDF/kyqKInznjk/JqMt4dbKNwanjRZvZ3AMEK30U5q1Zi+FkCFqGVN7ellkfOGXpx
                    p7r6+bwhLlp6IRYKVla9UEao9JZBfn+wC2OyRk5tkUL8S9PqYVbfAUk19YJR91qxFoIvJ9
                    vgcuNjLsIvTNTIEEMkhsBx9YuNNm0BwMIb7t4jjm6hE9XJtC4Ff09ZRhLbZJkn4k2gEyBZ
                    jrdQbLvRSR229ZQ70GnGCkmx/Ml7YiwGCCSqlM/B2tIlUTsLutFE525SAAekU6KjEd77tB
                    3MA+gwdgmQjj/Wq+L/gsxhPpicbj3Le2yajYKLhsvB5SXG2aHtRIqyT/7OC/C99c8fpPYc
                    GIYszKkBmEk2NAut5flMAylBw1YMZ1DeXRmzvqB4G4kcOcKxYfgclEo6Ab9iatHLRkSFjP
                    ny70jD8orXeTCQfKi6AqApbcj9Tv3FOs88nxOI7Vh4rKM7ckdpwtar5WB7lHAi5buJ94Xe
                    L/PoPUv9ownfRE4pFvcsumO+sUu5WPYpXB7PMu9aPW/vk5J/ELGb4CAELtznTQz9NSleZ3
                    yYBeCUWPJFYTkogb1weZcC8H116kxZPnB3/envPXheY1VMZDPaK9Lsq9N2MQHq/XK1y3gV
                    YxojaZlsfCVY93qHsMlM9Zg2alkPBX4AqPBlzhFtoTqKFt21okHcsvNYECSjAaE1dpzHt5
                    WhBs56arAeFGH7X/hPtrMFCvt0QJsNOQhszGFd9swE3azj+I9I9s4EeKkVEsgAw3FPcf2v
                    rUsgJGitn2fowOI51NJchHukCGGNWXyypUdju0Gb6UxY1Dj/QvwCCl1Y5JGoTyP7y4ZH/e
                    rdmyFyLahuaAWA9rZaHi4HGARe99U4Wj7kKtJw6REbsy8piIH/OIhfLpmPyZm4X6SDSEhv
                    Dr5Um/wcHsmc6p+mrQSD0JZNtZHyQfZ6kckeBWIEBgEIAYEjkVAIlb/AI3RX69TWWBgCO0
                    IO2qps4G/M/SMbQ4ju5TEJIutvOBryCLRx5xtRP/b0TqQau7QPvp/fUCbzZtGkjl4u8dFC
                    0Y1DThPT2jAjHRQ6Vsk6bAt23Bjx2L66Bvt563qB47QswV7nAlhXMhxWni4SfCeMhvolwf
                    5AE/nSvZFjh4q4s2qJbZ1kIhme7Jc9uSz9Gh1xBbMOM4CazNivBxtRlXUGcBKcZBP5mlt9
                    EywDvKIX85/rg0w0BMknchD2m9/nNh9ej1NKRO2RfFPvqOokF0PEI+uU6fDNLZVJ6TLDVK
                    677x/IkrQaqOGL5+2BRmo3QovWQyKLDsDNDTOE6Bs2tzahXXiffEEwbecVBMfuS9i0IgqV
                    KFubF7kxKff7kkIBrdFeAzHiiLxaViiRzgIp+wdgRDrevlgm/b7rV0It/1gJQTBUgUWuH1
                    LYMQxMG9vY4UddWWeg7gqmWTOZJp+EC4oSnv8QF2Jbxq4nFN8Cm6PUGmZaYcLUKLFeoLJr
                    MKGSCbXq4kyszldHSguW0nQJoGG/pDEGL2RB6VnhG+N0mtoMgSl/msOU7/eMTtTNCCd4qh
                    FExCMBdXUzLUMiaL9OXqEQYEZOGKVx2mbLWur0Z5IZw10c0AhFGsj1Vw3xOQK1k8JfgXfB
                    PmJeJwdRv1EOSBzhg+ble+c5n4nxbg9KfsrvZUVlWATat7UwmGhYktazDSeBF2OY4YiDmi
                    TdXvNBqmqW+JMo7VBQTo8pX5eaN0LGqr/SBt8MAGlVFzY9tTjAvspBNoN73jY3EUTK5lrx
                    itMShRMANVDJgn296T7wAp57Tsrz164EL8MmGmg0HaxO0rJXhWnAf+eAjB8TBZxSRr2Ljj
                    3aksnDGgyvtg60zctQYKfFEf8OZkbfXefIqIxWMYaBncqa/ZhOIuBHASwa5pAdi8ADrW8b
                    UL//0fPHRAJgeMA7tpqQsjEkSxq2MWsDGjZJzC+YM2GXhetWhmpTxFYKWVa9kLR/FaAOuS
                    HpOu3W5JRTaJa4e+Wjf5q5DscZv78rAAo0pcixwc8R9vq3c0J2GrHHBaMgQ0mCGOVzw01E
                    q8kw+1G+EWKuFFm5mfhzrd9/YVfMYqp7JxZQlfuUqiQpLVi8Gm0adYznWHWgkwfvMZOBve
                    oFcKx94T63Ltl70Vrs1PYvlMxLVD3tS43jwtonbVW/srKFj7N7YuP0GbP9SoJaCeycu8mo
                    nWtBX0OTv83VRQe3mhw+fDrDsmrN6xBaKYKSqIWhnOkxd1KTV6hp2tKogxugAMqHGCosBB
                    hnjW0KzflFcYACrCAPAf4YODOvAW09MR9f6ecC9UQWt8aRt4hA7dbT/RmJwLisVmQre5kr
                    3LMMZNFll7JFino2SrrmVSb1oOQI6ZvTPUE8ldujGTt5bfBhXMEx+K4Aa53rA2qON9OSrV
                    aTHjtPyTtEewlgg+IzW4zazOsBqEzT6aHX0ikKcexXbxrXtlqYkMysKXxG8cWmRFUAwYos
                    ASpxy311CEegKGGrX95oXEZT9ykGWoPk1oM+flOCMXoM3mP/gdGb7ZOMIPOcUzrlNipiea
                    mOx2nzbBtjCsv60KK1RE3m5/oLOov2IHIAT1npte/KytsVwbBFuQl0cUhwQVgF6XArE+97
                    j1pQqAv2Atx7m/1GLs1W1KXnPC3F7bG1zbzL4va9IepvXANsPDpp4b+5qSn4+/EfHK2gcs
                    8tM9JdECtifw9SCJ8eon68OkHClEuMTOi4rLK+wvu5mFJffUC/SZlpO57o3hZO7SdTL7An
                    5sa+HQ4sSYxriIlv33b0AbtBGPhwUggx7HCt8bt+XrLv/BxSYGkbHdYSuBNxehSF02LPPz
                    MRiNjLSyB4UjKlA/KlMpZs+zqkj74Q0ssrCBE3AGISl5iA9ljJlyRRv8WR08WYjEiV/CIB
                    tYbyQmG2L/4KcmP2oRJvq/x2yr8YHsWHszAcWGhZqWRBFE081DdVDAAePSZLqEmeKF5ngF
                    qb9k4os8fUtRDvOOI9jj/b0PzCOnChxpulCMOy+l4u2SjB0q+9FMbdYqzwpEU04Zh4g/jK
                    pRE8dGah9w1GCwIGTDuAoNI91aVmlF1I9/Ow/hFyjDd0mL3PKPtR/9TSmSibGdMs7asGE8
                    HZT6+ATG6JSF5M7XkZFMUuSas0RithzPaw4/QOyQ7Dy6zBCQGNKSSer0tJIC4+qmhyI+De
                    OW0CsH2rl54/g1deo4VM00BFq3e2D+IG2RorQxFD76V1OY6m98aXGPzWUUVJfoOoS7Xs/n
                    Cgv4Fqdz8iJE6TNOgKJMECgtn/oK5icwaDI/2gkN5A0E9OHF2FDvxelAUTyNdxb1Ma3fJW
                    P1677WM/nGeOY18vIQScRkA6sc4AsUOQBqD1Zqva23hVw5DgOsuj/nzaL30TMnt0l3vdOh
                    xhryMsq1fHFyazaTGEYJRKqIEM1Afbjqhb991mrxwQFDi/ne22yiZ/KHXX871fV8+A/rYa
                    tY6YeiuNaXGJ5zYfRmcBQd7TCAQjx72uwe08IZAPnSGJT9HkquBHdXlb3RjI2sABqGVhDw
                    zwbpzSzA8kFp7S19MikkvpwiNAeSrFymH1KECwsQv2/nD/uKqgV5fdPcy8oXn3hIVkyWMd
                    kNi14kuArXKyqGobp3+F90CqD2+zVhuuDyK9yPLUhvIYr21jOQvnyK0kQkKuHW0SY6knDP
                    /4VzZ3nfgg13cjKEcJ2VgvcU1Iz/P/c2h8vpTn53wGWuEAjxmMqxWyeP7UcPDdaHuiYsk/
                    6JzpHUg35qwfaQ3Ogmc2N49wlXAT+WSqZ7YJDwu8kN0lkhnJRCQMw3W+gCXcuT6PhqUoCI
                    n/SxJpKUb5bv6jWMIIqPlxWU14+DboeEZooE9amPFkvNkRu38vjERHXxdG0cZ4hwC4f5PW
                    I935o4/5wLXSgKLNo8kBSV64FmvOD9w93zU8NXBVGaTO3POjsTuqWPW6xHfGxGBfMxND75
                    GEqQvsdlRIqkz6nfUis5hQZmrqSaPzNKBRUVmgKZJvzwVPB1n8ADvUrDNCfrehso2xQdN3
                    ZITf/bM0PwXmckuhmyfuJPem5YsEwxW3g+h4TTnNMXT3UNyu78CZ/++t+fZBXKSTKATWZc
                    kLnw7HGUflajkTF2q09Hi5TUIAIMhEzExuzJ3XfbYkzIdLV3Y+7+CB5+CJIwf/DKM91C7V
                    Gn3OOTuxRhOYISyNmwuehD0OpCVFYO6paMvZX/TWnch1x8MkiQU2jGDk2ckQCSGSK3wbXm
                    pXQ0u/L5he3snL6WJzBg5bre9natEGj4BKf/1ISrhUK5bvcSF7q+WN3p1UFDs32QRq+8+A
                    Dl1dPG05wHonDoQxZl9s9IhG7ybM3CuRhw+u1rRfh+/9o0HLTGTVf80Gab2p24r9T8ky87
                    avlECBZEGBLVGkZvIn+8xpRsTgUbL77yrRJ5wv9Z/ANsWV8gBoEFytiab/dAI7Suw1FHm0
                    kv0vAOA7/JnzAadLQKoR6RQloKXQsdSPCadk3ZmFEQ3jqzBZfgWv9X0wMZP+Y8GvI1izgE
                    WSLuxnpQOA3TXlvMuCNRwbL/sgVp1tinMkSHqAzSGLZ0Xuhisrizp/uf3Z18DLrHvkOj0H
                    9qCH/ixwynHxh6Q4JxBBTw6qwNSTV5m/6cBGVxkllnkshTjll0WmtBDSLsHjfIuAXpvc/+
                    EpA8gOmu2e64XpU59t+MTNJtZ8p3S2oRjT29tnPV8XI9wJgnADA8n/pY1iMrA/P8/ZALGg
                    UTeTvW65D2v0gR6KKuoxb64Fv40UhsnGfxeHxEqzXuEf6gAyN+9BgY0UlAVJDNnHnHl988
                    2XsFHVYO0mMmT7lWT+G5a2WU2ZWbZdruFn8dLXnwAhVT8529cYjbsYE2dWh4hWHAXxncuH
                    wXAD3Yq573S0SiPvAtTHrHCsp2gn2gn8hfk31Jf9Tuk2XIyN6bprCdonYacIYb49VaPpY7
                    RoVGXbS75LdOL1mlFaEIcCUhzGz+Y+oVIoqbKt6sBJR9S/Sn8piDcRZeidqbo4yNz3XjS3
                    pg+u1JDBJHbLTEwO7L0L7Yha+Syb7+2Wc7X3+OjZV6zqvz3stqWT58LostTXF/TOkxDCNU
                    zuKxyKAajNxntEDwPBs/cEWEKxudQyXcpE3fCYEY65jzJkZQg5Y2T1G062LPj8cC0hpNYc
                    3n1LQxo9xvDeMR1A4oA8KMZvHuV2IeG9t9jOH666ip2YNtUlsh0tck6+4v/GfWqumDI6F3
                    o4LqBzPJ+YhFdwT9xma2l3q8w4VXBMi4k060/KUArQiH+uPWkQkn8xNV2FnAPq3QMLd7Ev
                    A8ceLO7OueCqa+KDz1T0XZ7pb4d1/EqFn+JcB+QmsMUx4Ff2HhE2z2Q4k2vEFdSwa+1CiY
                    TttzwnzQ9OEmtIbD2EVlBmu7UJ9l1PlCCuaFrmqQRvzxdMuhZmDIDNLCnzZS4qlwf7vKQ0
                    Oxp6phYvlRiWgpuWbNcwMiOQdxwGIBXmwJx4BIckCca0QyDZp2Itj5oWLmeoVeL1gQdk6e
                    JnLnjp6Myk2yvOneZDuPxL9+htC1nyVhmlqP0Zzj0bfGlULdVu+uupCt8KOwNqBoZXhRrN
                    je5NM8yH7W6vV3LblaJrocbPjIj/NaApunT8ChjS8UWv0Q6jbea7v8vK+hDTpkPzV8EpXl
                    o4jjYajd2J0Nof4/+o0xMsYsvhmid2oO2sOI+a8uMslbLZbD5w9yIzrWY//YhBMs//bosX
                    SjS4oc2oxdqyq7+UvcWoA2htx8mhPPa/UGC3O9hG4e/NDqZm5EyCUCsPPR/Zp+l8IsW1eg
                    3B+X6FHIA9aHPABTcA8jXxAh6ZOcxYtU+i5gQRUnMhqvL1UBBmA5Yb47farS8dLhTN0MiB
                    m7+lXMsGfv8bILWlHqMEsPXIJvGfN5k7J9oeHMm7CBn5o9uaOW3M4BLurMv5HA9IBrbJ2A
                    2V0NY0HGvD6LdBvUWkPt/+ROHi22Fqn27ekG9yvC0fHTP4hhhBC5VTER2AabpPSOx4J0Az
                    tTjbMvX3mBd5BSnHc8DELRMcXYaEcytNcSdRkYCYsW+BOWOxjQCG95KIJjSNgE73wkmcka
                    a+Re9IenSauxiIl7wzDdrpzCoNhovdR5MUehviLMi4dCB3hKrW88uQHbgcHDcBcxManTkK
                    GEkQWcsYH+Fs4wSy9kjdifpr7UoleWkBSDcatlQt/gR2P+aE2hout5AInhl/5wsO5U53bw
                    +dn5mHJGsZhoMIRko7J5ryEcR2+62wm3fGS8O028BBGKNRsOYCj6Iai6iwgWcoG2lOE1z0
                    2DnK6DOBOLvk8kHRplAHSu7hBM+dTjY/KvUw5t6XkDjPN+XQ0Y5ugf6UuOmhZp8vxKy+dS
                    NkN0HsOI+LvPbRwZCvPoRUX5HNrS4Y47RqlXlVFiGiDtNDNO4lY0ND/J/897/ABQJMbY4x
                    d5n19bMNln5u5CucY649L+mibjm+Z3DnFHfHQcKatVbEFErZVM4LHZ/6KujbrzNX81Xc0w
                    XQ5/9wp5JV07rHNnVMUpDf3N6HSDsDeQAJau+uk3PVdtck3ATZ0tcXzcOrRgC9LTtA5NTv
                    L7GwNHK0h9Iy0zloUKXXCJ0rIP+kgYNAL+i3mf15dPlnl7wA20djzLHyHxyoy2TxKwnf6b
                    uteE2dHLmNrHVXUleMwDR4s0YPkqvPEL8JFdDjwdqWhBmZnu94hsSdZSwtxrEPbD2IzQQC
                    49jNntv5c8gN8c/zgJR4hndnStBjZ26m34XJ3vjKPGFTOhHsrKRhSAgu4qsfSnSVJQBAj9
                    sgWJj1yz4asA2VyWNfq5rEZCeGN/BXKO1qYinovNbuMU+OLmrIvX0Y0pushG9kj2TMRQTE
                    WQRtgSHA8F9dfI39PhU11Vs44Mp1SHGjbDp27hvqFCXmyH9vgqX3OB3ohS4plycnARUpMd
                    6XDKP9l1u2x7A7uc94+D+zdGEctmvlFW3E2sVMjuzBBSx3zyiF4w2F/t3pItTKe93pSe17
                    iP4b8YPKJpNGpS/d9Nq2FBP7e3BVAI5iuqWwAjWBcDcrwb7YtDYMNXyAp7+yAdgTLsPjJY
                    Q0Knxnu0lXtPG1Wui6/UhUA5H6JrWzOAzgah68hcoFgSTpftxiIheVThdtBC02fPyJe9RK
                    eVIHMvNb/pRhqo2b7J8hPDkxdPcgdh5xPzTNVyhZbaZ1bMZ4W4zSluMwnCrD/g4AYo+kvC
                    Jo12DXs0BkeGhvEjA0YJ9Tr7zZS6Y/IPs9vQU0mlOK6tWwgPNVcTSmNJOo0ovQoi88w7Ui
                    VzkhqCTktUNJgq/G6Lyyha2VerkvTFKNJazuIxjsFiWyqmcbjCzVgygi29LVT+tWkNg6eR
                    7lPjpCFbe/DbhqPNrqcvHC7JXRMKSHFk+EzK6yRcFRNo6t50tE+ILY5/bmueyFiKv+zqtM
                    mXROreqPLu+Eu4D+xoPqkpLzdztgKzEISat5rUJYO1gxpHXb+5EjsWnBTnrNPF2Z32Pq42
                    3yZPFHBrae/v8mAp42QQj8N6YlxwZjpNAo30qwh/2OmfeWvIAiVaJCQWyzKw438yEf5V/X
                    2BKl9tIaCYNp645gUGfcMDqGC46o0gI1YpdTqoxzyIARV4ZQh4M8jcNqJzCrK+sFvGu/9f
                    Y+OnR9Ed/ImU+U5zeeeTsCoRRYIzbCl+G0pm0/sWOqLuBsVDgrNKwy88Zw24jnGI88hQr9
                    sbD7ec1BZ6Cg4zykZIXUNaaV6M9vVi44MgooGrNnUIAbt/6/ZDa/pMx0uRwYnfNPfxNr4L
                    RnsH3DzPHKseM0++l5hV4cJWI847rksF7oJ2ZSjeWsh96srQgo9eSK2shoDbekIC8HngnF
                    BAvcIn69B9jeOUdLfsKrwR3CtqP3HszRBDCw1/7CkC85qXWFnPYguiAmKmKHB/pIg8kpHh
                    JFWBv68qoOTclw1IqSqcuSA4+8nTI1y+jGKvPiSUTxQr8TY4qGgD3ZGVg4rpMU3wKO0hlV
                    qZlJnl2aXELdT9tSAVPF+QbQ6na3CKZWeyzkD3F3wLHtHsaoH4m++cED6DSYgg2qxfMZE0
                    rVNm9/Ja0IBXOKgIFkFDhCRg/JcwtW8OosaBLtlCkpDF85V8sXJnCZVyneJjSnVypp4AG6
                    gREmpOv68JDhrLljiq05IhiAfkRVR7+c5L4gzYJG/jBTzjRntgpXhdXUywOoiimxPqFAyo
                    LFuz8sVRcWHF6ff+7Xgly7leMHXil7Mi1RnzvmOKxbG6TLNGcQop5GzR8wM/LVvaFuARQ0
                    TLLm5VUVeNJ8lBhOQ+d1duVEMqo/S55T4HRb5q91zE8N3N7TM4bR1y0oRG4kiov96jXllQ
                    qgeZEcOpmQga/feYw4BDA7NND+B0yE7N4gLCeh7B72THZX7pb3oF7MlLRkfaLr2ML1De4R
                    cm/y88ijgMd+qRROh/eHOE7OLEVvsxXVM5MQ9IaYBSClfzNEUYUi1n3PkuwIpZ+5Qq7p+H
                    UTydXvRdgzGHh0vdPp4LxES1Q4TuZaXQ5y3DbcoD2CrEYVo0EwfcmLM2VmEp0+W/2G8aVI
                    j1+6btQoL+RF603LZAbvwDvCDFDbALpBADnBK084jetwJsLlJ9ntdI9XUPvJuIXrq1ZcIr
                    l8OUjMvr2jBTzm809LOLfY718uW4bBS2zVIoRQxW6l/5nSReGFTbXhy/qoGbZm3fEdV9BC
                    4mBuhTg7hEbaVVYn6gtixfInfkrvp87UdJ9lqskIB8+GXOX12s0SX33zRGVBbheJ/j00Nd
                    2FjfshSjabPn/toowhmYyPVKFLYPIJKTCks/2RA1/k7z0gE2+jDobWQT7IqXg8mXdIA/qJ
                    tAl+/i4gNhLRV6cNneiTLmUC2N7DV3ZyctkiVByGjcD0UmkHD9yVhGdofhd1OYYv5jt6T7
                    NcWeOyhXR3C53C/N8tEHz01KnpJ3i8vBw/hu6SJNZhU5xDIlSZfuryZmxlxm9oJ4toAy50
                    WxOWImREfFlm8IXhKxm0hLvAwJvRE8kCpkVEIcQCGx+CTYgYSEj+UT94+TVIBcmWspPv8m
                    UjhwvMMkNMPdVLsDlXIBjp81yaOwmEC4mspXmPldQNoiVucgDY51H/py83Yg45Vpglak4e
                    8vngjlCa57aA8/opDOhj4x2ns12UHSPU5RTRqWuVkFXoQ+UM1emSWGwPPriH1byJvLvCxh
                    oUjC0oPhwkkkCOoHZmR89OSlK/3J4nGdtpnsUXZgyybswFvCJjKa+C581UhFylgb/fXOuU
                    UCgQg782u7SFh8/zP/uO+BIWuk35pySLOhHb+Geatv5qhpXpQW2LOIWz+oH/sfYbS0G5kN
                    dMWk/PjLM+rQZ8L3Ai7j4ksqjmlFiz+EWKcXYT18gsfh/SGVz+Z80HL9CQE46WU2w7x03K
                    rMRplAsaiO64l/PSMSu3UH6g2oDwnk3IkWYAsuw9OJly7G2eF/v6I1uRlvFBWFvvEOl8VC
                    pyO55328QQes/oKpJXgBOGA9e8u+ycp88h+9uq8Z5gzDHEeT66K4xEiG5SYIURrE6T/DVz
                    d0f26GIHQk/P0UmvfXUcrjcGpkOFmOG8ydluqru5fKO6QlaByOtrMyPtLiKVJ123vgRMfb
                    DBJK8nX4patiUXKrR0p5wWXBo0YYazx92o5DcF2JKJYEw39Ibhp+7h5CUcuYsN9ZVCOywJ
                    FUZh36rVA4VM3oCgWw09sVJtNp/1N6Wk43ZZs0rVg61oudh0j+RvsS/+y6GeP3Lq+nV9Vm
                    LjzMRAdAuLzrdwoJwDXs6M5jt/atebybuyug6LJrcS+fM8N6fd9N0IXZVi7B4c3Vpdbgr+
                    sPXe9AQOw0/MEiq568bejyxVWk40NT2gbcXuzOkVh8rINbKHTsKeiyWR0i98VnNOaX3Cyf
                    yRiMPt3rEatnw+aZmsL+N+dyNP1LjqDS/SfqtmK4FzDYHkffgH/lbd8Ij8WmCZEa8aqO6t
                    gmL2wB08eA0WYtHMc2Tie4bxx/EuPcyk8y0aCzfUWDdpQ/XTJ6uJI26jH/0g9RnN/qhhyz
                    g6hUv7CdeXWBbK8fQS5VHRhexfSd6szrAs0GQmamQBvTGh72GxGy5sqe0yuMNvYxLhFVVV
                    veyh0fsAxbhDym7BZMEng5PCcMSXiS3JyRBcKwCka+fDnv4IfAyx/eEdu88VOD41qGJoK3
                    h/DFQ+g9u0vXzqTjLKXToeBgc2msJH+I9WeE/4hBcdCvNDatksawttQN4d7tFQwR2lan8y
                    ayHcFUYtdbIWIwT7w8YIdt3rlrM5550A1UiHuk18Q2EMoPw7e0ttYNl8o0GBXdPR6tCXKO
                    VEkRO2cY89+EQnjOth6bWNUs1BRjyNxm4R4O61BqRl0fRFTWKgPSMY/zYh6A+F53Kz1C3e
                    wkbVFXfDI/jyJMeKU0QJlnXXmlHVVUD1IzyyUOzW0Uhnprghi1DTCaOwca1KzCFw6JcrER
                    B0C1MPdY/k0Dahu+9xunuXN13RU5qWjciNMeId1kjC8GgzgVVO4sE4Q7vQWZjIn5dOY2wh
                    m/L/5N/DgU9qqeuzIBCNJnjqMWkriVVub5q/jZKfI2HSONkjWgJ2ipxzCQHqFYV8pa6Wj0
                    xI09XLV7MSbNusyg6qWTyOzm73ilmhmPaOSrjPUm1rksURa+hLoPot1onpjVszQdyndMqZ
                    KhAann0eXpQMZMbtBXHk//WVJ3luaIX+xyfbw0+N9t5tE6cr10AjFVWFZjScRHFsKZcnxD
                    g2nO5ZTABQKJVMTgFXz4CN0lFQ28kv0oovjWXbLLi0yvuaiaLPqgd6a5mQ3fWNE+u+s7Sy
                    Fk3zMjzzlp2E6pn2Q9Jqg2JMtRjjzQ4eoZT5ngj8mKRqUx9SgtO1cdXqjET79GJapx3zx3
                    FiY0X20VNfE0YqIIPs5afRWtT7GzjDcqhVZDtGgox0kywpS8dPBStOmNUKM0UG7NI4aiho
                    g0UbPvDA205TvokQgPWhPRC5BiqQeMzbVdqY8jjcJNBM0jBKsvbkhr5vHfcm6SfC9bz7vB
                    FWi0OaNLFJONGbYX51dbblGgbMzdzXM/pvLEQiqpLOpYuo2fBvLNtRLwentXp+puby1jEL
                    il3bkbyYvpfriJh+uoJyzm+2sFsG5HZdYVxp6KOEZoAXShVHWA5d6jy2x1zJAfjloqq409
                    HH2Fi1hi8T49y34RtwXvlU2mMxrcBV58+mES9VEQ3jlTplEpTh2B0aKxdNHkqm2zFGlpDm
                    C7pBrbPj5VVbVLYKQ+B3yLcTwl0mAY/w0U2+5423dFNlDYM49Zvbe8mawdIHSAIwLKMxMp
                    o6/p3vC8KFzrZ15WyKuPgY7+bNY8SQddFfsO4dqozF9h3JlIGF//nCaSGB3UhUDQZ4ievN
                    Ke3RvqY6X+2sv0GWFCXDqYYyo9xqPI6mmLVkNr8wpJTjGZb2utKEhCu6qdSLVh5YbLLPcp
                    X+u1l8MO7dGvTfEsw+w4ta5rz1GU/eghqmA/EEVmjsryVKCkVYSWy3nO2MVkuK7vfxVuNn
                    9tATWqFJZtO7kxW34zpJvQdJ+tO2xt4trm3MjdQqgrU9oQ9DqWc/cC4G9BdeUuKtzdm6Iu
                    16djJk/RTAo74EdmSiAO1ARwyyMjLjUy8qHnWULNGvIHZBE4ZJuEocEIJovnw5XliQVeSw
                    r6ZNtKlekjAL6pKIN89C+AZHWSzdAshV5QAKjU+S0u6rDeyDQmoG3/kmPnDJvDVOfVs5Z2
                    Qxau5KrA70kqYYjRmVMkLLANxAfjnWCeMTvNEf1n8UArGabsLY/CTB5qZmUkCinMpXSyjJ
                    Ebiwa1+Hj5UQsMoKxx1nsqi6HtYoVMhbeQMcBm7dJl0AeiHRWVcgP5TmwTLg4CyJLeWO1h
                    zafTo31D1UqcMO6RpvB2Yl2nfyEDuyISSoAHj50WhNkJnFNEtKmdsUKJqyPndscJKDMTKd
                    L+my9z31NPKe+T//Xiq29UGiJrcjlneSioBjljjiWvrIL0kj1jAXhyo8kqdDAtC3/2QY1k
                    9SM1Ot2FjgdOt+HfzTRHTswPRI4LTdAOCKxTJf8KhyMVQ2KqmQyfqg7wGM2p7CcniNMyRA
                    0AP5jFrFvBvdU0QNsdjcjIn9IIwvvdeusrrBl7fDYg6V0o3MxWk+JACjGIKoDOgaXadDg7
                    41x1piWmtZ7GzjCy7Z+zMnblW94xpJElY07b4bEMAiSMDDzSznzs8LKBARGPxDZuuz9XN5
                    Hlr98jH6BCUoijR0e/2VWT2l6iawSxkvztgYOi6zFCQfoSt/+wJ+uE4kNK7g3OK8PglOYk
                    h5PJG+6wbhXMeXUQI4DewobvsJpbPzblT3pAWtscWojuyPbz8zrtBLaExnjy328Y0REql4
                    KSPiPsoEZ1PT74yiif5e8XpU7u+pnGz93UCgKSWoXRaytbZ6jyO48QnFLvGkQolMv09W8y
                    gvLuSMzD3N7638z4nxC4JKXz9FckRv12QRNfK0TzLciMVGi84/G3y1Er1HtgK3nBgAPsXN
                    cKVtzrPZxVXY3q1N1vGBlGWM5lCUjpj9lPc+C3sIctk1LJYVZS4lraBJ332ISNFintGryr
                    iskS3aY+18magwq3ifJYCEFp9ca1JScGXjzea19gYujknU7KWI1xPoTVgO9EoYA9mvlQjB
                    MOMXRxclrm/Bgc29mGw0OTyo5Gwk+MBMtT6NeqvxV0ayJbTrfZYsAHLT+us8+44CIs48Te
                    5EXgC+WYghuBQmw3jq72U8p6KBBb6miuLaCqhA9UCiAhHhA3KNPw7TM6FUDgYmbQTYyAvP
                    mgJbvAXD9DjdasAPKt8H3wWUdcfMZFp/tuBabrsEot/Xnk5fwNzSs7Pj11/i4EllxQ2kcw
                    Mo0Kb0Gu4YilFfDD83lVnzs0JnpXggFeoDHTjPd3gXeEqvTS8bHWc5xAJeHwL2/xw6QCeZ
                    dTIGkgjFw1OSizRvwuHhPpZYdOLrgl6QW/UToDQ/pj0QUoKsgdkE87QxCPC0xOzXXNU2FN
                    SFDhZqC2iIecWQC48rWut+j0H70xr7dHyBCJpxOigtxWGImNA4boWZbnIohR6qv56h+mDq
                    SxA8zcsdQD9V0WZNHivDdwvn9cDBKmdwP74fIPsZDkMu/fWc5n1JunNGGExE7jvICuPkof
                    XpebE0vmqclDw0unovDWk7Q5TMjRKIHdrK5QJ2ePs49iAUy6BWzQ+WUfUKKe6gN1VBowkK
                    iqyebAVr98IsBzlTL2Hr3Ls+r5rwT+c6hrCpTB4p+YrHPg6TU9Js/7Vz9rvmXymmWnFI+b
                    071EPjZNknOkZiWmrEIPz4ymeTLjH4jUQeVQWnv86i2AzqIRhl/I90VhrJLcMPR5ZzVR5Z
                    DfdVwZtl/652MaAlYGVplgTwFNYagpmsPF2YEzGN+WoMr+98dmj6YaxfR0XA7rs1W9SuBI
                    97sxEtITDBGw8IKRtepw3Dtavn3Y3c8Bo6fWBvH7ZGEiSsfbU75EJPceN+8kXBPmHovZV2
                    J8Ou7CvRtuPO45aZbRgx+HxEXaWUqXzrn45jqwkkPSxhVY9j/Z3losj16kbfEA5Sr4dZdK
                    EcE0/TRLhhpjT/S4+fmUOzP7jbamxfzuLZ1F92r0TfmkNbE89BdlxrnFRV33nALRc5IFNn
                    J8szbWlH7Blql3yjITgtGg73R4dbG+buuK6DCvAWpJT7fbEcRTb5RBemrm+sBrFkpoiblJ
                    SoTDFEYxIzCZPcOpZcVshw3xYF77lw2CNxupyGHsCRGaVHdk30D7GWw/LaAtyXTJSGty77
                    VwllT2itDVa8wz1/x3ZyI4ltb+0DAOLNepSK5p51H1X6DmUiGv+SlRsfGkxV34YiK8hj2s
                    N/VktF4vOf1Em0TCGgymmAHcKYBJcdZbpNTU3rjVWlhD214HwsWF5Yr9AmTw4IQg5/UAVh
                    g6R7WwyL7jJ2s1RUsrWKQMRLW5v+zXMmU8u1nV2jiq64hzSebMjbqXAFj7w6USN+3YoyU8
                    1jPliK6AgXComRZXOEmbVqDB0h/Gh7btF5jA1sKCBg3wsZLfe6ueE7t9ygDn226QiuM9xH
                    Ytu6v+bzz4RmMJ/07B202LYEkm/b9ZDfvP2p+D1O5Ygfm8HXJQ6vYEAWNys++tr85PeJX+
                    sk4LHf5peWauwlkBmKF7ZRtyjPqghAVnm2/ddzsiFIRe7nqlBq6C537fZghHxFO5EmbW4V
                    JmG3sRUcDj4egMd+WA8iwHlQZ3jCmdXQqHxixTFVccQtQYvHZCd7QuF/58to7jBibY0ArP
                    vD6vpoVsxlHqwZXPBqhJSxoBn4v2TtwOFKSjkYP8F7RJSwMWAXAaOg72yHgpZWl+5S8tpn
                    aSBE/w+CHT7ua4NUPegTz60aj1gh3EaTr2wOcBUikHkAvFeIMbUUKhOEHttypekNaPFA88
                    NxSTS0pYNsjOrWw815FKGPCjIikcTfwdHD0s2Zs7hi5fc6zZJOjkvs8P1uxLaOlLF3VNNi
                    iC36pwK6ngM5ubGHk5tmQvsUcX8x6p2aqZBNPJVCJdU4qrbFevZ3FbJcduWKpPGTcDDoAl
                    xZJWwSxeKnCUjGJEYT+Yc8emDJG8dpFKwdT7Vpu4ZN+DKxVvzzQ9/c30C+LfvPgDIHBEiM
                    SQ=="
                 />
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
use yew_router::prelude::*;
use yew::prelude::*;
use stylist::yew::use_style;

mod comps;
mod actionpage;
mod arnoldpage;
mod brucewillispage;
mod cartoonspage;
mod comedypage;
mod dramapage;
mod documentarypage;
mod fantasypage;
mod godzillapage;
mod harrypotterpage;
mod indianajonespage;
mod jamesbondpage;
mod johnwaynepage;

#[function_component]
fn MainPage() -> Html {
    let main_style = use_style!("
        height: 100vh;
        width: 900px;
        margin-right: auto;
        margin-left: auto;
    ");

    let hr_style = use_style!("
        width: 75%;
        background-color: white;
    ");

    html!(
        <>
            <main class={ main_style }>

                <comps::home_page_comps::MyTitle />

                <comps::home_page_comps::MovieSVG />

                <hr class={ hr_style } />

                <comps::home_page_comps::MovieCatagories />

                <comps::home_page_comps::PlayerControls />

                <comps::home_page_comps::ImageGroupTwo />

            </main>
        </>
    )
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/action")]
    Action,
    #[at("/arnold")]
    Arnold,
    #[at("/brucewillis")]
    BruceWillis,
    #[at("/cartoons")]
    Cartoons,
    #[at("/comedy")]
    Comedy,
    #[at("/drama")]
    Drama,
    #[at("/documentary")]
    Documentary,
    #[at("/fantasy")]
    Fantasy,
    #[at("/godzilla")]
    Godzilla,
    #[at("/harrypotter")]
    HarryPotter,
    #[at("/indianajones")]
    IndianaJones,
    #[at("/jamesbond")]
    JamesBond,
    #[at("/johnwayne")]
    JohnWayne,
    // #[at("/johnwick")]
    // JohnWick,
    // #[at("/jurassicpark")]
    // JurassicPark,
    // #[at("/kingsmen")]
    // KingsMen,
    // #[at("/meninblack")]
    // MenInBlack,
    // #[at("/misc")]
    // Misc,
    // #[at("/nicolascage")]
    // NicolasCage,
    // #[at("/pirates")]
    // Pirates,
    // #[at("/riddick")]
    // Riddick,
    // #[at("/StarWars")]
    // StarWars,
    // #[at("/startrek")]
    // StarTrek,
    // #[at("/superheroes")]
    // SuperHeroes,
    // #[at("/scifi")]
    // SciFi,
    // #[at("/tomcruize")]
    // TomCruize,
    // #[at("/transformers")]
    // Transformers,
    // #[at("/tremors")]
    // Tremors,
    // #[at("/therock")]
    // TheRock,
    // #[at("/xmen")]
    // Xmen,
    #[not_found]
    #[at("/404")]
    NotFound,
}



fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{ <MainPage /> },
        Route::Action => html!{ <actionpage::ActionPage /> },
        Route::Arnold => html!{ <arnoldpage::ArnoldPage /> },
        Route::BruceWillis => html!{ <brucewillispage::BruceWillisPage /> },
        Route::Cartoons => html!{ <cartoonspage::CartoonsPage /> },
        Route::Comedy => html!( <comedypage::ComedyPage /> ),
        Route::Drama => html!( <dramapage::DramaPage /> ),
        Route::Documentary => html!( <documentarypage::DocumentaryPage /> ),
        Route::Fantasy => html!( <fantasypage::FantasyPage /> ),
        Route::Godzilla => html!( <godzillapage::GodzillaPage /> ),
        Route::HarryPotter => html!( <harrypotterpage::HarryPotterPage /> ),
        Route::IndianaJones => html!( <indianajonespage::IndianaJonesPage /> ),
        Route::JamesBond => html!( <jamesbondpage::JamesBondPage /> ),
        Route::JohnWayne => html!( <johnwaynepage::JohnWaynePage /> ),
        Route::NotFound => html!{ <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter> 
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

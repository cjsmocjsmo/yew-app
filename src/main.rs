use yew_router::prelude::*;
use yew::prelude::*;
use stylist::yew::use_style;

mod comps;
mod movies;
mod tvshows;
mod images;

#[function_component]
fn MainPage() -> Html {
    let main_style = use_style!("
        height: 100vh;
        width: 900px;
        margin-right: auto;
        margin-left: auto;
    ");

    html!(
        <>
            <main class={ main_style }>

                <comps::mainpage_comps::MyTitle />

                <comps::mainpage_comps::TVShowsSVG />

                <comps::mainpage_comps::MovieCatagories />

                <comps::mainpage_comps::PlayerControls />

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
    #[at("/johnwick")]
    JohnWick,
    #[at("/jurassicpark")]
    JurassicPark,
    #[at("/kingsmen")]
    KingsMen,
    #[at("/meninblack")]
    MenInBlack,
    #[at("/misc")]
    Misc,
    #[at("/nicolascage")]
    NicolasCage,
    #[at("/pirates")]
    Pirates,
    #[at("/riddick")]
    Riddick,
    #[at("/StarWars")]
    StarWars,
    #[at("/startrek")]
    StarTrek,
    #[at("/superheroes")]
    SuperHeroes,
    #[at("/scifi")]
    SciFi,
    #[at("/tomcruize")]
    TomCruize,
    #[at("/transformers")]
    Transformers,
    #[at("/tremors")]
    Tremors,
    #[at("/therock")]
    TheRock,
    #[at("/xmen")]
    XMen,

    #[at("/tvshows")]
    TVShows,
    #[at("/tvfantazy")]
    TVFantazy,
    #[at("/tvstartrek")]
    TVStarTrek,
    #[at("/tvstarwars")]
    TVStarWars,
    #[at("/tvsciti")]
    TVSciFi,
    #[at("/mcu")]
    MCU,
    #[at("/western")]
    Western,
    #[not_found]
    #[at("/404")]
    NotFound,
}



fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{ <MainPage /> },
        Route::Action => html!{ <movies::actionpage::ActionPage /> },
        Route::Arnold => html!{ <movies::arnoldpage::ArnoldPage /> },
        Route::BruceWillis => html!{ <movies::brucewillispage::BruceWillisPage /> },
        Route::Cartoons => html!{ <movies::cartoonspage::CartoonsPage /> },
        Route::Comedy => html!( <movies::comedypage::ComedyPage /> ),
        Route::Drama => html!( <movies::dramapage::DramaPage /> ),
        Route::Documentary => html!( <movies::documentarypage::DocumentaryPage /> ),
        Route::Fantasy => html!( <movies::fantasypage::FantasyPage /> ),
        Route::Godzilla => html!( <movies::godzillapage::GodzillaPage /> ),
        Route::HarryPotter => html!( <movies::harrypotterpage::HarryPotterPage /> ),
        Route::IndianaJones => html!( <movies::indianajonespage::IndianaJonesPage /> ),
        Route::JamesBond => html!( <movies::jamesbondpage::JamesBondPage /> ),
        Route::JohnWayne => html!( <movies::johnwaynepage::JohnWaynePage /> ),
        Route::JohnWick => html!( <movies::johnwickpage::JohnWickPage /> ),
        Route::JurassicPark => html!( <movies::jurassicparkpage::JurassicParkPage /> ),
        Route::KingsMen => html!( <movies::kingsmenpage::KingsMenPage /> ),
        Route::MenInBlack => html!( <movies::meninblackpage::MenInBlackPage /> ),
        Route::Misc => html!( <movies::miscpage::MiscPage /> ),
        Route::NicolasCage => html!( <movies::nicolascagepage::NicolasCagePage /> ),
        Route::Pirates => html!( <movies::piratespage::PiratesPage /> ),
        Route::Riddick => html!( <movies::riddickpage::RiddickPage /> ),
        Route::SciFi => html!( <movies::scifipage::SciFiPage /> ),
        Route::StarTrek => html!( <movies::startrekpage::StarTrekPage /> ),
        Route::StarWars => html!( <movies::starwarspage::StarWarsPage /> ),
        Route::SuperHeroes => html!( <movies::superheroespage::SuperHeroesPage /> ),
        Route::TheRock => html!( <movies::therockpage::TheRockPage /> ),
        Route::TomCruize => html!( <movies::tomcruizepage::TomCruizePage /> ),
        Route::Transformers => html!( <movies::transformerspage::TransformersPage /> ),
        Route::Tremors => html!( <movies::tremorspage::TremorsPage /> ),
        Route::XMen => html!( <movies::xmenpage::XMenPage /> ),

        Route::TVShows => html!( <tvshows::tvshowspage::TVShowsPage /> ),
        Route::TVFantazy => html!( <tvshows::fantasypage::TVFantasyPage /> ),
        Route::TVStarTrek => html!( <tvshows::startrekpage::TVStarTrekPage /> ),
        Route::TVStarWars => html!( <tvshows::starwarspage::TVStarWarsPage /> ),
        Route::TVSciFi => html!( <tvshows::scifipage::TVSciFiPage /> ),
        Route::MCU => html!( <tvshows::mcupage::MCUPage /> ),
        Route::Western => html!( <tvshows::westerenspage::WesternPage /> ),
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

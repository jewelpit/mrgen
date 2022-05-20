use yew::prelude::*;
use yew_router::prelude::*;

mod generators;
mod roller;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,

    #[at("/characters")]
    Characters,
    #[at("/magic")]
    Magic,
    #[at("/monsters")]
    Monsters,
    #[at("/npcs")]
    NPCs,
    #[at("/items")]
    Items,
    #[at("/cities")]
    Cities,
    #[at("/wild")]
    TheWild,
    #[at("/maze")]
    TheMaze,

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <>
                <h2 class="title">{"Welcome!"}</h2>
                <p>
                    {"
                    Welcome to the unofficial Maze Rats generator! Maze Rats (by Ben Milton/Questing
                    Beast Games) has a lot of really wonderful random tables in it, and I thought that
                    making a generator that rolls on them would be a fun day project.
                    "}
                </p>
            </>
        },

        Route::Characters => html! { <generators::Characters /> },
        Route::Magic => html! { <generators::Magic /> },
        // Route::Monsters => html! { <Monsters /> },
        Route::NPCs => html! { <generators::NPCs /> },
        // Route::Items => html! { <Items /> },
        // Route::Cities => html! { <Cities /> },
        // Route::TheWild => html! { <TheWild /> },
        // Route::TheMaze => html! { <TheMaze /> },
        Route::NotFound | _ => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav class="navbar is-info has-shadow" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <div class="navbar-item">
                        <span class="icon is-large">
                            <span class="mdi mdi-36px mdi-rodent">
                            </span>
                        </span>
                    </div>
                    <p class="navbar-item title" style="margin-bottom: 0px;">
                        {"Maze Rats Generator (unofficial)"}
                    </p>
                </div>
            </nav>
            <section class="section">
                <div class="container">
                    <div class="columns">
                        <div class="column is-narrow">
                            <aside class="menu box">
                                <p class="menu-label">{"Maze Rats Generator"}</p>
                                <ul class="menu-list">
                                    <li><Link<Route> to={Route::Characters}>{"Characters"}</Link<Route>></li>
                                    <li><Link<Route> to={Route::Magic}>{"Magic"}</Link<Route>></li>
                                    <li><Link<Route> to={Route::Monsters}>{"Monsters"}</Link<Route>></li>
                                    <li><Link<Route> to={Route::NPCs}>{"NPCs"}</Link<Route>></li>
                                    <li><Link<Route> to={Route::Items}>{"Items"}</Link<Route>></li>
                                    <li><Link<Route> to={Route::Cities}>{"Cities"}</Link<Route>></li>
                                    <li><Link<Route> to={Route::TheWild}>{"The Wild"}</Link<Route>></li>
                                    <li><Link<Route> to={Route::TheMaze}>{"The Maze"}</Link<Route>></li>
                                </ul>
                            </aside>
                        </div>
                        <div class="column">
                            <Switch<Route> render={Switch::render(switch)} />
                        </div>
                    </div>
                </div>
            </section>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Main>();
}

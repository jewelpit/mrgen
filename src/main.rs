use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

mod components;
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
    #[at("/npcs")]
    NPCs,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone)]
struct AppState {
    character: generators::characters::CharacterData,
    spells: Rc<Vec<String>>,
    npcs: Rc<Vec<generators::npcs::NPCData>>,
}

fn switch(routes: &Route, state: &UseStateHandle<AppState>) -> Html {
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

        Route::Characters => {
            let state = state.clone();
            let inner_state = (*state).clone();
            html! {
                <generators::characters::Character
                    data={state.character.clone()}
                    update={Callback::once(move |c| state.set(AppState {character: c, ..inner_state}))} />
            }
        }
        Route::Magic => {
            let state = state.clone();
            let inner_state = (*state).clone();
            html! {
                <generators::magic::Magic
                    spells={state.spells.clone()}
                    update={Callback::once(move |s| state.set(AppState {spells: s, ..inner_state}))} />
            }
        }
        Route::NPCs => {
            let state = state.clone();
            let inner_state = (*state).clone();
            html! {
                <generators::npcs::NPCs
                    npcs={state.npcs.clone()}
                    update={Callback::once(move |n| state.set(AppState {npcs: n, ..inner_state}))} />
            }
        }
        Route::NotFound => html! { <h1 class="title">{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    let state = use_state(|| AppState {
        character: generators::characters::CharacterData::new(),
        spells: generators::magic::MagicProps::get_spells(),
        npcs: generators::npcs::NPCProps::get_npcs(),
    });

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
                <div class="navbar-menu">
                    <div class="navbar-end">
                        <div class="navbar-item">
                            <a class="navbar-link is-arrowless" href="https://github.com/jewelpit/mrgen">{"source"}</a>
                        </div>
                    </div>
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
                                    <li><Link<Route> to={Route::NPCs}>{"NPCs"}</Link<Route>></li>
                                </ul>
                            </aside>
                        </div>
                        <div class="column">
                            <Switch<Route> render={Switch::render(move |r| switch(r, &state))} />
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

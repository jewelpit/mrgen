use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <section class="section">
                <nav class="navbar" role="navigation" aria-label="main navigation">
                    <div class="navbar-brand">
                        <p class="navbar-item title">
                            {"Maze Rats Generator (unofficial)"}
                        </p>

                        <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                        </a>
                    </div>
                </nav>
                <div class="container">
                    <div class="columns">
                        <div class="column is-narrow">
                            <aside class="menu">
                                <p class="subtitle has-text-centered">{"Maze Rats Generator"}</p>
                                <ul class="menu-list">
                                    <li><a>{"Characters"}</a></li>
                                    <li><a>{"Magic"}</a></li>
                                    <li><a>{"Monsters"}</a></li>
                                    <li><a>{"NPCs"}</a></li>
                                    <li><a>{"Items"}</a></li>
                                    <li><a>{"Cities"}</a></li>
                                    <li><a>{"The Wild"}</a></li>
                                    <li><a>{"The Maze"}</a></li>
                                </ul>
                            </aside>
                        </div>
                        <div class="column">
                            <h1 class="title">{"Section"}</h1>
                            <h2 class="subtitle">
                                {"A simple container to divide your page into <strong>sections</strong>, like the one you're currently reading."}
                            </h2>
                            <div>
                                <button onclick={link.callback(|_| Msg::AddOne)}>{"+1"}</button>
                                <p>{self.value}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

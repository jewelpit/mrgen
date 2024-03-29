use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct RerollButtonProps {
    pub onclick: Callback<MouseEvent>,

    #[prop_or("Reroll!".to_string())]
    pub text: String,
}

#[function_component(RerollButton)]
pub fn reroll_button(props: &RerollButtonProps) -> Html {
    html! {
        <button class="button is-danger" onclick={props.onclick.clone()}>
            <span class="icon is-medium">
                <span class="mdi mdi-dice-multiple"></span>
            </span>
            <span>{&props.text}</span>
        </button>
    }
}

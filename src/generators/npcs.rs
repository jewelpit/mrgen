use yew::prelude::*;

use crate::roller::Rollable;

pub fn get_name() -> String {
    let given_name = [
        [
            "Adelaide", "Alma", "Barsaba", "Beatrix", "Bianca", "Cleopha",
        ],
        [
            "Clover",
            "Constance",
            "Damaris",
            "Daphne",
            "Demona",
            "Elsbeth",
        ],
        ["Esme", "Fern", "Hester", "Hippolyta", "Jesamine", "Jilly"],
        ["Morgot", "Minerva", "Nerissa", "Odette", "Olga", "Orchid"],
        ["Pepper", "Phoebe", "Piety", "Poppy", "Silence", "Sybil"],
        ["Trillby", "Tuesday", "Ursula", "Vivian", "Wendy", "Zora"],
        [
            "Balthazar",
            "Basil",
            "Bertram",
            "Blaxton",
            "Chadwick",
            "Clovis",
        ],
        [
            "Destrian", "Ellis", "Erasmus", "Faustus", "Finn", "Fitzhugh",
        ],
        ["Florian", "Fox", "Godwin", "Hannibal", "Jasper", "Jiles"],
        ["Jules", "Leopold", "Merrick", "Mortimer", "Ogden", "Orion"],
        [
            "Oswald",
            "Percival",
            "Peregrine",
            "Quentin",
            "Redmaine",
            "Reinhold",
        ],
        [
            "Silas",
            "Stilton",
            "Stratford",
            "Tenpiece",
            "Waverly",
            "Webster",
        ],
    ]
    .roll()
    .roll();

    let family_name = [
        [
            "Belvedere",
            "Bithesea",
            "Calaver",
            "Carvolo",
            "De Rippe",
            "Droll",
        ],
        ["Dunlow", "Edevane", "Erelong", "Febland", "Fernsby", "Fisk"],
        [
            "Gastrell",
            "Girdwood",
            "Gorgon",
            "Grimeson",
            "Gruger",
            "Hitheryon",
        ],
        [
            "La Marque",
            "Malmora",
            "Miter",
            "Oblington",
            "Onymous",
            "Phillifent",
        ],
        [
            "Porendorfer",
            "Romatet",
            "Rothery",
            "Skorbeck",
            "Slora",
            "Southwark",
        ],
        [
            "Stavish",
            "Vandermeer",
            "Wellbelove",
            "Westergren",
            "Wexley",
            "Wilberforce",
        ],
        [
            "Barrow",
            "Beetleman",
            "Berrycloth",
            "Birdwhistle",
            "Bobich",
            "Chips",
        ],
        [
            "Coffin",
            "Crumpling",
            "Culpepper",
            "Dankworth",
            "Digworthy",
            "Dreggs",
        ],
        [
            "Gimble",
            "Graveworm",
            "Greelish",
            "Hardwick",
            "Hatman",
            "Hovel",
        ],
        [
            "Knibbs",
            "Midnighter",
            "Needle",
            "Nethercoat",
            "Pestle",
            "Relish",
        ],
        [
            "Rumbold",
            "Rummage",
            "Sallow",
            "Saltmarsh",
            "Silverless",
            "Skitter",
        ],
        [
            "Slee",
            "Slitherly",
            "Stoker",
            "Tarwater",
            "Tumbler",
            "Villin",
        ],
    ]
    .roll()
    .roll();

    format!("{} {}", given_name, family_name)
}

#[function_component(NPCs)]
pub fn npcs() -> Html {
    html! {
        <>
            <p class="title has-text-centered">{get_name()}</p>
        </>
    }
}

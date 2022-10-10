use std::rc::Rc;

use yew::prelude::*;

use crate::components::RerollButton;
use crate::roller::Rollable;

use super::city::get_faction;
use super::magic::get_insanity;

#[derive(Clone, PartialEq)]
pub struct NPCData {
    name: String,
    job: String,
    asset: String,
    liability: String,
    goal: String,
    misfortune: String,
    mission: String,
    methods: String,
    appearance: String,
    physical_details: String,
    clothing: String,
    personality: String,
    mannerisms: String,
    secret: String,
    reputation: String,
    hobby: String,
}

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

fn get_civilized_job() -> String {
    [
        [
            "Acolyte",
            "Actor",
            "Apothecary",
            "Baker",
            "Barber",
            "Blacksmith",
        ],
        [
            "Brewer",
            "Bureaucrat",
            "Butcher",
            "Carpenter",
            "Clockmaker",
            "Courier",
        ],
        [
            "Courtier",
            "Diplomat",
            "Fishmonger",
            "Guard",
            "Haberdasher",
            "Innkeeper",
        ],
        [
            "Item-seller",
            "Jeweler",
            "Knight",
            "Locksmith",
            "Mason",
            "Miller",
        ],
        [
            "Musician", "Noble", "Painter", "Priest", "Scholar", "Scribe",
        ],
        [
            "Sculptor",
            "Shipwright",
            "Soldier",
            "Tailor",
            "Taxidermist",
            "Wigmaker",
        ],
    ]
    .roll()
    .roll()
    .to_string()
}

fn get_underworld_job() -> String {
    [
        [
            "Alchemist",
            "Beggar-prince",
            "Blackmailer",
            "Bounty-hunter",
            "Chimney sweep",
            "Coin-clipper",
        ],
        [
            "Contortionist",
            "Counterfeiter",
            "Cultist",
            "Cutpurse",
            "Debt-collector",
            "Deserter",
        ],
        [
            "Fence",
            "Fortuneteller",
            "Galley slave",
            "Gambler",
            "Gravedigger",
            "Headsman",
        ],
        [
            "Hedge knight",
            "Highwayman",
            "Housebreaker",
            "Kidnapper",
            "Mad prophet",
            "Mountebank",
        ],
        [
            "Peddler",
            "Pit-fighter",
            "Poisoner",
            "Rat-catcher",
            "Scrivener",
            "Sellsword",
        ],
        [
            "Slave",
            "Smuggler",
            "Street performer",
            "Tattooist",
            "Urchin",
            "Usurer",
        ],
    ]
    .roll()
    .roll()
    .to_string()
}

fn get_wilderness_job() -> String {
    [
        [
            "Apiarist",
            "Bandit",
            "Caravan guard",
            "Caravaneer",
            "Druid",
            "Exile",
        ],
        [
            "Explorer",
            "Farmer",
            "Fisherman",
            "Forager",
            "Fugitive",
            "Hedge wizard",
        ],
        [
            "Hermit",
            "Hunter",
            "Messenger",
            "Minstrel",
            "Monk",
            "Monster hunter",
        ],
        [
            "Outlander",
            "Tinker",
            "Pilgrim",
            "Poacher",
            "Raider",
            "Ranger",
        ],
        ["Sage", "Scavenger", "Scout", "Shepherd", "Seer", "Surveyor"],
        [
            "Tinker",
            "Tomb raider",
            "Trader",
            "Trapper",
            "Witch",
            "Woodcutter",
        ],
    ]
    .roll()
    .roll()
    .to_string()
}

fn get_job() -> String {
    [get_civilized_job, get_underworld_job, get_wilderness_job].roll()()
}

fn get_asset() -> String {
    [
        [
            || "Authority".to_string(),
            || "Avoids detection".to_string(),
            || "Calls in favors".to_string(),
            || "Charming".to_string(),
            || "Cooks the books".to_string(),
            || "Erases evidence".to_string(),
        ],
        [
            || "Excellent liar".to_string(),
            || "Extremely rich".to_string(),
            || format!("Leader of (Faction: {})", get_faction()),
            || format!("Member of (Faction: {})", get_faction()),
            || "Feared".to_string(),
            || "Fortified base".to_string(),
        ],
        [
            || "Gorgeous".to_string(),
            || "Hears rumrs".to_string(),
            || "Huge family".to_string(),
            || "Huge library".to_string(),
            || "Impersonator".to_string(),
            || "Interrogator".to_string(),
        ],
        [
            || "Knows a guy".to_string(),
            || "Knows a way in".to_string(),
            || "Launders money".to_string(),
            || "Learned".to_string(),
            || "Local celebrity".to_string(),
            || "Local knowledge".to_string(),
        ],
        [
            || "Loyal henchmen".to_string(),
            || "Middling oracle".to_string(),
            || "Nothing to lose".to_string(),
            || "Owns the guards".to_string(),
            || "Powerful spouse".to_string(),
            || "Procures gear".to_string(),
        ],
        [
            || "Pulls the strings".to_string(),
            || "Secret lab".to_string(),
            || "Sells contraband".to_string(),
            || "Smuggles goods".to_string(),
            || "Spy network".to_string(),
            || "War hero".to_string(),
        ],
    ]
    .roll()
    .roll()()
}

fn get_liability() -> String {
    [
        [
            || "Addiction".to_string(),
            || "Alcoholic".to_string(),
            || "Corrupt ally".to_string(),
            || "Coward".to_string(),
            || "Decadent".to_string(),
            || "Forbidden love".to_string(),
        ],
        [
            || "Gambler".to_string(),
            || "Glutton".to_string(),
            || "Greedy".to_string(),
            || "Heretic".to_string(),
            || "Huge debts".to_string(),
            || "Imposter".to_string(),
        ],
        [
            get_insanity,
            || "Jealous".to_string(),
            || "Leaves evidence".to_string(),
            || "Many enemies".to_string(),
            || "Misinformed".to_string(),
            || "Money trail".to_string(),
        ],
        [
            || "Narcissist".to_string(),
            || "Needs medicine".to_string(),
            || "OCD".to_string(),
            || "Paranoid".to_string(),
            || "Partyer".to_string(),
            || "Poor equipment".to_string(),
        ],
        [
            || "Protective".to_string(),
            || "Scandalous".to_string(),
            || "Softhearted".to_string(),
            || "Strict routines".to_string(),
            || "Superstitious".to_string(),
            || "Suspicious".to_string(),
        ],
        [
            || "Temper".to_string(),
            || "Trusting".to_string(),
            || "Vulnerable base".to_string(),
            || "Wanted".to_string(),
            || "Weak-willed".to_string(),
            || "Widely despised".to_string(),
        ],
    ]
    .roll()
    .roll()()
}

fn get_goal() -> String {
    "goal".to_string()
}

fn get_misfortune() -> String {
    "misfortune".to_string()
}

fn get_mission() -> String {
    "mission".to_string()
}

fn get_methods() -> String {
    "methods".to_string()
}

fn get_appearance() -> String {
    "appearance".to_string()
}

fn get_physical_details() -> String {
    "physical_details".to_string()
}

fn get_clothing() -> String {
    "clothing".to_string()
}

pub fn get_personality() -> String {
    [
        [
            "Bitter", "Brave", "Cautious", "Chipper", "Contrary", "Cowardly",
        ],
        [
            "Cunning",
            "Driven",
            "Entitled",
            "Gregarious",
            "Grumpy",
            "Heartless",
        ],
        [
            "Honor-bound",
            "Hotheaded",
            "Inquisitive",
            "Irascible",
            "Jolly",
            "Know-it-all",
        ],
        [
            "Lazy",
            "Loyal",
            "Menacing",
            "Mopey",
            "Nervous",
            "Protective",
        ],
        [
            "Righteous",
            "Rude",
            "Sarcastic",
            "Savage",
            "Scheming",
            "Serene",
        ],
        [
            "Spacey",
            "Stoic",
            "Stubborn",
            "Stuck-up",
            "Suspicious",
            "Wisecracking",
        ],
    ]
    .roll()
    .roll()
    .to_string()
}

fn get_mannerisms() -> String {
    "mannerisms".to_string()
}

fn get_secret() -> String {
    "secret".to_string()
}

fn get_reputation() -> String {
    "reputation".to_string()
}

fn get_hobby() -> String {
    "hobby".to_string()
}

#[derive(Clone, PartialEq, Properties)]
pub struct NPCProps {
    pub npcs: Rc<Vec<NPCData>>,
    pub update: Callback<Rc<Vec<NPCData>>>,
}

impl NPCProps {
    pub fn get_npcs() -> Rc<Vec<NPCData>> {
        Rc::new(
            (0..10)
                .map(|_| NPCData {
                    name: get_name(),
                    job: get_job(),
                    asset: get_asset(),
                    liability: get_liability(),
                    goal: get_goal(),
                    misfortune: get_misfortune(),
                    mission: get_mission(),
                    methods: get_methods(),
                    appearance: get_appearance(),
                    physical_details: get_physical_details(),
                    clothing: get_clothing(),
                    personality: get_personality(),
                    mannerisms: get_mannerisms(),
                    secret: get_secret(),
                    reputation: get_reputation(),
                    hobby: get_hobby(),
                })
                .collect(),
        )
    }
}

#[function_component(NPCs)]
pub fn npcs(props: &NPCProps) -> Html {
    let reroll = {
        let update = props.update.clone();
        Callback::from(move |_| update.emit(NPCProps::get_npcs()))
    };

    html! {
        <>
        <nav class="level">
            <h1 class="level-item title has-text-centered" style={"margin: 0px;"}>{"NPCs"}</h1>
        </nav>
            {
                props.npcs.iter().map(|npc| {
                    html! {
                        <div class="block">
                            <div class="card">
                                <header class="card-header">
                                    <p class="card-header-title subtitle my-0 py-0">
                                        {&npc.name}
                                    </p>
                                    <span class="card-header-icon" aria-label="more options">
                                        <RerollButton onclick={&reroll} />
                                    </span>
                                </header>
                                <div class="card-content">
                                    <div class="columns">
                                        <div class="column">
                                            <strong>{"Job: "}</strong>{&npc.job}<br />
                                            {"reputation: "}{&npc.reputation}<br />
                                            <strong>{"Asset: "}</strong>{&npc.asset}<br />
                                            <strong>{"Liability: "}</strong>{&npc.liability}<br />
                                            {"goal: "}{&npc.goal}<br />
                                        </div>
                                        <div class="column">
                                            {"misfortune: "}{&npc.misfortune}<br />
                                            {"mission: "}{&npc.mission}<br />
                                            {"methods: "}{&npc.methods}<br />
                                            {"appearance: "}{&npc.appearance}<br />
                                            {"physical_details: "}{&npc.physical_details}<br />
                                        </div>
                                        <div class="column">
                                            {"clothing: "}{&npc.clothing}<br />
                                            {"personality: "}{&npc.personality}<br />
                                            {"mannerisms: "}{&npc.mannerisms}<br />
                                            {"secret: "}{&npc.secret}<br />
                                            {"hobby: "}{&npc.hobby}<br />
                                        </div>
                                    </div>
                                    <div class="content">
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                }).collect::<Html>()
            }
        </>
    }
}

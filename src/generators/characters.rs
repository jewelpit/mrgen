use yew::prelude::*;

use crate::generators::magic::get_spell;
use crate::generators::npcs::get_name;
use crate::roller::Rollable;

struct Abilities {
    str: usize,
    dex: usize,
    will: usize,
}

impl Abilities {
    fn get() -> Self {
        let &(str, dex, will) = ([
            (2, 1, 0),
            (2, 0, 1),
            (1, 2, 0),
            (0, 2, 1),
            (1, 0, 2),
            (0, 1, 2),
        ])
        .roll();
        Self { str, dex, will }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TrainingPath {
    Briarborn,
    Fingersmith,
    Roofrunner,
    Shadowjack,
}

impl std::fmt::Display for TrainingPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).trim_start_matches(r"\S+::"))
    }
}

#[derive(Clone, PartialEq)]
enum Features {
    AttackBonus,
    SpellSlot,
    Training(TrainingPath),
}

impl Features {
    fn get() -> Self {
        ([
            Self::AttackBonus,
            Self::SpellSlot,
            Self::Training(
                *([
                    TrainingPath::Briarborn,
                    TrainingPath::Fingersmith,
                    TrainingPath::Roofrunner,
                    TrainingPath::Shadowjack,
                ])
                .roll(),
            ),
        ]
        .roll())
        .clone()
    }
}

impl std::fmt::Display for Features {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AttackBonus => write!(f, "+1 attack bonus"),
            Self::SpellSlot => {
                let (first, second) = get_spell();
                write!(f, "spell slot: {} {}", first, second)
            }
            Self::Training(path) => write!(f, "{} training", path),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum WeaponType {
    LightWeapon,
    HeavyWeapon,
    RangedWeapon,
}

struct Weapon {
    name: String,
    kind: WeaponType,
}

impl Weapon {
    fn get(kind: Option<WeaponType>) -> Weapon {
        let kind = kind.unwrap_or_else(|| {
            *([
                WeaponType::LightWeapon,
                WeaponType::HeavyWeapon,
                WeaponType::RangedWeapon,
            ]
            .roll())
        });
        match &kind {
            WeaponType::LightWeapon => Weapon {
                name: [
                    "Axe",
                    "Dagger",
                    "Mace",
                    "Short sword",
                    "Flail",
                    "One-handed spear",
                ]
                .roll()
                .to_string(),
                kind,
            },
            WeaponType::HeavyWeapon => Weapon {
                name: ["Spear", "Halberd", "Long sword", "Warhammer"]
                    .roll()
                    .to_string(),
                kind,
            },
            WeaponType::RangedWeapon => Weapon {
                name: ["Bow", "Crossbow", "Sling"].roll().to_string(),
                kind,
            },
        }
    }
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({})",
            self.name,
            match self.kind {
                WeaponType::LightWeapon => "1 hand",
                WeaponType::HeavyWeapon => "+1 damage, 2 hands",
                WeaponType::RangedWeapon => "ranged, 2 hands",
            }
        )
    }
}

fn get_starting_weapons() -> Vec<Weapon> {
    let first = Weapon::get(None);
    match first.kind {
        WeaponType::LightWeapon => {
            vec![first, Weapon::get(None)]
        }
        WeaponType::HeavyWeapon => {
            vec![
                first,
                Weapon::get(Some(
                    *([WeaponType::LightWeapon, WeaponType::RangedWeapon].roll()),
                )),
            ]
        }
        WeaponType::RangedWeapon => {
            vec![
                first,
                Weapon::get(Some(
                    *([WeaponType::LightWeapon, WeaponType::HeavyWeapon].roll()),
                )),
            ]
        }
    }
}

fn get_starting_item() -> String {
    ([
        [
            "Animal scent",
            "Bear trap",
            "Bedroll",
            "Caltrops",
            "Chain (10 ft.)",
            "Chalk",
        ],
        [
            "Chisel",
            "Crowbar",
            "Fishing net",
            "Glass marbles",
            "Glue",
            "Grappling hook",
        ],
        [
            "Grease",
            "Hacksaw",
            "Hammer",
            "Hand drill",
            "Horn",
            "Iron spikes",
        ],
        [
            "Iron tongs",
            "Lantern and oil",
            "Large sack",
            "Lockpicks (3)",
            "Manacles",
            "Medicine (3)",
        ],
        [
            "Metal file",
            "Rations (3)",
            "Rope (50 ft.)",
            "Steel wire",
            "Shovel",
            "Steel mirror",
        ],
        [
            "Ten foot pole",
            "Tinderbox",
            "Torch",
            "Vial of acid",
            "Vial of poison",
            "Waterskin",
        ],
    ]
    .roll()
    .roll())
    .to_string()
}

fn get_personal_traits() -> Vec<(String, String)> {
    vec![
        (
            "Appearance".to_string(),
            [
                [
                    "Aquiline",
                    "Athletic",
                    "Barrel-chested",
                    "Boney",
                    "Brawny",
                    "Brutish",
                ],
                [
                    "Bullnecked",
                    "Chiseled",
                    "Coltish",
                    "Corpulent",
                    "Craggy",
                    "Delicate",
                ],
                [
                    "Furrowed", "Gaunt", "Gorgeous", "Grizzled", "Haggard", "Handsome",
                ],
                ["Hideous", "Lanky", "Pudgy", "Ripped", "Rosy", "Scrawny"],
                [
                    "Sinewy",
                    "Slender",
                    "Slumped",
                    "Solid",
                    "Square-jawed",
                    "Statuesque",
                ],
                [
                    "Towering",
                    "Trim",
                    "Weathered",
                    "Willowy",
                    "Wiry",
                    "Wrinkled",
                ],
            ]
            .roll()
            .roll()
            .to_string(),
        ),
        (
            "Physical detail".to_string(),
            [
                [
                    "Acid scars",
                    "Battle scars",
                    "Birthmark",
                    "Braided hair",
                    "Brand mark",
                    "Broken nose",
                ],
                [
                    "Bronze-skinned",
                    "Burn scars",
                    "Bushy eyebrows",
                    "Curly hair",
                    "Dark skinned",
                    "Dreadlocks",
                ],
                [
                    "Exotic accent",
                    "Flogging scars",
                    "Freckles",
                    "Gold tooth",
                    "Hoarse voice",
                    "Huge beard",
                ],
                [
                    "Long hair",
                    "Matted hair",
                    "Missing ear",
                    "Missing teeth",
                    "Mustache",
                    "Muttonchops",
                ],
                [
                    "Nine fingers",
                    "Oiled hair",
                    "One-eyed",
                    "Pale skinned",
                    "Piercings",
                    "Ritual scars",
                ],
                [
                    "Sallow skin",
                    "Shaved head",
                    "Sunburned",
                    "Tangled hair",
                    "Tattoos",
                    "Topknot",
                ],
            ]
            .roll()
            .roll()
            .to_string(),
        ),
        (
            "Background".to_string(),
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
            .to_string(),
        ),
        (
            "Clothing".to_string(),
            [
                [
                    "Antique",
                    "Battle-torn",
                    "Bedraggled",
                    "Blood-stained",
                    "Ceremonial",
                    "Dated",
                ],
                [
                    "Decaying",
                    "Eccentric",
                    "Elegant",
                    "Embroidered",
                    "Exotic",
                    "Fashionable",
                ],
                [
                    "Flamboyant",
                    "Food-stained",
                    "Formal",
                    "Frayed",
                    "Frumpy",
                    "Garish",
                ],
                [
                    "Grimy",
                    "Haute couture",
                    "Lacey",
                    "Livery",
                    "Mud-stained",
                    "Ostentatious",
                ],
                [
                    "Oversized",
                    "Patched",
                    "Patterned",
                    "Perfumed",
                    "Practical",
                    "Rumpled",
                ],
                [
                    "Sigils",
                    "Singed",
                    "Tasteless",
                    "Undersized",
                    "Wine-stained",
                    "Worn out",
                ],
            ]
            .roll()
            .roll()
            .to_string(),
        ),
        (
            "Personality".to_string(),
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
            .to_string(),
        ),
        (
            "Mannerism".to_string(),
            [
                [
                    "Anecdotes",
                    "Breathy",
                    "Chuckles",
                    "Clipped",
                    "Cryptic",
                    "Deep voice",
                ],
                [
                    "Drawl",
                    "Enunciates",
                    "Flowery speech",
                    "Gravelly voice",
                    "Highly formal",
                    "Hypnotic",
                ],
                [
                    "Interrupts",
                    "Laconic",
                    "Laughs",
                    "Long pauses",
                    "Melodious",
                    "Monotone",
                ],
                [
                    "Mumbles",
                    "Narrates",
                    "Overly casual",
                    "Quaint sayings",
                    "Rambles",
                    "Random facts",
                ],
                [
                    "Rapid-fire",
                    "Rhyming",
                    "Robotic",
                    "Slow speech",
                    "Speechifies",
                    "Squeaky",
                ],
                [
                    "Street slang",
                    "Stutters",
                    "Talks to self",
                    "Trails off",
                    "Very loud",
                    "Whispers",
                ],
            ]
            .roll()
            .roll()
            .to_string(),
        ),
    ]
}

#[function_component(Characters)]
pub fn characters() -> Html {
    let abilities = Abilities::get();
    let starting_feature = Features::get();
    let starting_weapons = get_starting_weapons();
    let can_use_shield = starting_weapons
        .iter()
        .any(|w| w.kind == WeaponType::LightWeapon);
    let starting_items = (0..5).map(|_| get_starting_item()).collect::<Vec<_>>();
    let personal_traits = get_personal_traits();

    html! {
        <div class="columns">
            <div class="column">
                <h1 class="title has-text-centered">{get_name()}</h1>
                <nav class="level">
                    <div class="level-item"></div>
                    <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{"STR"}</p>
                            <p class="subtitle">{abilities.str}</p>
                        </div>
                    </div>
                    <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{"DEX"}</p>
                            <p class="subtitle">{abilities.dex}</p>
                        </div>
                    </div>
                    <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{"WIL"}</p>
                            <p class="subtitle">{abilities.will}</p>
                        </div>
                    </div>
                    <div class="level-item"></div>
                </nav>
                <table class="table is-bordered is-fullwidth">
                    <tbody>
                        <tr>
                            <td><strong>{"Level 1"}</strong></td>
                            <td>{format!("4 max health, {}", &starting_feature)}</td>
                        </tr>
                    </tbody>
                </table>
                <p class="subtitle">{"Combat"}</p>
                <nav class="level">
                    <div class="level-item"></div>
                    <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{"ARMOR"}</p>
                            <p class="subtitle">{if can_use_shield { 9 } else { 8 }}</p>
                        </div>
                    </div>
                    <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{"ATTACK"}</p>
                            <p class="subtitle">{if starting_feature == Features::AttackBonus { "+1" } else { "0" }}</p>
                        </div>
                    </div>
                    <div class="level-item has-text-centered">
                        <div>
                            <p class="heading">{"HEALTH"}</p>
                            <p class="subtitle">{4}</p>
                        </div>
                    </div>
                    <div class="level-item"></div>
                </nav>
                <div class="content">
                    <ul>
                        <li>{"Light armor (+1 armor)"}</li>
                        <li>{
                            format!(
                                "Shield (+1 armor, 1 hand{})",
                                if can_use_shield { "" } else { ", inactive: no 1h weapon" }
                            )
                        }</li>
                        {starting_weapons.iter().map(|w| html! {<li>{w}</li>}).collect::<Html>()}
                    </ul>
                </div>
            </div>
            <div class="column">
                <p class="subtitle">{"Gear"}</p>
                <div class="content">
                    <ul>
                        {starting_items.iter().map(|item| html! {<li>{item}</li>}).collect::<Html>()}
                    </ul>
                </div>
                <p class="subtitle">{"Personal"}</p>
                <div class="content">
                    <ul>
                        {personal_traits.iter().map(|(t, v)| html! {
                            <li><strong>{format!("{}: ", t)}</strong>{v}</li>
                        }).collect::<Html>()}
                    </ul>
                </div>
            </div>
        </div>
    }
}

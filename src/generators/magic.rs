use std::rc::Rc;

use yew::prelude::*;

use crate::components::RerollButton;
use crate::roller::Rollable;

use super::monsters::{get_animal, get_monster_ability, get_monster_feature, get_monster_trait};
use super::npcs::get_personality;

pub fn get_physical_effect() -> &'static str {
    [
        [
            "Animating",
            "Attracting",
            "Binding",
            "Blossoming",
            "Consuming",
            "Creeping",
        ],
        [
            "Crushing",
            "Diminishing",
            "Dividing",
            "Duplicating",
            "Enveloping",
            "Expanding",
        ],
        [
            "Fusing",
            "Grasping",
            "Hastening",
            "Hindering",
            "illuminating",
            "Imprisoning",
        ],
        [
            "Levitating",
            "Opening",
            "Petrifying",
            "Phasing",
            "Piercing",
            "Pursuing",
        ],
        [
            "Reflecting",
            "Regenerating",
            "Rending",
            "Repelling",
            "Resurrecting",
            "Screaming",
        ],
        [
            "Sealing",
            "Shapeshifting",
            "Shielding",
            "Spawning",
            "Transmuting",
            "Transporting",
        ],
    ]
    .roll()
    .roll()
}

pub fn get_physical_element() -> &'static str {
    [
        ["Acid", "Amber", "Bark", "Blood", "Bone", "Brine"],
        ["Clay", "Crow", "Crystal", "Ember", "Flesh", "Fungus"],
        ["Glass", "Honey", "Ice", "Insect", "Wood", "Lava"],
        ["Moss", "Obsidian", "Oil", "Poison", "Rat", "Salt"],
        ["Sand", "Sap", "Serpent", "Slime", "Stone", "Tar"],
        ["Thorn", "Vine", "Water", "Wine", "Wood", "Worm"],
    ]
    .roll()
    .roll()
}

pub fn get_physical_form() -> &'static str {
    [
        ["Altar", "Armor", "Arrow", "Beast", "Blade", "Cauldron"],
        ["Chain", "Chariot", "Claw", "Cloak", "Colossus", "Crown"],
        ["Elemental", "Eye", "Fountain", "Gate", "Golem", "Hammer"],
        ["Horn", "Key", "Mask", "Monolith", "Pit", "Prison"],
        ["Sentinel", "Servant", "Shield", "Spear", "Steed", "Swarm"],
        ["Tentacle", "Throne", "Torch", "Trap", "Wall", "Web"],
    ]
    .roll()
    .roll()
}

pub fn get_ethereal_effect() -> &'static str {
    [
        [
            "Avenging",
            "Banishing",
            "Bewildering",
            "Blinding",
            "Charming",
            "Communicating",
        ],
        [
            "Compelling",
            "Concealing",
            "Deafening",
            "Deceiving",
            "Deciphering",
            "Disguising",
        ],
        [
            "Dispelling",
            "Emboldening",
            "Encoding",
            "Energizing",
            "Enlightening",
            "Enraging",
        ],
        [
            "Excruciating",
            "Foreseeing",
            "Intoxicating",
            "Maddening",
            "Mesmerizing",
            "Mindreading",
        ],
        [
            "Nullifying",
            "Paralyzing",
            "Revealing",
            "Revolting",
            "Scrying",
            "Silencing",
        ],
        [
            "Soothing",
            "Summoning",
            "Terrifying",
            "Warding",
            "Wearying",
            "Withering",
        ],
    ]
    .roll()
    .roll()
}

pub fn get_ethereal_element() -> &'static str {
    [
        ["Ash", "Chaos", "Distortion", "Dream", "Dust", "Echo"],
        ["Ectoplasm", "Fire", "Fog", "Ghost", "Harmony", "Heat"],
        [
            "Light",
            "Lightning",
            "Memory",
            "Mind",
            "Mutation",
            "Negation",
        ],
        ["Plague", "Plasma", "Probability", "Rain", "Rot", "Shadow"],
        ["Smoke", "Snow", "Soul", "Star", "Stasis", "Steam"],
        ["Thunder", "Time", "Void", "Warp", "Whisper", "Wind"],
    ]
    .roll()
    .roll()
}

fn get_ethereal_form() -> &'static str {
    [
        ["Aura", "Beacon", "Beam", "Blast", "Blob", "Bolt"],
        ["Bubble", "Call", "Cascade", "Circle", "Cloud", "Coil"],
        ["Cone", "Cube", "Dance", "Disk", "Field", "Form"],
        ["Gaze", "Loop", "Moment", "Nexus", "Portal", "Pulse"],
        ["Pyramid", "Ray", "Shard", "Sphere", "Spray", "Storm"],
        ["Swarm", "Torrent", "Touch", "Vortex", "Wave", "Word"],
    ]
    .roll()
    .roll()
}

pub fn get_spell() -> String {
    let decision_table: [[(&dyn Fn() -> &'static str, &dyn Fn() -> &'static str); 6]; 2] = [
        [
            (&get_physical_effect, &get_physical_form),
            (&get_physical_effect, &get_ethereal_form),
            (&get_ethereal_effect, &get_physical_form),
            (&get_ethereal_effect, &get_ethereal_form),
            (&get_physical_element, &get_physical_form),
            (&get_physical_element, &get_ethereal_form),
        ],
        [
            (&get_ethereal_element, &get_physical_form),
            (&get_ethereal_element, &get_ethereal_form),
            (&get_physical_effect, &get_physical_element),
            (&get_physical_effect, &get_ethereal_element),
            (&get_ethereal_effect, &get_physical_element),
            (&get_ethereal_effect, &get_ethereal_element),
        ],
    ];

    let (first, second) = decision_table.roll().roll();

    format!("{} {}", first().to_string(), second().to_string(),)
}

pub fn get_insanity() -> String {
    [
        [
            || "Always lies".to_string(),
            || "Always polite".to_string(),
            || format!("Belief: {}-form", get_animal()),
            || "Cannot count".to_string(),
            || "Cannot lie".to_string(),
            || "Faceblind".to_string(),
        ],
        [
            || "Fears birds".to_string(),
            || "Fears blood".to_string(),
            || "Fears books".to_string(),
            || "Fears darkness".to_string(),
            || "Fears fire".to_string(),
            || "Fears gold".to_string(),
        ],
        [
            || "Fears horses".to_string(),
            || "Fears iron".to_string(),
            || "Fears music".to_string(),
            || "Fears own hand".to_string(),
            || "Fears a player character".to_string(),
            || "Fears rain".to_string(),
        ],
        [
            || "Fears rivers".to_string(),
            || "Fears silence".to_string(),
            || "Fears sleep".to_string(),
            || "Fears sunlight".to_string(),
            || "Fears the moon".to_string(),
            || "Fears trees".to_string(),
        ],
        [
            || "Belief: is a genius".to_string(),
            || "Belief: is gorgeous".to_string(),
            || "Hates violence".to_string(),
            || "Belief: is invisible".to_string(),
            || "Belief: is invulnerable".to_string(),
            || format!("Belief: {}", get_monster_ability()),
        ],
        [
            || format!("Belief: {}", get_monster_feature()),
            || format!("Belief: {}", get_monster_trait()),
            || "Must sing".to_string(),
            || format!("New personality: {}", get_personality()),
            || "Says thoughts".to_string(),
            || "Sees dead people".to_string(),
        ],
    ]
    .roll()
    .roll()()
}

#[derive(Clone, PartialEq, Properties)]
pub struct MagicProps {
    pub spells: Rc<Vec<String>>,
    pub update: Callback<Rc<Vec<String>>>,
}

impl MagicProps {
    pub fn get_spells() -> Rc<Vec<String>> {
        Rc::new((0..10).map(|_| get_spell()).collect())
    }
}

#[function_component(Magic)]
pub fn magic(props: &MagicProps) -> Html {
    let reroll = {
        let update = props.update.clone();
        Callback::from(move |_| update.emit(MagicProps::get_spells()))
    };

    html! {
        <>
        <nav class="level">
            <h1 class="level-item title has-text-centered" style={"margin: 0px;"}>{"Spells"}</h1>
            <RerollButton onclick={reroll} />
        </nav>
            <div class="content">
                <ol>{
                    props.spells.iter().map(|spell| {
                        html! { <li>{spell}</li> }
                    }).collect::<Html>()
                }</ol>
            </div>
        </>
    }
}

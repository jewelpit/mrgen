use std::rc::Rc;

use yew::prelude::*;

use crate::components::RerollButton;
use crate::roller::Rollable;

pub fn get_spell() -> String {
    let physical_effects = [
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
    ];
    let physical_elements = [
        ["Acid", "Amber", "Bark", "Blood", "Bone", "Brine"],
        ["Clay", "Crow", "Crystal", "Ember", "Flesh", "Fungus"],
        ["Glass", "Honey", "Ice", "Insect", "Wood", "Lava"],
        ["Moss", "Obsidian", "Oil", "Poison", "Rat", "Salt"],
        ["Sand", "Sap", "Serpent", "Slime", "Stone", "Tar"],
        ["Thorn", "Vine", "Water", "Wine", "Wood", "Worm"],
    ];
    let physical_forms = [
        ["Altar", "Armor", "Arrow", "Beast", "Blade", "Cauldron"],
        ["Chain", "Chariot", "Claw", "Cloak", "Colossus", "Crown"],
        ["Elemental", "Eye", "Fountain", "Gate", "Golem", "Hammer"],
        ["Horn", "Key", "Mask", "Monolith", "Pit", "Prison"],
        ["Sentinel", "Servant", "Shield", "Spear", "Steed", "Swarm"],
        ["Tentacle", "Throne", "Torch", "Trap", "Wall", "Web"],
    ];
    let ethereal_effects = [
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
    ];
    let ethereal_elements = [
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
    ];
    let ethereal_forms = [
        ["Aura", "Beacon", "Beam", "Blast", "Blob", "Bolt"],
        ["Bubble", "Call", "Cascade", "Circle", "Cloud", "Coil"],
        ["Cone", "Cube", "Dance", "Disk", "Field", "Form"],
        ["Gaze", "Loop", "Moment", "Nexus", "Portal", "Pulse"],
        ["Pyramid", "Ray", "Shard", "Sphere", "Spray", "Storm"],
        ["Swarm", "Torrent", "Touch", "Vortex", "Wave", "Word"],
    ];

    let decision_table = [
        [
            (&physical_effects, &physical_forms),
            (&physical_effects, &ethereal_forms),
            (&ethereal_effects, &physical_forms),
            (&ethereal_effects, &ethereal_forms),
            (&physical_elements, &physical_forms),
            (&physical_elements, &ethereal_forms),
        ],
        [
            (&ethereal_elements, &physical_forms),
            (&ethereal_elements, &ethereal_forms),
            (&physical_effects, &physical_elements),
            (&physical_effects, &ethereal_elements),
            (&ethereal_effects, &physical_elements),
            (&ethereal_effects, &ethereal_elements),
        ],
    ];

    let (first, second) = decision_table.roll().roll();

    format!(
        "{} {}",
        first.roll().roll().to_string(),
        second.roll().roll().to_string(),
    )
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

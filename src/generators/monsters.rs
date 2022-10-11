use crate::roller::Rollable;

use super::magic::{
    get_ethereal_effect, get_ethereal_element, get_physical_effect, get_physical_element,
};

fn get_aerial_animal() -> &'static str {
    [
        [
            "Albatross",
            "Bat",
            "Beetle",
            "Bird of paradise",
            "Butterfly",
            "Condor",
        ],
        ["Crane", "Crow", "Dragonfly", "Eagle", "Falcon", "Firefly"],
        [
            "Flamingo",
            "Fly",
            "Flying squirrel",
            "Goose",
            "Gull",
            "Hummingbird",
        ],
        [
            "Kingfisher",
            "Locust",
            "Magpie",
            "Mantis",
            "Mockingbird",
            "Mosquito",
        ],
        ["Moth", "Owl", "Parrot", "Peacock", "Pelican", "Pteranodon"],
        [
            "Rooster",
            "Sparrow",
            "Swan",
            "Vulture",
            "Wasp",
            "Woodpecker",
        ],
    ]
    .roll()
    .roll()
}

fn get_terrestrial_animal() -> &'static str {
    [
        ["Ant", "Ape", "Armadillo", "Badger", "Bear", "Boar"],
        [
            "Caterpiller",
            "Centipede",
            "Chameleon",
            "Cockroach",
            "Deer",
            "Elephant",
        ],
        ["Ferret", "Fox", "Giraffe", "Goat", "Horse", "Human"],
        ["Mole", "Ostrich", "Ox", "Porcupine", "Rabbit", "Raccoon"],
        ["Rat", "Rhinocerous", "Scorpion", "Sheep", "Slug", "Snail"],
        ["Snake", "Spider", "Squirrel", "Tiger", "Wolf", "Wolverine"],
    ]
    .roll()
    .roll()
}

fn get_aquatic_animal() -> &'static str {
    [
        [
            "Alligator",
            "Amoeba",
            "Anglerfish",
            "Beaver",
            "Clam",
            "Crab",
        ],
        [
            "Dolphin",
            "Eel",
            "Frog",
            "Hippopotamus",
            "Jellyfish",
            "Leech",
        ],
        [
            "Lobster",
            "Manatee",
            "Manta ray",
            "Muskrat",
            "Narwhal",
            "Newt",
        ],
        [
            "Octopus",
            "Otter",
            "Penguin",
            "Platypus",
            "Pufferfish",
            "Salamander",
        ],
        [
            "Sea Anemonoe",
            "Sea urchin",
            "Seahorse",
            "Seal",
            "Shark",
            "Shrimp",
        ],
        ["Squid", "Swordfish", "Tadpole", "Turtle", "Walrus", "Whale"],
    ]
    .roll()
    .roll()
}

pub fn get_animal() -> &'static str {
    [
        get_aerial_animal,
        get_terrestrial_animal,
        get_aquatic_animal,
    ]
    .roll()()
}

pub fn get_monster_feature() -> &'static str {
    [
        [
            "Antlers",
            "Beak",
            "Carapace",
            "Claws",
            "Compound eyes",
            "Eye stalks",
        ],
        ["Fangs", "Fins", "Fur", "Gills", "Hooves", "Horns"],
        [
            "Legless",
            "Long tongue",
            "Many-eyed",
            "Many-limbed",
            "Mucus",
            "Pincers",
        ],
        [
            "Plates",
            "Plumage",
            "Proboscis",
            "Scales",
            "Segments",
            "Shaggy hair",
        ],
        [
            "Shell",
            "Spikes",
            "Spinnerets",
            "Spines",
            "Stinger",
            "Suction cups",
        ],
        ["Tail", "Talons", "Tentacles", "Trunk", "Tusks", "Wings"],
    ]
    .roll()
    .roll()
}

pub fn get_monster_trait() -> &'static str {
    [
        [
            || "Amphibious",
            || "Bloated",
            || "Brittle",
            || "Cannibal",
            || "Clay-like",
            || "Colossal",
        ],
        [
            || "Crystalline",
            || "Decaying",
            get_ethereal_element,
            || "Ethereal",
            || "Ever-young",
            || "Eyeless",
        ],
        [
            || "Fearless",
            || "Fluffy",
            || "Fungal",
            || "Gelatinous",
            || "Geometric",
            || "Hardened",
        ],
        [
            || "Illusory",
            || "Intelligent",
            || "Iridescent",
            || "Luminous",
            || "Many-headed",
            || "Mechanical",
        ],
        [
            get_physical_element,
            || "Planar",
            || "Reflective",
            || "Rubbery",
            || "Shadowy",
            || "Sharp",
        ],
        [
            || "Skeletal",
            || "Slimy",
            || "Sticky",
            || "Stinking",
            || "Tiny",
            || "Translucent",
        ],
    ]
    .roll()
    .roll()()
}

pub fn get_monster_ability() -> &'static str {
    [
        [
            || "Absorbing",
            || "Acid blood",
            || "Anti-magic",
            || "Blinding",
            || "Breath weapon",
            || "Camouflaging",
        ],
        [
            || "Duplicating",
            || "Electron",
            || "Entangling",
            get_ethereal_effect,
            || "Exploding",
            || "Flying",
        ],
        [
            || "Gaze weapon",
            || "Hypnotizing",
            || "Impervious",
            || "Invisible",
            || "Life-draining",
            || "Magnetic",
        ],
        [
            || "Mimicking",
            || "Mind-reading",
            || "Paralyzing",
            || "Phasing",
            get_physical_effect,
            || "Poisonous",
        ],
        [
            || "Radioactive",
            || "Reflective",
            || "Regenerating",
            || "Shapeshifting",
            || "Spell-casting",
            || "Stealthy",
        ],
        [
            || "Strangling",
            || "Super-strength",
            || "Telekinetic",
            || "Teleporting",
            || "Vampiric",
            || "Wall-crawling",
        ],
    ]
    .roll()
    .roll()()
}

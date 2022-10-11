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
            || format!("Leader of <{}>", get_faction()),
            || format!("Member of <{}>", get_faction()),
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
    [
        [
            || "A better life".to_string(),
            || "Acceptance".to_string(),
            || "Acquire a specific item".to_string(),
            || "Craft a specific item".to_string(),
            || format!("Destroy <{}>", get_faction()),
            || "Destroy a specific item".to_string(),
        ],
        [
            || "Enlightenment".to_string(),
            || "Fame".to_string(),
            || format!("Found <{}>", get_faction()),
            || "Freedom".to_string(),
            || "Glory".to_string(),
            || "Impress a specific NPC".to_string(),
        ],
        [
            || "Infamy".to_string(),
            || format!("Infiltrate <{}>", get_faction()),
            || "Justice".to_string(),
            || "Kidnap a specific NPC".to_string(),
            || format!("Lead <{}>", get_faction()),
            || "Learning".to_string(),
        ],
        [
            || "Locate a specific NPC".to_string(),
            || "Love".to_string(),
            || "Mastery".to_string(),
            || "Power".to_string(),
            || "Reach a location".to_string(),
            || "Rescue a specific NPC".to_string(),
        ],
        [
            || "Resolve a dispute".to_string(),
            || format!("Restore <{}>", get_faction()),
            || "Reveal a secret".to_string(),
            || "Revenge".to_string(),
            || format!("Sabotage <{}>", get_faction()),
            || "Serve a deity".to_string(),
        ],
        [
            || "Serve evil".to_string(),
            || format!("Serve <{}>", get_faction()),
            || "Serve an ideology".to_string(),
            || "Serve a leader".to_string(),
            || "Serve the needy".to_string(),
            || "Wealth".to_string(),
        ],
    ]
    .roll()
    .roll()()
}

fn get_misfortune() -> String {
    [
        [
            "Abandoned",
            "Addicted",
            "Arrested",
            "Blackmailed",
            "Burgled",
            "Challenged",
        ],
        [
            "Condemned",
            "Crippled",
            "Cursed",
            "Defrauded",
            "Demoted",
            "Depressed",
        ],
        [
            "Discredited",
            "Dismissed",
            "Disowned",
            "Exiled",
            "Famished",
            "Forgotten",
        ],
        [
            "Framed",
            "Haunted",
            "Humiliated",
            "Impoverished",
            "Kidnapped",
            "Lost",
        ],
        [
            "Mobbed",
            "Mutilated",
            "Overworked",
            "Poisoned",
            "Pursued",
            "Rejected",
        ],
        [
            "Replaced",
            "Robbed",
            "Sick",
            "Sued",
            "Suspected",
            "Transformed",
        ],
    ]
    .roll()
    .roll()
    .to_string()
}

fn get_mission() -> String {
    [
        [
            "Apprehend",
            "Assassinate",
            "Blackmail",
            "Burgle",
            "Chart",
            "Convince",
        ],
        [
            "Deface",
            "Defraud",
            "Deliver",
            "Destroy",
            "Discredit",
            "Escort",
        ],
        [
            "Exfiltrate",
            "Extort",
            "Follow",
            "Frame",
            "Impersonate",
            "Impress",
        ],
        [
            "Infiltrate",
            "Interrogate",
            "investigate",
            "Kidnap",
            "Locate",
            "Plant",
        ],
        ["Protect", "Raid", "Replace", "Retrieve", "Rob", "Ruin"],
        [
            "Sabotage",
            "Smuggle",
            "Surveil",
            "Take over",
            "Terrorize",
            "Threaten",
        ],
    ]
    .roll()
    .roll()
    .to_string()
}

fn get_methods() -> String {
    [
        [
            "Alchemy",
            "Blackmail",
            "Bluster",
            "Bribery",
            "Bullying",
            "Bureaucracy",
        ],
        [
            "Charm",
            "Commerce",
            "Cronies",
            "Debate",
            "Deceit",
            "Deduction",
        ],
        [
            "Eloquence",
            "Espionage",
            "Fast-talking",
            "Favors",
            "Hard work",
            "Humor",
        ],
        [
            "Investigation",
            "Legal maneuvers",
            "Manipulation",
            "Misdirection",
            "Money",
            "Nagging",
        ],
        [
            "Negotiation",
            "Persistence",
            "Piety",
            "Preparation",
            "Quick wit",
            "Research",
        ],
        [
            "Rumors", "Sabotage", "Teamwork", "Theft", "Threats", "Violence",
        ],
    ]
    .roll()
    .roll()
    .to_string()
}

fn get_appearance() -> String {
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
    .to_string()
}

fn get_physical_details() -> String {
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
            "Bronze skinned",
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
    .to_string()
}

fn get_clothing() -> String {
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
            "Patterened",
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
    .to_string()
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
    .to_string()
}

fn get_secret() -> String {
    [
        [
            || "Addicted".to_string(),
            || "Artificial".to_string(),
            || "Assassin".to_string(),
            || "Bankrupt".to_string(),
            || "Beholden".to_string(),
            || "Counterspy".to_string(),
        ],
        [
            || "Cultist".to_string(),
            || "Demigod".to_string(),
            || "Evil lineage".to_string(),
            || "Exile".to_string(),
            || "Fence".to_string(),
            || "Fugitive".to_string(),
        ],
        [
            || "Ghost".to_string(),
            || "Has a child".to_string(),
            || "Heretic".to_string(),
            || "High born".to_string(),
            || "Huge fortune".to_string(),
            || "Illusion".to_string(),
        ],
        [
            || "Insurrectionist".to_string(),
            || "Low born".to_string(),
            || "Married".to_string(),
            || "Mind-controlled".to_string(),
            get_misfortune,
            || "Monster hunter".to_string(),
        ],
        [
            || "Non-human".to_string(),
            || "A specific NPC".to_string(),
            || "Polygamist".to_string(),
            || "Protects relic".to_string(),
            || "Scandalous birth".to_string(),
            || "Secret police".to_string(),
        ],
        [
            || "Serial killer".to_string(),
            || "Smuggler".to_string(),
            || "Spy".to_string(),
            || "Time traveler".to_string(),
            || "Transformed".to_string(),
            || "War criminal".to_string(),
        ],
    ]
    .roll()
    .roll()()
}

fn get_reputation() -> String {
    [
        [
            "Ambitious",
            "Authoritative",
            "Boor",
            "Borrower",
            "Celebrity",
            "Charitable",
        ],
        [
            "Cheat",
            "Dangerous",
            "Entertainer",
            "Gossip",
            "Hardworking",
            "Holy",
        ],
        [
            "Honest",
            "Hypochondriac",
            "Idiot",
            "Influential",
            "Layabout",
            "Leader",
        ],
        [
            "Misanthrope",
            "Misanthrope",
            "Neighborly",
            "Nutjob",
            "Obnoxious",
            "Overeducated",
        ],
        [
            "Partier",
            "Pious",
            "Proper",
            "Prophet of doom",
            "Repulsive",
            "Respected",
        ],
        [
            "Riffraff",
            "Scandalous",
            "Slime ball",
            "Terrifying",
            "Weirdo",
            "Wise",
        ],
    ]
    .roll()
    .roll()
    .to_string()
}

fn get_hobby() -> String {
    [
        [
            "Archaeology",
            "Art collecting",
            "Bad fiction",
            "Calligraphy",
            "Card games",
            "Clockwork",
        ],
        [
            "Collecting cats",
            "Cuisine",
            "Dark lore",
            "Dog breeding",
            "Embroidery",
            "Exercise",
        ],
        [
            "Falconry",
            "Fashion",
            "Fishing",
            "Foreign cultures",
            "Gardening",
            "History",
        ],
        [
            "Horseracing",
            "Hunting",
            "Instrument",
            "Knitting",
            "Lawn games",
            "Mountaineering",
        ],
        [
            "Opera",
            "Painting",
            "Poetry",
            "Puzzle-solving",
            "Riddling",
            "Science",
        ],
        [
            "Sculpture",
            "Sketching",
            "Smoking",
            "Theater",
            "Weaving",
            "Whiskey",
        ],
    ]
    .roll()
    .roll()
    .to_string()
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
                                            <strong>{"Reputation: "}</strong>{&npc.reputation}<br />
                                            <strong>{"Asset: "}</strong>{&npc.asset}<br />
                                            <strong>{"Liability: "}</strong>{&npc.liability}<br />
                                            <strong>{"Goal: "}</strong>{&npc.goal}<br />
                                        </div>
                                        <div class="column">
                                            <strong>{"Misfortune: "}</strong>{&npc.misfortune}<br />
                                            <strong>{"Mission: "}</strong>{&npc.mission}<br />
                                            <strong>{"Methods: "}</strong>{&npc.methods}<br />
                                            <strong>{"Appearance: "}</strong>{&npc.appearance}<br />
                                            <strong>{"Physical details: "}</strong>{&npc.physical_details}<br />
                                        </div>
                                        <div class="column">
                                            <strong>{"Clothing: "}</strong>{&npc.clothing}<br />
                                            <strong>{"Personality: "}</strong>{&npc.personality}<br />
                                            <strong>{"Mannerisms: "}</strong>{&npc.mannerisms}<br />
                                            <strong>{"Secret: "}</strong>{&npc.secret}<br />
                                            <strong>{"Hobby: "}</strong>{&npc.hobby}<br />
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

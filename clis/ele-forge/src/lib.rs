use std::fmt;
use std::str;

const MIN_COST: u32 = 10000;

pub trait CraftingCost {
    fn cost(&self, forge: &Forge) -> u32;
}

pub trait ForgePrice {
    fn price(&self) -> Option<u32>;
}

#[derive(Debug, Clone, Copy)]
pub enum Ele {
    Air,
    Earth,
    Fire,
    Water,
    Pressure,
    Rain,
    Smoke,
    Dust,
    Fog,
    Wind,
    Heat,
    Life,
    Energy,
    Lava,
    Steam,
    Mud,
    Time,
    Glass,
    Worm,
    Oil,
    Sun,
    Ocean,
    Mountain,
    Metal,
    Pond,
    Seed,
    Plastic,
    Umbrella,
}

impl Ele {
    pub fn price(&self) -> Option<u32> {
        match self {
            Ele::Air => Some(10000),
            Ele::Earth => Some(10000),
            Ele::Fire => Some(10000),
            Ele::Water => Some(10000),
            Ele::Pressure => Some(42000),
            Ele::Rain => Some(42000),
            Ele::Smoke => Some(42000),
            Ele::Dust => Some(42000),
            Ele::Fog => Some(42000),
            Ele::Wind => Some(42000),
            Ele::Heat => Some(42000),
            Ele::Life => Some(42000),
            Ele::Energy => Some(42000),
            Ele::Lava => Some(42000),
            Ele::Steam => Some(42000),
            Ele::Mud => Some(42000),
            _ => None,
        }
    }
}

impl CraftingCost for Ele {
    fn cost(&self, forge: &Forge) -> u32 {
        if let Some(recipe) = forge.find_recipe(&format!("{}", self)) {
            recipe.cost(forge)
        } else {
            MIN_COST
        }
    }
}

impl fmt::Display for Ele {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ele::Air => write!(f, "Air"),
            Ele::Earth => write!(f, "Earth"),
            Ele::Fire => write!(f, "Fire"),
            Ele::Water => write!(f, "Water"),
            Ele::Pressure => write!(f, "Pressure"),
            Ele::Rain => write!(f, "Rain"),
            Ele::Smoke => write!(f, "Smoke"),
            Ele::Dust => write!(f, "Dust"),
            Ele::Fog => write!(f, "Fog"),
            Ele::Wind => write!(f, "Wind"),
            Ele::Heat => write!(f, "Heat"),
            Ele::Life => write!(f, "Life"),
            Ele::Energy => write!(f, "Energy"),
            Ele::Lava => write!(f, "Lava"),
            Ele::Steam => write!(f, "Steam"),
            Ele::Mud => write!(f, "Mud"),
            Ele::Time => write!(f, "Time"),
            Ele::Glass => write!(f, "Glass"),
            Ele::Worm => write!(f, "Worm"),
            Ele::Oil => write!(f, "Oil"),
            Ele::Sun => write!(f, "Sun"),
            Ele::Ocean => write!(f, "Ocean"),
            Ele::Mountain => write!(f, "Mountain"),
            Ele::Metal => write!(f, "Metal"),
            Ele::Pond => write!(f, "Pond"),
            Ele::Seed => write!(f, "Seed"),
            Ele::Plastic => write!(f, "Plastic"),
            Ele::Umbrella => write!(f, "Umbrella"),
        }
    }
}

impl str::FromStr for Ele {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Air" => Ok(Ele::Air),
            "Earth" => Ok(Ele::Earth),
            "Fire" => Ok(Ele::Fire),
            "Water" => Ok(Ele::Water),
            "Pressure" => Ok(Ele::Pressure),
            "Rain" => Ok(Ele::Rain),
            "Smoke" => Ok(Ele::Smoke),
            "Dust" => Ok(Ele::Dust),
            "Fog" => Ok(Ele::Fog),
            "Wind" => Ok(Ele::Wind),
            "Heat" => Ok(Ele::Heat),
            "Life" => Ok(Ele::Life),
            "Energy" => Ok(Ele::Energy),
            "Lava" => Ok(Ele::Lava),
            "Steam" => Ok(Ele::Steam),
            "Mud" => Ok(Ele::Mud),
            "Time" => Ok(Ele::Time),
            "Glass" => Ok(Ele::Glass),
            "Worm" => Ok(Ele::Worm),
            "Oil" => Ok(Ele::Oil),
            "Sun" => Ok(Ele::Sun),
            "Ocean" => Ok(Ele::Ocean),
            "Mountain" => Ok(Ele::Mountain),
            "Metal" => Ok(Ele::Metal),
            "Pond" => Ok(Ele::Pond),
            "Seed" => Ok(Ele::Seed),
            "Plastic" => Ok(Ele::Plastic),
            "Umbrella" => Ok(Ele::Umbrella),
            _ => Err(()),
        }
    }
}

type Recipe = [Ele; 4];

impl CraftingCost for Recipe {
    fn cost(&self, forge: &Forge) -> u32 {
        self.iter().map(|&ele| ele.cost(forge)).sum()
    }
}

impl ForgePrice for Recipe {
    fn price(&self) -> Option<u32> {
        let prices: Option<Vec<u32>> = self.iter().map(|&ele| ele.price()).collect();
        prices.map(|p| p.iter().sum())
    }
}

#[derive(Debug)]
enum Recipes {
    Pressure(Recipe),
    Rain(Recipe),
    Smoke(Recipe),
    Dust(Recipe),
    Fog(Recipe),
    Wind(Recipe),
    Heat(Recipe),
    Life(Recipe),
    Energy(Recipe),
    Lava(Recipe),
    Steam(Recipe),
    Mud(Recipe),
    Time(Recipe),
    Glass(Recipe),
    Worm(Recipe),
    Oil(Recipe),
    Sun(Recipe),
    Ocean(Recipe),
    Mountain(Recipe),
    Metal(Recipe),
    Pond(Recipe),
    Seed(Recipe),
    Plastic(Recipe),
    Umbrella(Recipe),
}

impl fmt::Display for Recipes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Recipes::Pressure(_) => write!(f, "Pressure"),
            Recipes::Rain(_) => write!(f, "Rain"),
            Recipes::Smoke(_) => write!(f, "Smoke"),
            Recipes::Dust(_) => write!(f, "Dust"),
            Recipes::Fog(_) => write!(f, "Fog"),
            Recipes::Wind(_) => write!(f, "Wind"),
            Recipes::Heat(_) => write!(f, "Heat"),
            Recipes::Life(_) => write!(f, "Life"),
            Recipes::Energy(_) => write!(f, "Energy"),
            Recipes::Lava(_) => write!(f, "Lava"),
            Recipes::Steam(_) => write!(f, "Steam"),
            Recipes::Mud(_) => write!(f, "Mud"),
            Recipes::Time(_) => write!(f, "Time"),
            Recipes::Glass(_) => write!(f, "Glass"),
            Recipes::Worm(_) => write!(f, "Worm"),
            Recipes::Oil(_) => write!(f, "Oil"),
            Recipes::Sun(_) => write!(f, "Sun"),
            Recipes::Ocean(_) => write!(f, "Ocean"),
            Recipes::Mountain(_) => write!(f, "Mountain"),
            Recipes::Metal(_) => write!(f, "Metal"),
            Recipes::Pond(_) => write!(f, "Pond"),
            Recipes::Seed(_) => write!(f, "Seed"),
            Recipes::Plastic(_) => write!(f, "Plastic"),
            Recipes::Umbrella(_) => write!(f, "Umbrella"),
        }
    }
}

const FORGE_RECIPES: [Recipes; 24] = [
    Recipes::Pressure([Ele::Air, Ele::Earth, Ele::Earth, Ele::Earth]),
    Recipes::Rain([Ele::Air, Ele::Air, Ele::Water, Ele::Water]),
    Recipes::Smoke([Ele::Air, Ele::Fire, Ele::Earth, Ele::Fire]),
    Recipes::Dust([Ele::Air, Ele::Air, Ele::Earth, Ele::Earth]),
    Recipes::Fog([Ele::Air, Ele::Air, Ele::Water, Ele::Earth]),
    Recipes::Wind([Ele::Air, Ele::Air, Ele::Air, Ele::Air]),
    Recipes::Heat([Ele::Fire, Ele::Air, Ele::Fire, Ele::Fire]),
    Recipes::Life([Ele::Air, Ele::Fire, Ele::Earth, Ele::Water]),
    Recipes::Energy([Ele::Air, Ele::Water, Ele::Water, Ele::Water]),
    Recipes::Lava([Ele::Fire, Ele::Fire, Ele::Fire, Ele::Earth]),
    Recipes::Steam([Ele::Water, Ele::Water, Ele::Fire, Ele::Water]),
    Recipes::Mud([Ele::Water, Ele::Water, Ele::Earth, Ele::Earth]),
    Recipes::Time([Ele::Dust, Ele::Dust, Ele::Dust, Ele::Dust]),
    Recipes::Glass([Ele::Pressure, Ele::Heat, Ele::Earth, Ele::Fire]),
    Recipes::Worm([Ele::Earth, Ele::Earth, Ele::Life, Ele::Mud]),
    Recipes::Oil([Ele::Pressure, Ele::Life, Ele::Life, Ele::Earth]),
    Recipes::Sun([Ele::Fire, Ele::Energy, Ele::Lava, Ele::Heat]),
    Recipes::Ocean([Ele::Water, Ele::Water, Ele::Water, Ele::Life]),
    Recipes::Mountain([Ele::Earth, Ele::Energy, Ele::Earth, Ele::Earth]),
    Recipes::Metal([Ele::Heat, Ele::Heat, Ele::Pressure, Ele::Earth]),
    Recipes::Pond([Ele::Earth, Ele::Rain, Ele::Water, Ele::Water]),
    Recipes::Seed([Ele::Rain, Ele::Life, Ele::Earth, Ele::Earth]),
    Recipes::Plastic([Ele::Oil, Ele::Oil, Ele::Pressure, Ele::Heat]),
    Recipes::Umbrella([Ele::Plastic, Ele::Plastic, Ele::Rain, Ele::Metal]),
];

pub struct Forge {
    recipes: [Recipes; 24],
}

impl Forge {
    pub fn find_recipe(&self, name: &str) -> Option<&Recipe> {
        for recipe in &self.recipes {
            match recipe {
                Recipes::Pressure(_) if name == "Pressure" => {
                    if let Recipes::Pressure(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Rain(_) if name == "Rain" => {
                    if let Recipes::Rain(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Smoke(_) if name == "Smoke" => {
                    if let Recipes::Smoke(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Dust(_) if name == "Dust" => {
                    if let Recipes::Dust(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Fog(_) if name == "Fog" => {
                    if let Recipes::Fog(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Wind(_) if name == "Wind" => {
                    if let Recipes::Wind(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Heat(_) if name == "Heat" => {
                    if let Recipes::Heat(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Life(_) if name == "Life" => {
                    if let Recipes::Life(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Energy(_) if name == "Energy" => {
                    if let Recipes::Energy(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Lava(_) if name == "Lava" => {
                    if let Recipes::Lava(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Steam(_) if name == "Steam" => {
                    if let Recipes::Steam(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Mud(_) if name == "Mud" => {
                    if let Recipes::Mud(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Time(_) if name == "Time" => {
                    if let Recipes::Time(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Glass(_) if name == "Glass" => {
                    if let Recipes::Glass(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Worm(_) if name == "Worm" => {
                    if let Recipes::Worm(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Oil(_) if name == "Oil" => {
                    if let Recipes::Oil(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Sun(_) if name == "Sun" => {
                    if let Recipes::Sun(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Ocean(_) if name == "Ocean" => {
                    if let Recipes::Ocean(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Mountain(_) if name == "Mountain" => {
                    if let Recipes::Mountain(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Metal(_) if name == "Metal" => {
                    if let Recipes::Metal(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Pond(_) if name == "Pond" => {
                    if let Recipes::Pond(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Seed(_) if name == "Seed" => {
                    if let Recipes::Seed(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Plastic(_) if name == "Plastic" => {
                    if let Recipes::Plastic(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Umbrella(_) if name == "Umbrella" => {
                    if let Recipes::Umbrella(elements) = recipe {
                        return Some(elements);
                    }
                }
                _ => {}
            }
        }
        None
    }
}

impl Default for Forge {
    fn default() -> Self {
        Forge {
            recipes: FORGE_RECIPES,
        }
    }
}

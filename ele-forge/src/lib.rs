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
    Rain,
    Heat,
    Pressure,
    Life,
    Oil,
    Metal,
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
            Ele::Rain => Some(42000),
            Ele::Heat => Some(42000),
            Ele::Pressure => Some(42000),
            Ele::Life => Some(42000),
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
            Ele::Rain => write!(f, "Rain"),
            Ele::Heat => write!(f, "Heat"),
            Ele::Pressure => write!(f, "Pressure"),
            Ele::Life => write!(f, "Life"),
            Ele::Oil => write!(f, "Oil"),
            Ele::Metal => write!(f, "Metal"),
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
            "Rain" => Ok(Ele::Rain),
            "Heat" => Ok(Ele::Heat),
            "Pressure" => Ok(Ele::Pressure),
            "Life" => Ok(Ele::Life),
            "Oil" => Ok(Ele::Oil),
            "Metal" => Ok(Ele::Metal),
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
    Rain(Recipe),
    Pressure(Recipe),
    Life(Recipe),
    Heat(Recipe),
    Oil(Recipe),
    Metal(Recipe),
    Plastic(Recipe),
    Umbrella(Recipe),
}

impl fmt::Display for Recipes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Recipes::Rain(_) => write!(f, "Rain"),
            Recipes::Pressure(_) => write!(f, "Pressure"),
            Recipes::Life(_) => write!(f, "Life"),
            Recipes::Heat(_) => write!(f, "Heat"),
            Recipes::Oil(_) => write!(f, "Oil"),
            Recipes::Metal(_) => write!(f, "Metal"),
            Recipes::Plastic(_) => write!(f, "Plastic"),
            Recipes::Umbrella(_) => write!(f, "Umbrella"),
        }
    }
}

const FORGE_RECIPES: [Recipes; 8] = [
    Recipes::Rain([Ele::Air, Ele::Air, Ele::Water, Ele::Water]),
    Recipes::Pressure([Ele::Air, Ele::Earth, Ele::Earth, Ele::Earth]),
    Recipes::Life([Ele::Air, Ele::Fire, Ele::Earth, Ele::Water]),
    Recipes::Heat([Ele::Fire, Ele::Air, Ele::Fire, Ele::Fire]),
    Recipes::Oil([Ele::Pressure, Ele::Life, Ele::Life, Ele::Earth]),
    Recipes::Metal([Ele::Heat, Ele::Heat, Ele::Pressure, Ele::Earth]),
    Recipes::Plastic([Ele::Oil, Ele::Oil, Ele::Pressure, Ele::Heat]),
    Recipes::Umbrella([Ele::Plastic, Ele::Plastic, Ele::Rain, Ele::Metal]),
];

pub struct Forge {
    recipes: [Recipes; 8],
}

impl Forge {
    pub fn find_recipe(&self, name: &str) -> Option<&Recipe> {
        for recipe in &self.recipes {
            match recipe {
                Recipes::Rain(_) if name == "Rain" => {
                    if let Recipes::Rain(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Pressure(_) if name == "Pressure" => {
                    if let Recipes::Pressure(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Life(_) if name == "Life" => {
                    if let Recipes::Life(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Heat(_) if name == "Heat" => {
                    if let Recipes::Heat(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Oil(_) if name == "Oil" => {
                    if let Recipes::Oil(elements) = recipe {
                        return Some(elements);
                    }
                }
                Recipes::Metal(_) if name == "Metal" => {
                    if let Recipes::Metal(elements) = recipe {
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

// Define a struct to handle all CSV entires
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Enum to handle Power/Toughness/Defense of cards
/// These values can either be an int, or a combination of strings
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum CardLayers {
    Int(u64),
    String(String),
}

/// Enum to handle the variation tag
/// Variations can be an int, or a combination of strings
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum CardVariations {
    Int(u64),
    String(String),
}

/// Enum to handle the converted manacost section
/// Variations can be an int, or a combination of strings
/// Some cards have multiple types of CMC values. For example, split cards
/// can have a value of 1 or 2 depending on the mode of spell(s) chosen
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ConvertedManacostVariations {
    Int(u64),
    String(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entry {
    name: Option<String>,
    set: Option<String>,
    set_code: Option<String>,
    id: Option<u64>,
    #[serde(rename = "type")]
    card_type: Option<String>,
    power: Option<CardLayers>,
    toughness: Option<CardLayers>,
    loyalty: Option<u32>,
    defense: Option<CardLayers>,
    manacost: Option<String>,
    converted_manacost: Option<ConvertedManacostVariations>,
    artist: Option<String>,
    flavor: Option<String>,
    color: Option<String>,
    generated_mana: Option<String>,
    number: Option<u64>,
    rarity: Option<String>,
    rating: Option<u64>,
    ruling: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    variation: Option<CardVariations>,
    #[serde(deserialize_with = "csv::invalid_option")]
    variation_local: Option<CardVariations>,
    ability: Option<String>,
    #[serde(rename = "pricing_EUR")]
    pricing_euro: Option<String>,
    #[serde(rename = "pricing_USD")]
    pricing_usd: Option<String>,
    #[serde(rename = "pricing_TXT")] // MTGO currency
    pricing_txt: Option<String>,
    watermark: Option<String>,
    print_number: Option<u32>,
    is_original: Option<u32>,
    back_id: Option<u32>,
    number_int: Option<u32>,

    #[serde(rename = "name_CN")]
    name_cn: Option<String>,
    #[serde(rename = "name_TW")]
    name_tw: Option<String>,
    #[serde(rename = "name_FR")]
    name_fr: Option<String>,
    #[serde(rename = "name_DE")]
    name_de: Option<String>,
    #[serde(rename = "name_IT")]
    name_it: Option<String>,
    #[serde(rename = "name_JP")]
    name_jp: Option<String>,
    #[serde(rename = "name_PT")]
    name_pt: Option<String>,
    #[serde(rename = "name_RU")]
    name_ru: Option<String>,
    #[serde(rename = "name_ES")]
    name_es: Option<String>,
    #[serde(rename = "name_KO")]
    name_ko: Option<String>,

    #[serde(rename = "type_CN")]
    type_cn: Option<String>,
    #[serde(rename = "type_TW")]
    type_tw: Option<String>,
    #[serde(rename = "type_FR")]
    type_fr: Option<String>,
    #[serde(rename = "type_DE")]
    type_de: Option<String>,
    #[serde(rename = "type_it")]
    type_it: Option<String>,
    #[serde(rename = "type_JP")]
    type_jp: Option<String>,
    #[serde(rename = "type_PT")]
    type_pt: Option<String>,
    #[serde(rename = "type_RU")]
    type_ru: Option<String>,
    #[serde(rename = "type_ES")]
    type_es: Option<String>,
    #[serde(rename = "type_KO")]
    type_ko: Option<String>,

    #[serde(rename = "ability_CN")]
    ability_cn: Option<String>,
    #[serde(rename = "ability_TW")]
    ability_tw: Option<String>,
    #[serde(rename = "ability_FR")]
    ability_fr: Option<String>,
    #[serde(rename = "ability_DE")]
    ability_de: Option<String>,
    #[serde(rename = "ability_IT")]
    ability_it: Option<String>,
    #[serde(rename = "ability_JP")]
    ability_jp: Option<String>,
    #[serde(rename = "ability_PT")]
    ability_pt: Option<String>,
    #[serde(rename = "ability_RU")]
    ability_ru: Option<String>,
    #[serde(rename = "ability_ES")]
    ability_es: Option<String>,
    #[serde(rename = "ability_KO")]
    ability_ko: Option<String>,

    #[serde(rename = "flavor_CN")]
    flavor_cn: Option<String>,
    #[serde(rename = "flavor_TW")]
    flavor_tw: Option<String>,
    #[serde(rename = "flavor_FR")]
    flavor_fr: Option<String>,
    #[serde(rename = "flavor_DE")]
    flavor_de: Option<String>,
    #[serde(rename = "flavor_IT")]
    flavor_it: Option<String>,
    #[serde(rename = "flavor_JP")]
    flavor_jp: Option<String>,
    #[serde(rename = "flavor_PT")]
    flavor_pt: Option<String>,
    #[serde(rename = "flavor_RU")]
    flavor_ru: Option<String>,
    #[serde(rename = "flavor_ES")]
    flavor_es: Option<String>,
    #[serde(rename = "flavor_KO")]
    flavor_ko: Option<String>,

    color_identify: Option<String>,
    fully_handled: Option<String>,
    custom_sort: Option<String>,
    is_special: Option<String>,
    card_status: Option<String>,
    associated_id: Option<String>,

    #[serde(rename = "legality_Standard")]
    legality_standard: Option<String>,
    #[serde(rename = "legality_Pioneer")]
    legality_pioneer: Option<String>,
    #[serde(rename = "legality_Modern")]
    legality_modern: Option<String>,
    #[serde(rename = "legality_Legacy")]
    legality_legacy: Option<String>,
    #[serde(rename = "legality_Vintage")]
    legality_vintage: Option<String>,
    #[serde(rename = "legality_Highlander")]
    legality_highlander: Option<String>,
    #[serde(rename = "legality_Duel_Commander")]
    legality_duel_commander: Option<String>,
    #[serde(rename = "legality_Tiny_leaders_Commander")]
    legality_tiny_leaders_commander: Option<String>,
    #[serde(rename = "legality_Commander")]
    legality_commander: Option<String>,
    #[serde(rename = "legality_Peasant")]
    legality_peasant: Option<String>,
    #[serde(rename = "legality_Pauper")]
    legality_pauper: Option<String>,
}

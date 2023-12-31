use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PartOfSpeech {
    // Adjectives
    Adjective(AdjectiveType),

    // Adverb
    Adverb,
    AdverbTo,

    // Auxilary
    Auxilary,
    AuxilaryAdj,
    AuxilaryVerb,

    // Other
    Conjunction,
    Counter,
    Expr,
    Interjection,

    Noun(NounType),

    Numeric,
    Pronoun,
    Prefix,
    Suffix,
    Particle,
    Unclassified,

    Sfx,

    // Verb
    Verb(VerbType),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VerbType {
    Nidan(NidanVerb),
    Yodan(VerbEnding),
    Godan(GodanVerbEnding),
    Irregular(IrregularVerb),
    Unspecified,
    Intransitive,
    Transitive,
    Ichidan,
    IchidanZuru,
    IchidanKureru,
    Kuru,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AdjectiveType {
    PreNounVerb,
    /// I Adjective
    Keiyoushi,
    /// I Adjective conjugated like いい
    KeiyoushiYoiIi,
    Ku,
    Na,
    Nari,
    No,
    PreNoun,
    Shiku,
    Taru,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NounType {
    Normal,
    Adverbial,
    Prefix,
    Suffix,
    Temporal,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IrregularVerb {
    Nu,
    Ru,
    NounOrAuxSuru,
    Suru,
    SuruSpecial,
    Su,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NidanVerb {
    class: VerbClass,
    ending: VerbEnding,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VerbClass {
    Upper,
    Lower,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VerbEnding {
    Bu,
    Dzu,
    Gu,
    Hu,
    Ku,
    Mu,
    Nu,
    Ru,
    Su,
    Tsu,
    U,
    Yu,
    Zu,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GodanVerbEnding {
    Bu,
    Gu,
    Ku,
    Mu,
    Nu,
    Ru,
    Su,
    Tsu,
    U,

    Aru,
    USpecial,
    Uru,
    RuIrreg,
    IkuYuku,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Misc {
    Ship,
    Abbreviation,
    Archaism,
    Character,
    ChildrensLanguage,
    Colloquialism,
    CompanyName,
    Creature,
    DatedTerm,
    Deity,
    Derogatory,
    Document,
    Event,
    Euphemistic,
    FamiliarLanguage,
    FemaleTermOrLanguage,
    Fiction,
    GivenName,
    Group,
    HistoricalTerm,
    HonorificLanguage,
    HumbleLanguage,
    IdiomaticExpression,
    JocularHumorousTerm,
    Legend,
    LiteraryOrFormalTerm,
    MangaSlang,
    MaleTermOrLanguage,
    Mythology,
    InternetSlang,
    Object,
    ObsoleteTerm,
    ObscureTerm,
    OnomatopoeicOrMimeticWord,
    OrganizationName,
    Other,
    Personname,
    PlaceName,
    PoeticalTerm,
    PoliteLanguage,
    ProductName,
    Proverb,
    Quotation,
    Rare,
    Religion,
    Sensitive,
    Service,
    Slang,
    RailwayStation,
    FamilyOrSurname,
    UsuallyWrittenInKana,
    UnclassifiedName,
    VulgarExpressionOrWord,
    ArtWork,
    RudeOrXRatedTerm,
    Yojijukugo,
}

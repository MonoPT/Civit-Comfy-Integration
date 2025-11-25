use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum NSFW {
    None,
    Soft,
    Mature,
    X
}

#[derive(Debug, Deserialize)]
pub enum Sort {
    MostReactions,
    MostComments,
    Newest,
}

#[derive(Debug, Deserialize)]
pub enum Period {
    AllTime,
    Year,
    Month,
    Week,
    Day
}
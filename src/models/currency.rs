use serde::Serialize;

#[derive(
    Debug,
    Clone,
    Hash,
    Eq,
    PartialEq,
    Serialize,
)]

pub enum Currency {

    USD,

    EUR,

    GBP,

    JPY,
}
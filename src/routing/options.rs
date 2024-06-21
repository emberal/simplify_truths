use serde::Deserialize;
use crate::expressions::truth_table::{Hide, Sort};
use crate::utils::serialize::{ret_true, deserialize_bool};

// TODO deserialize_bool should not be necessary
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimplifyOptions {
    #[serde(
        default = "ret_true",
        deserialize_with = "deserialize_bool"
    )]
    pub simplify: bool,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub ignore_case: bool,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TruthTableOptions {
    #[serde(default)]
    pub sort: Sort,
    #[serde(default)]
    pub hide: Hide,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub hide_intermediate_steps: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimplifyAndTableOptions {
    #[serde(flatten)]
    pub simplify_options: SimplifyOptions,
    #[serde(flatten)]
    pub table_options: TruthTableOptions,
}

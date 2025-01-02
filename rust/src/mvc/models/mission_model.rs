use super::{FrequencyPresetModel, PowerPresetModel};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct MissionModel {
    pub mission_name: String,
    pub mission_desc: String,
    pub mission_id: i32,
    // pub frequency_preset: String,
    // pub power_preset: String,
    pub frequency_model: FrequencyPresetModel,
    pub power_model: PowerPresetModel,
    pub flagged: bool,

    // creation date in milliseconds i64
}

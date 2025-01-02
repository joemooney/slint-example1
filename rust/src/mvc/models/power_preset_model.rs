use super::PowerModel;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct PowerPresetModel {
    pub power_preset_name: String,
    // pub power_preset_id: usize,
    pub power_preset_desc: String,
    pub values: PowerModel,
}

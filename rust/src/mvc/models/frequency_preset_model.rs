use super::FrequencyModel;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct FrequencyPresetModel {
    // pub preset_id: usize,
    pub frequency_preset_name: String,
    pub frequency_preset_desc: String,
    pub values: FrequencyModel,
}

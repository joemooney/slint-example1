// use std::{cell::RefCell, collections::HashMap, rc::Rc};
use std::{cell::RefCell, rc::Rc};

use super::traits;
use crate::mvc;

#[derive(Clone)]
pub struct MockFrequencyPresetRepository {
    // frequency_presets: Rc<RefCell<HashMap<String, mvc::FrequencyPresetModel>>>,
    frequency_presets: Rc<RefCell<Vec<mvc::FrequencyPresetModel>>>,
}

impl MockFrequencyPresetRepository {
    // pub fn new(frequency_presets: HashMap<String, mvc::FrequencyPresetModel>) -> Self {
    pub fn new(frequency_presets: Vec<mvc::FrequencyPresetModel>) -> Self {
        Self { frequency_presets: Rc::new(RefCell::new(frequency_presets)) }
    }
}

impl traits::FrequencyPresetRepository for MockFrequencyPresetRepository {
    fn frequency_preset_count(&self) -> usize {
        self.frequency_presets.borrow().len()
    }

    fn get_frequency_preset(&self, index: usize) -> Option<mvc::FrequencyPresetModel> {
        self.frequency_presets.borrow().get(index).cloned()
    }

    // fn toggle_flagged(&self, index: usize) -> bool {
    //     if let Some(frequency_preset) = self.missions.borrow_mut().get_mut(index) {
    //         frequency_preset.flagged = !mission.flagged;
    //         return true;
    //     }

    //     false
    // }

    fn remove_frequency_preset(&self, index: usize) -> bool {
        if index < self.frequency_presets.borrow().len() {
            self.frequency_presets.borrow_mut().remove(index);
            return true;
        }

        false
    }

    fn push_frequency_preset(&self, mission: mvc::FrequencyPresetModel) -> bool {
        self.frequency_presets.borrow_mut().push(mission);
        true
    }

    fn update_frequency_preset(&self, index: usize, preset: mvc::FrequencyPresetModel) -> bool {
        // self.frequency_presets.borrow_mut().push(preset);
        if index < self.frequency_presets.borrow().len() {
            self.frequency_presets.borrow_mut()[index] = preset;
            return true
        }
        false
    }   

    // fn frequency_preset_count(&self) -> usize {
    //     self.frequency_presets.borrow().len()
    // }

    // fn get_frequency_preset_by_name(&self, frequency_preset_name: &str) -> Option<mvc::FrequencyPresetModel> {
    //     self.frequency_presets.borrow().get(preset_name).cloned()
    // }

    // fn get_frequency_preset_by_idx(&self, index: usize) -> Option<mvc::FrequencyPresetModel> {
    //     self.frequency_presets.borrow().values().filter(|x| x.preset_id == index).next().map(|x| x.to_owned())
    // }

    // fn set_freq1(&self, frequency_preset_name: &str, freq1: f32) -> bool {
    //     if let Some(frequency_preset) = self.frequency_presets.borrow_mut().get_mut(frequency_preset_name) {
    //         frequency_preset.values.freq1 = freq1;
    //         return true;
    //     }
    //     false
    // }

    // fn remove_frequency_preset_by_name(&self, frequency_preset_name: &str) -> bool {
    //     if let Some(_) = self.frequency_presets.borrow().get(frequency_preset_name) {
    //         self.frequency_presets.borrow_mut().remove(frequency_preset_name);
    //         return true;
    //     }
    //     false
    // }

    // fn remove_frequency_preset_by_idx(&self, index: usize) -> bool {
    //     if let Some(frequency_preset) = self.get_frequency_preset_by_idx(index) {
    //         return self.remove_frequency_preset_by_name(&frequency_preset.preset_name);
    //     }

    //     false
    // }

    // fn push_frequency_preset(&self, frequency_preset: mvc::FrequencyPresetModel) -> bool {
    //     self.frequency_presets.borrow_mut().insert(frequency_preset.preset_name.to_owned(), frequency_preset);
    //     true
    // }
}
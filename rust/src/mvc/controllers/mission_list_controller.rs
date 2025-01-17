
use std::rc::Rc;

use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

use crate::mvc;
// use crate::ui;
// use crate::mvc::FrequencyModel;
// use crate::mvc::PowerModel;
use crate::Callback;

#[derive(Clone)]
pub struct MissionListController {
    mission_model: MissionModel,
    show_create_mission_callback: Rc<Callback<(), ()>>,
    show_edit_mission_callback: Rc<Callback<(), ()>>,
}

impl MissionListController {
    pub fn new(repo: impl mvc::traits::MissionRepository + 'static) -> Self {
        Self {
            mission_model: MissionModel::new(repo),
            show_create_mission_callback: Rc::new(Callback::default()),
            show_edit_mission_callback: Rc::new(Callback::default()),
        }
    }

    pub fn mission_model(&self) -> ModelRc<mvc::MissionModel> {
        ModelRc::new(self.mission_model.clone())
    }

    // pub fn toggle_flagged(&self, index: usize) {
    //     self.mission_model.toggle_flagged(index)
    // }

    pub fn get_mission(&self, index: usize) -> Option<mvc::MissionModel> {
        self.mission_model.get_mission(index)
    }

    pub fn remove_mission(&self, index: usize) {
        self.mission_model.remove_mission(index)
    }


    // pub fn update_mission(&self, index: usize, mission_name: &str, mission_desc: &str, power_preset: &PowerPresetSlintStruct, frequency_preset: &FrequencyPresetSlintStruct) {
    // pub fn update_mission(&self, index: usize, mission: &ui::MissionSlintStruct) {
    //     println!("controllers/frequency_mission_list_controller:create_mission");
    //     // let mission_id = self.mission_model.
    //     self.mission_model.update_mission(index, mvc::MissionModel {
    //         mission_name: mission_name.into(),
    //         mission_desc: mission_desc.into(),
    //         power_model: power_preset.into(),
    //         frequency_model: frequency_preset.into(),
    //         ..Default::default()
    //     })
    // }
    pub fn update_mission(&self, index: usize, mission: mvc::MissionModel) {
        println!("controllers/frequency_mission_list_controller:create_mission");
        // let mission_id = self.mission_model.
        self.mission_model.update_mission(index, mission)
    }

    // pub fn create_mission(&self, mission: MissionSlintStruct) {
    //     self.mission_model.push_mission(mvc::MissionModel {
    //         mission_name: mission_name.into(),
    //         mission_id,
    //         mission_desc: mission_desc.into(),
    //         flagged,
    //         power_model: power_preset.into(),
    //         frequency_model: frequency_preset.into(),
    //         ..Default::default()
    //     })
    // }
    pub fn create_mission(&self, mission: mvc::MissionModel) {
        // self.mission_model.push_mission(mvc::MissionModel {
        //     mission_name: mission_name.into(),
        //     mission_id,
        //     mission_desc: mission_desc.into(),
        //     flagged,
        //     power_model: power_preset.into(),
        //     frequency_model: frequency_preset.into(),
        //     ..Default::default()
        // })
        self.mission_model.push_mission(mission)
    }

    pub fn show_edit_mission(&self) {
        println!("mission_list_controller.rs show_edit_mission_callback.invoke");
        self.show_edit_mission_callback.invoke(&());
    }

    pub fn on_show_edit_mission(&self, mut callback: impl FnMut() + 'static) {
        println!("mission_list_controller.rs on_show_edit_mission");
        self.show_edit_mission_callback.on(move |()| {
            callback();
        });
    }    

    pub fn show_create_mission(&self) {
        self.show_create_mission_callback.invoke(&());
    }

    pub fn on_show_create_mission(&self, mut callback: impl FnMut() + 'static) {
        self.show_create_mission_callback.on(move |()| {
            callback();
        });
    }

}

#[derive(Clone)]
struct MissionModel {
    repo: Rc<dyn mvc::traits::MissionRepository>,
    notify: Rc<ModelNotify>,
}

impl MissionModel {
    fn new(repo: impl mvc::traits::MissionRepository + 'static) -> Self {
        Self { repo: Rc::new(repo), notify: Rc::new(Default::default()) }
    }

    // fn toggle_flagged(&self, index: usize) {
    //     if !self.repo.toggle_flagged(index) {
    //         return;
    //     }
    //     self.notify.row_changed(index)
    // }

    fn get_mission(&self, index: usize) -> Option<mvc::MissionModel> {
        self.repo.get_mission(index)
    }
    fn update_mission(&self, index: usize, mission: mvc::MissionModel) {
        if !self.repo.update_mission(index, mission) {
            return;
        }
        self.notify.row_changed(index);
    }
    fn remove_mission(&self, index: usize) {
        if !self.repo.remove_mission(index) {
            return;
        }

        self.notify.row_removed(index, 1)
    }

    fn push_mission(&self, mission: mvc::MissionModel) {
        if !self.repo.push_mission(mission) {
            return;
        }

        self.notify.row_added(self.row_count() - 1, 1);
    }
}

impl Model for MissionModel {
    type Data = mvc::MissionModel;

    fn row_count(&self) -> usize {
        self.repo.mission_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.repo.get_mission(row)
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        self.notify.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mvc::{self, FrequencyModel, FrequencyPresetModel, PowerModel, PowerPresetModel};
    use std::cell::Cell;

    fn mission1() -> mvc::MissionModel {
        let power1 = PowerModel{power1: 1.1, power2: 1.2, power3: 3.2, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1};
        let power_preset1 = PowerPresetModel{power_preset_name: "power1".into(), power_preset_desc: "desc".into(), values: power1 };
        let frequency1 = FrequencyModel{freq1: 1.1, freq2: 1.1, freq3: vec![1.1]};
        let frequency_preset1 = FrequencyPresetModel{frequency_preset_name: "frequency1".into(), frequency_preset_desc: "desc".into(), values: frequency1 };
        let mission1 = mvc::MissionModel { mission_name: "mission1".into(), mission_id: 1, mission_desc: "desc".into(), flagged: false, power_model: power_preset1, frequency_model: frequency_preset1  };
        mission1
    }
    fn mission2() -> mvc::MissionModel {
        let power2 = PowerModel{power1: 2.2, power2: 2.2, power3: 3.2, power4: 2.2, power5: 2.2, power6: 2.2, power7: 2.2, power8: 2.2, power9: 2, power10: 2};
        let power_preset2 = PowerPresetModel{power_preset_name: "power2".into(), power_preset_desc: "desc".into(), values: power2 };
        let frequency2 = FrequencyModel{freq1: 2.2, freq2: 2.2, freq3: vec![2.2]};
        let frequency_preset2 = FrequencyPresetModel{frequency_preset_name: "frequency2".into(), frequency_preset_desc: "desc".into(), values: frequency2 };
        let mission2 = mvc::MissionModel { mission_name: "mission2".into(), mission_id: 2, mission_desc: "desc".into(), flagged: false, power_model: power_preset2, frequency_model: frequency_preset2  };
        mission2
    }
    fn mission3() -> mvc::MissionModel {
        let power3 = PowerModel{power1: 3.3, power2: 3.3, power3: 3.3, power4: 3.3, power5: 3.3, power6: 3.3, power7: 3.3, power8: 3.3, power9: 3, power10: 3};
        let power_preset3 = PowerPresetModel{power_preset_name: "power3".into(), power_preset_desc: "desc".into(), values: power3 };
        let frequency3 = FrequencyModel{freq1: 3.3, freq2: 3.3, freq3: vec![3.3]};
        let frequency_preset3 = FrequencyPresetModel{frequency_preset_name: "frequency3".into(), frequency_preset_desc: "desc".into(), values: frequency3 };
        let mission3 = mvc::MissionModel { mission_name: "mission3".into(), mission_id: 3, mission_desc: "desc".into(), flagged: false, power_model: power_preset3, frequency_model: frequency_preset3  };
        mission3
    }

    fn test_controller() -> MissionListController {
        MissionListController::new(mvc::MockMissionRepository::new(vec![
            mission1(), 
            mission2(),
        ]))
    }

    #[test]
    fn test_missions() {
        let controller = test_controller();
        let mission_model = controller.mission_model();

        assert_eq!(mission_model.row_count(), 2);
        assert_eq!(
            mission_model.row_data(0),
            Some(mission1())
        );
        assert_eq!(
            mission_model.row_data(1),
            Some(mission2())
        );
    }

    // #[test]
    // fn test_toggle_mission_checked() {
    //     let controller = test_controller();
    //     let mission_model = controller.mission_model();

    //     assert!(mission_model.row_data(0).unwrap().flagged);
    //     controller.toggle_flagged(0);
    //     assert!(!mission_model.row_data(0).unwrap().flagged);
    // }

    #[test]
    fn test_remove_mission() {
        let controller = test_controller();
        let mission_model = controller.mission_model();
        // let power3 = PowerModel{power1: 1.1, power2: 1.2, power3: 3.2, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1};

        assert_eq!(mission_model.row_count(), 2);
        controller.remove_mission(0);
        assert_eq!(mission_model.row_count(), 1);

        assert_eq!(
            mission_model.row_data(0),
            Some(mission2())
        );
    }

    #[test]
    fn test_show_create_mission() {
        let controller = test_controller();

        let callback_invoked = Rc::new(Cell::new(false));

        controller.on_show_create_mission({
            let callback_invoked = callback_invoked.clone();

            move || {
                callback_invoked.set(true);
            }
        });

        controller.show_create_mission();

        assert!(callback_invoked.get());
    }

    #[test]
    fn test_add_mission() {
        let controller = test_controller();
        let mission_model = controller.mission_model();
        // let power3 = PowerModel{power1: 1.1, power2: 1.2, power3: 3.2, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1};

        assert_eq!(mission_model.row_count(), 2);
        //controller.create_mission("mission3", 3, "desc".into(), false, PowerModel{power1: 1.0, power2: 2}, FrequencyModel{freq1: 1.0, freq2: 1.1, freq3: vec![2.0]});
        // controller.create_mission("mission3", 3, "desc".into(), false, "power1", "band1");
        controller.create_mission(mission3());
        assert_eq!(mission_model.row_count(), 3);

        assert_eq!(
            mission_model.row_data(2),
            Some(mission3())
        );
    }
}

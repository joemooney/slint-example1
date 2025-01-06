// use chrono::format::format;
// use chrono::DateTime;
use slint::*;
use std::rc::Rc;

use crate::{
    mvc::{MissionListController, MissionModel},
    ui,
};

// a helper function to make adapter and controller connection a little bit easier
pub fn connect_with_controller(
    view_handle: &ui::MainWindow,
    controller: &MissionListController,
    connect_adapter_controller: impl FnOnce(ui::MissionListAdapter, MissionListController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::MissionListAdapter>(), controller.clone());
}

// one place to implement connection between adapter (view) and controller
pub fn connect(view_handle: &ui::MainWindow, controller: MissionListController) {
    // sets a mapped list of the mission items to the ui
    view_handle
        .global::<ui::MissionListAdapter>()
        .set_missions(Rc::new(MapModel::new(controller.mission_model(), ui::create_mission_adapter::map_mission_to_slint)).into());

    // connect_with_controller(view_handle, &controller, {
    //     move |adapter, controller| {
    //         adapter.on_toggle_mission_checked(move |index| {
    //             controller.toggle_flagged(index as usize);
    //         })
    //     }
    // });

    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_remove_mission(move |index| {
                controller.remove_mission(index as usize);
            })
        }
    });

    // show-edit-mission
    connect_with_controller(view_handle, &controller, {
        let view_handle = view_handle.as_weak();
        move |adapter, controller| {
            adapter.on_show_edit_mission(move |index| {
                println!("getting mission preset for edit");
                if let Some(mission) = controller.get_mission(index as usize) {
                    println!("edit mission {}", mission.mission_name);
                    set_mission(index, mission, &view_handle.unwrap().global::<ui::MissionAdapter>());
                }
                println!("show edit mission");
                controller.show_edit_mission();
            })
        }
    });

    connect_with_controller(view_handle, &controller, {
        let view_handle = view_handle.as_weak();
        move |adapter: ui::MissionListAdapter, controller| {
            adapter.on_show_create_mission(move || {
                clear_mission(&view_handle.unwrap().global::<ui::MissionAdapter>());
                println!("create mission");
                controller.show_create_mission();
            })
        }
    });
}

fn set_mission(index: i32, mission: MissionModel, adapter: &ui::MissionAdapter) {
    println!("setting mission for edit");
    adapter.set_index(index);
    adapter.set_mode("Update".into());
    println!(">Editing mission:{}", mission.mission_name);
    adapter.set_mission(ui::create_mission_adapter::map_mission_to_slint(mission));
    println!("<Editing mission row:{}", index);
    // adapter.set_mission_desc(mission.mission_desc.into());
    // adapter.set_mission_id(mission.mission_id);
    // adapter.set_power_preset(mission.power_preset);
    // adapter.set_frequency_preset(mission.frequency_preset);
}

fn clear_mission(adapter: &ui::MissionAdapter) {
    adapter.set_mode("Create".into());
    adapter.set_mission(ui::MissionSlintStruct::default());
    // adapter.set_mission_name("".into());
    // adapter.set_mission_desc("".into());
    // adapter.set_mission_id(0);
    // adapter.set_power_preset("".into());
    // adapter.set_frequency_preset("".into());
}

// // maps a MissionModel (data) to a MissionSlintStruct (ui)
// fn map_mission_to_slint(mission: MissionModel) -> ui::MissionSlintStruct {
//     ui::MissionSlintStruct {
//         mission_name: mission.mission_name.into(),
//         checked: mission.flagged,
//         mission_id: mission.mission_id,
//         mission_desc: mission.mission_desc.into(),
//         mission_details: std::format!("{}>freq1:{}|freq2:{}|\n{}>power1:{}|power2:{}", 
//                             mission.frequency_model.frequency_preset_name,
//                             mission.frequency_model.values.freq1,
//                             std::format!("{:#?}", mission.frequency_model.values.freq2),
//                             mission.power_model.power_preset_name,
//                             mission.power_model.values.power1,
//                             mission.power_model.values.power2,
//                          ).into(),
//         frequency_model: ui::create_frequency_preset_adapter::map_frequency_preset_to_slint(mission.frequency_model),
//         power_model: ui::create_power_preset_adapter::map_power_preset_to_slint(mission.power_model),
//     }
// }

// // maps a MissionModel (data) to a MissionSlintStruct (ui)
// fn map_mission_from_slint(mission: ui::MissionSlintStruct) -> MissionModel {
//     MissionModel {
//         mission_name: mission.mission_name.into(),
//         mission_id: mission.mission_id,
//         mission_desc: mission.mission_desc.into(),
//         flagged: false,
//         frequency_model: ui::create_frequency_preset_adapter::map_frequency_preset_from_slint(mission.frequency_model),
//         power_model: ui::create_power_preset_adapter::map_power_preset_from_slint(mission.power_model),
//     }
// }

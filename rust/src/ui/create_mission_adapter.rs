use slint::*;

use crate::{
    mvc::{
        {MissionModel, CreateMissionController, MissionListController},
        // {DateModel, TimeModel},
    },
    ui,
};

// use super::create_frequency_preset_adapter::map_frequency_preset_from_slint;

// a helper function to make adapter and controller connection a little bit easier
fn connect_with_controller(
    view_handle: &ui::MainWindow,
    controller: &CreateMissionController,
    connect_adapter_controller: impl FnOnce(ui::MissionAdapter, CreateMissionController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::MissionAdapter>(), controller.clone());
}

// a helper function to make adapter and controller connection a little bit easier
fn connect_with_mission_list_controller(
    view_handle: &ui::MainWindow,
    controller: &MissionListController,
    connect_adapter_controller: impl FnOnce(ui::MissionAdapter, MissionListController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::MissionAdapter>(), controller.clone());
}

// one place to implement connection between adapter (view) and controller
pub fn connect(view_handle: &ui::MainWindow, controller: CreateMissionController) {
    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_back(move || {
                controller.back();
            })
        }
    });

    // connect_with_controller(view_handle, &controller, {
    //     move |adapter, controller| {
    //         adapter.on_current_date(move || map_date_model_to_date(controller.current_date()))
    //     }
    // });

    // connect_with_controller(view_handle, &controller, {
    //     move |adapter, controller| {
    //         adapter.on_current_time(move || map_time_model_to_time(controller.current_time()))
    //     }
    // });

    // connect_with_controller(view_handle, &controller, {
    //     move |adapter, controller| {
    //         adapter.on_date_string(move |date| {
    //             controller.date_string(map_date_to_date_model(date)).into()
    //         })
    //     }
    // });

    // connect_with_controller(view_handle, &controller, {
    //     move |adapter, controller| {
    //         adapter.on_time_string(move |time| {
    //             controller.time_string(map_time_to_time_model(time)).into()
    //         })
    //     indeindex
    // });

    // connect_with_controller(view_handle, &controller, {
    //     move |adapter, controller| {
    //         adapter.on_time_stamp(move |date, time| {
    //             controller
    //                 .time_stamp(map_date_to_date_model(date), map_time_to_time_model(time))
    //                 .into()
    //         })
    //     }
    // });
}

pub fn connect_mission_list_controller(view_handle: &ui::MainWindow, controller: MissionListController) {
    connect_with_mission_list_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_create(move |mission_model| {
                controller.create_mission(map_mission_from_slint(mission_model))
            })
        }
    });
    connect_with_mission_list_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_update(move |index, mission_model| {
                println!("ui/update_mission_adapter->controller/update_mission");
                controller.update_mission(index as usize, map_mission_from_slint(mission_model))
            })
        }
    });
}

// fn map_time_model_to_time(time_model: TimeModel) -> ui::Time {
//     ui::Time {
//         hour: time_model.hour as i32,
//         minute: time_model.minute as i32,
//         second: time_model.second as i32,
//     }
// }

// fn map_time_to_time_model(time: ui::Time) -> TimeModel {
//     TimeModel { hour: time.hour as u32, minute: time.minute as u32, second: time.second as u32 }
// }

// fn map_date_model_to_date(date_model: DateModel) -> ui::Date {
//     ui::Date { year: date_model.year, month: date_model.month as i32, day: date_model.day as i32 }
// }

// fn map_date_to_date_model(date: ui::Date) -> DateModel {
//     DateModel { year: date.year, month: date.month as u32, day: date.day as u32 }
// }

// maps a MissionModel (data) to a MissionSlintStruct (ui)
pub fn map_mission_to_slint(mission: MissionModel) -> ui::MissionSlintStruct {
    let power_model = ui::create_power_preset_adapter::map_power_preset_to_slint(mission.power_model);
    let frequency_model = ui::create_frequency_preset_adapter::map_frequency_preset_to_slint(mission.frequency_model);
    ui::MissionSlintStruct {
        mission_name: mission.mission_name.into(),
        mission_desc: mission.mission_desc.into(),
        mission_id: mission.mission_id,
        mission_details: std::format!("{} power:{}",frequency_model.preset_details,power_model.preset_name).into(),
        power_model,
        frequency_model,
        checked: false,
    }
}

// maps a MissionModel (data) to a MissionSlintStruct (ui)
pub fn map_mission_from_slint(mission: ui::MissionSlintStruct) -> MissionModel {
    let power_model = ui::create_power_preset_adapter::map_power_preset_from_slint(mission.power_model);
    let frequency_model = ui::create_frequency_preset_adapter::map_frequency_preset_from_slint(mission.frequency_model);
    MissionModel {
        mission_name: mission.mission_name.into(),
        mission_desc: mission.mission_desc.into(),
        mission_id: mission.mission_id,
        power_model,
        flagged: false,
        frequency_model,
    }
}
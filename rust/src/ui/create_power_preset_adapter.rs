use slint::*;

use crate::{
    mvc::{CreatePowerPresetController, PowerModel, PowerPresetListController, PowerPresetModel},
    // ui::{self, create_power_preset_adapter},
    ui,
};

// a helper function to make adapter and controller connection a little bit easier
fn connect_with_controller(
    view_handle: &ui::MainWindow,
    controller: &CreatePowerPresetController,
    connect_adapter_controller: impl FnOnce(ui::CreatePowerPresetAdapter, CreatePowerPresetController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::CreatePowerPresetAdapter>(), controller.clone());
}

// a helper function to make adapter and controller connection a little bit easier
fn connect_with_power_preset_list_controller(
    view_handle: &ui::MainWindow,
    controller: &PowerPresetListController,
    connect_adapter_controller: impl FnOnce(ui::CreatePowerPresetAdapter, PowerPresetListController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::CreatePowerPresetAdapter>(), controller.clone());
}

// one place to implement connection between adapter (view) and controller
pub fn connect(view_handle: &ui::MainWindow, controller: CreatePowerPresetController) {
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
    //     }
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

pub fn connect_power_preset_list_controller(view_handle: &ui::MainWindow, controller: PowerPresetListController) {
    connect_with_power_preset_list_controller(view_handle, &controller, {
        move |adapter, controller| {
            // adapter.on_create(move |preset_name, preset_desc, power1, power2, power3, power4, power5, power6, power7, power8, power9, power10| {
            //     println!("ui/create_power_preset_adapter->controller/create_preset");
            //     controller.create_preset(preset_name.as_str(), preset_desc.as_str(), power1 as f32, power2 as f32, power3 as f32, power4 as f32, power5 as f32, power6 as f32, power7 as f32, power8 as f32, power9 as u32, power10 as u32)
            // });
            adapter.on_create(move |preset| {
                println!("ui/create_power_preset_adapter->controller/create_preset");
                controller.create_preset(map_power_preset_from_slint(preset))
            });
        }
    });
    connect_with_power_preset_list_controller(view_handle, &controller, {
        move |adapter, controller| {
            // adapter.on_update(move |index, preset_name, preset_desc, power1, power2, power3, power4, power5, power6, power7, power8, power9, power10| {
            //     println!("ui/update_power_preset_adapter->controller/update_preset");
            //     controller.update_preset(index as usize, preset_name.as_str(), preset_desc.as_str(), power1 as f32, power2 as f32, power3 as f32, power4 as f32, power5 as f32, power6 as f32, power7 as f32, power8 as f32, power9 as u32, power10 as u32)
            // })
            adapter.on_update(move |index, preset| {
                println!("ui/update_power_preset_adapter->controller/update_preset preset_desc:{}", preset.preset_desc);
                controller.update_preset(index as usize, map_power_preset_from_slint(preset))
            })
        }
    });
}

// maps a PowerPresetModel (data) to a PowerPresetSlintStruct (ui)
pub fn map_power_preset_to_slint(preset: PowerPresetModel) -> ui::PowerPresetSlintStruct {
    ui::PowerPresetSlintStruct {
        preset_name: preset.power_preset_name.into(),
        preset_desc: preset.power_preset_desc.into(),
        preset_details: std::format!("power1:{} power2:{}",preset.values.power1,preset.values.power2).into(),
        checked: false,
        power1: preset.values.power1,
        power2: preset.values.power2,
        power3: preset.values.power3,
        power4: preset.values.power4,
        power5: preset.values.power5,
        power6: preset.values.power6,
        power7: preset.values.power7,
        power8: preset.values.power8,
        power9: preset.values.power9 as i32,
        power10: preset.values.power10 as i32,
        // priority: preset.priority.into(),
        // description: DateTime::from_timestamp_millis(preset.due_date)
        //     .unwrap()
        //     // example: Thu, Jun 6, 2024 16:29
        //     .format("%a, %b %d, %Y %H:%M")
        //     .to_string()
        //     .into(),
    }
}

// maps a PowerPresetSlintStruct (data) to a PowerPresetModel (ui)
pub fn map_power_preset_from_slint(preset: ui::PowerPresetSlintStruct) -> PowerPresetModel {
    let values = PowerModel{
        power1: preset.power1,
        power2: preset.power2,
        power3: preset.power3,
        power4: preset.power4,
        power5: preset.power5,
        power6: preset.power6,
        power7: preset.power7,
        power8: preset.power8,
        power9: preset.power9 as u32,
        power10: preset.power10 as u32,
    };
    PowerPresetModel {
        power_preset_name: preset.preset_name.into(),
        power_preset_desc: preset.preset_desc.into(),
        values,
    }
}
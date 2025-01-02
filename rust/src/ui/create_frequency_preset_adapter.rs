use slint::*;

use crate::{
    mvc::{
        CreateFrequencyPresetController, FrequencyModel, FrequencyPresetListController, FrequencyPresetModel
    },
    ui,
};

// a helper function to make adapter and controller connection a little bit easier
fn connect_with_controller(
    view_handle: &ui::MainWindow,
    controller: &CreateFrequencyPresetController,
    connect_adapter_controller: impl FnOnce(ui::CreateFrequencyPresetAdapter, CreateFrequencyPresetController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::CreateFrequencyPresetAdapter>(), controller.clone());
}

// a helper function to make adapter and controller connection a little bit easier
fn connect_with_frequency_preset_list_controller(
    view_handle: &ui::MainWindow,
    controller: &FrequencyPresetListController,
    connect_adapter_controller: impl FnOnce(ui::CreateFrequencyPresetAdapter, FrequencyPresetListController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::CreateFrequencyPresetAdapter>(), controller.clone());
}

// one place to implement connection between adapter (view) and controller
pub fn connect(view_handle: &ui::MainWindow, controller: CreateFrequencyPresetController) {
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

pub fn connect_frequency_preset_list_controller(view_handle: &ui::MainWindow, controller: FrequencyPresetListController) {
    connect_with_frequency_preset_list_controller(view_handle, &controller, {
        move |adapter, controller| {
            // see ui/views/create_frequency_preset_view.slint
            // adapter.on_create(move |preset_name, preset_desc, freq1, freq2, freq3_csv_list| {
            adapter.on_create(move |preset| {
                // see rust/src/mvc/controllers/frequency_preset_list_controller.rs
                // let freq3: Vec<f32> = freq3_csv_list
                // .split(',')
                // .filter_map(|s| s.trim().parse::<f32>().ok())
                // .collect();
                // controller.create_preset(preset_name.as_str(), preset_desc.as_str(), freq1 as f32, freq2 as f32, freq3)
                // controller.create_preset(preset_name.as_str(), preset_desc.as_str(), freq1 as f32, freq2 as f32, freq3)
                controller.create_preset(map_frequency_preset_from_slint(preset))
            })
        }
    });
}
// maps a FrequencyPresetModel (data) to a FrequencyPresetSlintStruct (ui)
pub fn map_frequency_preset_to_slint(preset: FrequencyPresetModel) -> ui::FrequencyPresetSlintStruct {
    ui::FrequencyPresetSlintStruct {
        preset_name: preset.frequency_preset_name.into(),
        preset_desc: preset.frequency_preset_desc.into(),
        preset_details: std::format!("frequency1:{} frequency2:{:#?}",preset.values.freq1,preset.values.freq2).into(),
        freq1: preset.values.freq1,
        freq2: preset.values.freq2,
        freq3: float_list_to_string(&preset.values.freq3).into(),
        checked: false,
        // priority: preset.priority.into(),
        // description: DateTime::from_timestamp_millis(preset.due_date)
        //     .unwrap()
        //     // example: Thu, Jun 6, 2024 16:29
        //     .format("%a, %b %d, %Y %H:%M")
        //     .to_string()
        //     .into(),
    }
}

// maps a FrequencyPresetModel (data) to a FrequencyPresetSlintStruct (ui)
pub fn map_frequency_preset_from_slint(preset: ui::FrequencyPresetSlintStruct) -> FrequencyPresetModel {
    let values = FrequencyModel {
        freq1: preset.freq1,
        freq2: preset.freq2,
        freq3: string_to_float_list(preset.freq3.as_str()),
    };
    FrequencyPresetModel {
        frequency_preset_name: preset.preset_name.into(),
        frequency_preset_desc: preset.preset_desc.into(),
        values,
    }
}

/// Converts a vector of floats into a comma-separated string with rounded integers
pub fn float_list_to_string(floats: &[f32]) -> String {
    floats.iter()
        .map(|&f| f.round() as i32) // Round each float to the nearest integer
        .map(|i| i.to_string())     // Convert each integer to a string
        .collect::<Vec<_>>()
        .join(",")                 // Join the strings with commas
}

/// Converts a comma-separated string of floats to vector of floats, toss invalid
pub fn string_to_float_list(float_list: &str) -> Vec<f32> {
    float_list
    .split(',')
    .filter_map(|s| s.trim().parse::<f32>().ok())
    .collect()
}
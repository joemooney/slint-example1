// use chrono::DateTime;
use slint::*;
use std::rc::Rc;

use crate::{
    mvc::{FrequencyPresetListController, FrequencyPresetModel},
    ui,
};

use super::create_frequency_preset_adapter::map_frequency_preset_to_slint;

// a helper function to make adapter and controller connection a little bit easier
pub fn connect_with_controller(
    view_handle: &ui::MainWindow,
    controller: &FrequencyPresetListController,
    connect_adapter_controller: impl FnOnce(ui::FrequencyPresetListAdapter, FrequencyPresetListController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::FrequencyPresetListAdapter>(), controller.clone());
}

// one place to implement connection between adapter (view) and controller
pub fn connect(view_handle: &ui::MainWindow, controller: FrequencyPresetListController) {
    // sets a mapped list of the frequency items to the ui
    view_handle
        .global::<ui::FrequencyPresetListAdapter>()
        // see ui/views/frequency_preset_list_view.slint
        .set_frequency_presets(Rc::new(MapModel::new(controller.preset_model(), ui::create_frequency_preset_adapter::map_frequency_preset_to_slint)).into());

    // connect_with_controller(view_handle, &controller, {
    //     move |adapter, controller| {
    //         adapter.on_toggle_frequency_checked(move |index| {
    //             controller.toggle_done(index as usize);
    //         })
    //     }
    // });

    // remove-frequency-preset
    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_remove_frequency_preset(move |index| {
                controller.remove_preset(index as usize);
            })
        }
    });

    // show-edit-frequency-preset
    connect_with_controller(view_handle, &controller, {
        let view_handle = view_handle.as_weak();
        move |adapter, controller| {
            adapter.on_show_edit_frequency_preset(move |index| {
                println!("getting frequency preset for edit");
                if let Some(preset) = controller.get_preset(index as usize) {
                    println!("edit preset {}", preset.frequency_preset_name);
                    set_frequency(index, preset, &view_handle.unwrap().global::<ui::FrequencyPresetAdapter>());
                }
                println!("show edit frequency preset");
                controller.show_edit_frequency_preset();
            })
        }
    });

    connect_with_controller(view_handle, &controller, {
        let view_handle = view_handle.as_weak();
        move |adapter: ui::FrequencyPresetListAdapter, controller| {
            adapter.on_show_create_frequency_preset(move || {
                clear_frequency(&view_handle.unwrap().global::<ui::FrequencyPresetAdapter>());
                controller.show_create_frequency_preset();
            })
        }
    });
}


fn set_frequency(index: i32, preset: FrequencyPresetModel, adapter: &ui::FrequencyPresetAdapter) {
    println!("setting frequency preset for edit");
    adapter.set_index(index);
    adapter.set_mode("Update".into());
    println!(">Editing preset:{}", preset.frequency_preset_name);
    adapter.set_preset(map_frequency_preset_to_slint(preset));
    println!(">Editing preset row:{}", index);
    // adapter.set_preset_name(preset.frequency_preset_name.into());
    // adapter.set_preset_desc(preset.frequency_preset_desc.into());
    // adapter.set_freq1(preset.values.freq1);
    // adapter.set_freq2(preset.values.freq2);
    // adapter.set_freq3(preset.values.freq3_csv().into());
}

fn clear_frequency(adapter: &ui::FrequencyPresetAdapter) {
    adapter.set_mode("Create".into());
    adapter.set_preset(map_frequency_preset_to_slint(FrequencyPresetModel::default()));
    // adapter.set_preset_name("".into());
    // adapter.set_preset_desc("".into());
    // adapter.set_freq1(0.0);
    // adapter.set_freq2(0.0);
    // adapter.set_freq3("".into());
}


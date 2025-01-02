// use chrono::DateTime;
use slint::*;
use std::rc::Rc;

use crate::{
    mvc::{PowerPresetListController, PowerPresetModel},
    ui,
};

use super::create_power_preset_adapter::map_power_preset_to_slint;

// a helper function to make adapter and controller connection a little bit easier
pub fn connect_with_controller(
    view_handle: &ui::MainWindow,
    controller: &PowerPresetListController,
    connect_adapter_controller: impl FnOnce(ui::PowerPresetListAdapter, PowerPresetListController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::PowerPresetListAdapter>(), controller.clone());
}

// one place to implement connection between adapter (view) and controller
pub fn connect(view_handle: &ui::MainWindow, controller: PowerPresetListController) {
    // sets a mapped list of the power items to the ui
    view_handle
        .global::<ui::PowerPresetListAdapter>()
        // see ui/views/power_preset_list_view.slint
        .set_xpower_presets(Rc::new(MapModel::new(controller.preset_model(), ui::create_power_preset_adapter::map_power_preset_to_slint)).into());

    // connect_with_controller(view_handle, &controller, {
    //     move |adapter, controller| {
    //         adapter.on_toggle_power_checked(move |index| {
    //             controller.toggle_done(index as usize);
    //         })
    //     }
    // });

    // remove-power-preset
    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_remove_power_preset(move |index| {
                controller.remove_preset(index as usize);
            })
        }
    });

    // show-edit-power-preset
    connect_with_controller(view_handle, &controller, {
        let view_handle = view_handle.as_weak();
        move |adapter, controller| {
            adapter.on_show_edit_power_preset(move |index| {
                // view_handle.unwrap().global::<ui::PowerPresetNavigationAdapter>().set_current_power_preset(index);
                // ui::PowerPresetNavigationAdapter::current_power_preset = index;
                println!("getting power preset for edit");
                if let Some(preset) = controller.get_preset(index as usize) {
                    println!("edit preset {}", preset.power_preset_name);
                    // view_handle.unwrap().global::<ui::CreatePowerPresetAdapter>().set_preset_name(preset.power_preset_name.into());
                    // view_handle.unwrap().global::<ui::CreatePowerPresetAdapter>().set_preset_desc(preset.power_preset_desc.into());
                    // view_handle.unwrap().global::<ui::CreatePowerPresetAdapter>().set_power1(preset.values.power1.into());
                    set_power(index, preset, &view_handle.unwrap().global::<ui::CreatePowerPresetAdapter>());
                }
                controller.show_edit_power_preset();
            })
        }
    });

    // show-create-power-preset
    connect_with_controller(view_handle, &controller, {
        let view_handle = view_handle.as_weak();
        move |adapter: ui::PowerPresetListAdapter, controller| {
            adapter.on_show_create_power_preset(move || {
                clear_power(&view_handle.unwrap().global::<ui::CreatePowerPresetAdapter>());
                println!("create preset");
                controller.show_create_power_preset();
            })
        }
    });
}

/// Called when we are entering Update mode on a Power Preset
fn set_power(index: i32, preset: PowerPresetModel, adapter: &ui::CreatePowerPresetAdapter) {
    adapter.set_index(index);
    adapter.set_mode("Update".into());
    println!(">Editing preset:{}", preset.power_preset_name);
    adapter.set_preset(map_power_preset_to_slint(preset));
    println!("<Editing preset row:{}", index);
    // adapter.set_preset_name(preset.power_preset_name.into());
    // adapter.set_preset_desc(preset.power_preset_desc.into());
    // adapter.set_power1(preset.values.power1);
    // adapter.set_power2(preset.values.power2);
    // adapter.set_power3(preset.values.power3);
    // adapter.set_power4(preset.values.power4);
    // adapter.set_power5(preset.values.power5);
    // adapter.set_power6(preset.values.power6);
    // adapter.set_power7(preset.values.power7);
    // adapter.set_power8(preset.values.power8);
    // adapter.set_power9(preset.values.power9 as i32);
    // adapter.set_power10(preset.values.power10 as i32);
}

fn clear_power(adapter: &ui::CreatePowerPresetAdapter) {
    adapter.set_mode("Create".into());
    adapter.set_preset(map_power_preset_to_slint(PowerPresetModel::default()));
    // adapter.set_preset_name("".into());
    // adapter.set_preset_desc("".into());
    // adapter.set_power1(0.0);
    // adapter.set_power2(0.0);
    // adapter.set_power3(0.0);
    // adapter.set_power4(0.0);
    // adapter.set_power5(0.0);
    // adapter.set_power6(0.0);
    // adapter.set_power7(0.0);
    // adapter.set_power8(0.0);
    // adapter.set_power9(0);
    // adapter.set_power10(0);
}

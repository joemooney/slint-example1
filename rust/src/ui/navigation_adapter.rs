// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use slint::*;

use crate::{
    mvc::{CreateTaskController, TaskListController},
    mvc::{CreateMissionController, MissionListController},
    mvc::{CreatePowerPresetController, PowerPresetListController},
    mvc::{CreateFrequencyPresetController, FrequencyPresetListController},
    ui,
};

// one place to implement connection between adapter (view) and controller
pub fn connect_create_task_controller(
    view_handle: &ui::MainWindow,
    controller: CreateTaskController,
) {
    controller.on_back({
        let view_handle = view_handle.as_weak();

        move || {
            view_handle.unwrap().global::<ui::NavigationAdapter>().invoke_previous_page();
        }
    });
}

// one place to implement connection between adapter (view) and controller
pub fn connect_task_list_controller(view_handle: &ui::MainWindow, controller: TaskListController) {
    controller.on_show_create_task({
        let view_handle = view_handle.as_weak();
        move || {
            view_handle.unwrap().global::<ui::NavigationAdapter>().invoke_next_page();
        }
    });
}
// one place to implement connection between adapter (view) and controller
pub fn connect_create_mission_controller(
    view_handle: &ui::MainWindow,
    controller: CreateMissionController,
) {
    controller.on_back({
        let view_handle = view_handle.as_weak();

        move || {
            view_handle.unwrap().global::<ui::MissionNavigationAdapter>().invoke_previous_page();
        }
    });
}

// one place to implement connection between adapter (view) and controller
pub fn connect_mission_list_controller(view_handle: &ui::MainWindow, controller: MissionListController) {
    controller.on_show_create_mission({
        let view_handle = view_handle.as_weak();
        move || {
            view_handle.unwrap().global::<ui::MissionNavigationAdapter>().invoke_next_page();
        }
    });
    controller.on_show_edit_mission({
        println!("navigation_adapter.rs on_show_edit_mission");
        let view_handle = view_handle.as_weak();
        move || {
            view_handle.unwrap().global::<ui::MissionNavigationAdapter>().invoke_next_page();
        }
    });
}
// one place to implement connection between adapter (view) and controller
pub fn connect_create_power_preset_controller(
    view_handle: &ui::MainWindow,
    controller: CreatePowerPresetController,
) {
    controller.on_back({
        let view_handle = view_handle.as_weak();
        move || {
            view_handle.unwrap().global::<ui::PowerPresetNavigationAdapter>().invoke_previous_page();
        }
    });
}

// one place to implement connection between adapter (view) and controller
pub fn connect_power_preset_list_controller(view_handle: &ui::MainWindow, controller: PowerPresetListController) {
    controller.on_show_create_power_preset({
        let view_handle = view_handle.as_weak();
        move || {
            view_handle.unwrap().global::<ui::PowerPresetNavigationAdapter>().invoke_next_page();
        }
    });
    controller.on_show_edit_power_preset( {
        println!("navigation_adapter.rs on_show_edit_power_preset");
        let view_handle = view_handle.as_weak();
        move || {
            view_handle.unwrap().global::<ui::PowerPresetNavigationAdapter>().invoke_next_page();
        }
    });
}

// one place to implement connection between adapter (view) and controller
pub fn connect_create_frequency_preset_controller(
    view_handle: &ui::MainWindow,
    controller: CreateFrequencyPresetController,
) {
    controller.on_back({
        let view_handle = view_handle.as_weak();
        move || {
            view_handle.unwrap().global::<ui::FrequencyPresetNavigationAdapter>().invoke_previous_page();
        }
    });
}

// one place to implement connection between adapter (view) and controller
pub fn connect_frequency_preset_list_controller(view_handle: &ui::MainWindow, controller: FrequencyPresetListController) {
    controller.on_show_create_frequency_preset({
        let view_handle = view_handle.as_weak();
        move || {
            view_handle.unwrap().global::<ui::FrequencyPresetNavigationAdapter>().invoke_next_page();
        }
    });
    controller.on_show_edit_frequency_preset({
        println!("navigation_adapter.rs on_show_edit_frequency_preset");
        let view_handle = view_handle.as_weak();

        move || {
            view_handle.unwrap().global::<ui::FrequencyPresetNavigationAdapter>().invoke_next_page();
        }
    });
}
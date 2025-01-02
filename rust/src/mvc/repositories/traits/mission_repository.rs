use crate::mvc;

pub trait MissionRepository {
    fn mission_count(&self) -> usize;
    fn get_mission(&self, index: usize) -> Option<mvc::MissionModel>;
    // fn toggle_flagged(&self, index: usize) -> bool;
    fn remove_mission(&self, index: usize) -> bool;
    fn push_mission(&self, mission: mvc::MissionModel) -> bool;
    fn update_mission(&self, index: usize, mission: mvc::MissionModel) -> bool;
}
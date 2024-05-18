// +- 1393 linijki kodu 

pub mod user;
pub mod menu;
pub mod generator;
pub mod algorithms;
pub mod data_handler;

use crate::user::UserService;

fn main() {
    // Starting user service
    let mut user_service = UserService::new();
    user_service.start_service();
}

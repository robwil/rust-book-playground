use crate::controllers::services::account_service::AccountService;
// use super::services::account_service::AccountService;

pub struct AccountController {}

impl AccountController {
    pub fn hello() {
        AccountService::hello();
        println!("hello from account controller");
    }
}
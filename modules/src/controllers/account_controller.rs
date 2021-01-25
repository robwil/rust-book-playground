use crate::controllers::services::account_service::AccountService;
// "Relative" import would also work here
// use super::services::account_service::AccountService;

pub struct AccountController {}

impl AccountController {
    pub fn hello() {
        AccountService::hello();
        println!("hello from account controller");
    }
}
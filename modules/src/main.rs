
use crate::controllers::account_controller::AccountController;

// The below fails because services mod is not public.
// use crate::controllers::services::account_service::AccountService;

mod controllers;


fn main() {
    AccountController::hello();
    // The below is impossible because AccountService (and all services) are private
    // AccountService::hello();
}

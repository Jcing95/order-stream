pub mod user;
pub mod category;
pub mod websocket;

pub fn provide_all() {
    websocket::provide();
    user::provide();
    category::provide();
}
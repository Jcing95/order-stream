pub mod user;
pub mod category;
pub mod product;
pub mod websocket;

pub fn provide_all() {
    user::provide();
    category::provide();
    product::provide();
    websocket::provide();
}
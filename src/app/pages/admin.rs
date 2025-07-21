use leptos::prelude::*;

use crate::app::components::admin::{create_category::CreateCategory, categories::Categories};


#[component]
pub fn Admin() -> impl IntoView {

    view!{
        <CreateCategory/>
        <Categories/>
    }
}
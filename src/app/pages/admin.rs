use leptos::prelude::*;

use crate::app::components::admin::{create_category::CreateCategory, categories::Categories};


#[component]
pub fn Admin() -> impl IntoView {

    view!{
        <div class="flex flex-row w-full">
            <CreateCategory attr:class="grow"/>
            <Categories attr:class="grow-3"/>
        </div>
    }
}
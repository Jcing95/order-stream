use leptos::prelude::*;
use crate::common::types::{Category, CreateCategoryRequest};
use crate::frontend::components::{category_form::CategoryForm, category_list::CategoryList};

#[component]
pub fn CategorySection<F1, F2>(
    categories: ReadSignal<Vec<Category>>,
    on_submit: F1,
    on_delete: F2,
) -> impl IntoView
where
    F1: Fn(CreateCategoryRequest) + 'static + Clone,
    F2: Fn(String) + 'static + Clone + Send,
{
    view! {
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div>
                <CategoryForm on_submit=on_submit />
            </div>
            <div>
                <CategoryList categories=categories on_delete=on_delete />
            </div>
        </div>
    }
}
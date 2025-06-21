use leptos::prelude::*;
use crate::common::types::Category;
use crate::frontend::design_system::{
    Card, CardVariant, Button, Text, Alert,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight},
};

#[component]
pub fn CategoryList<F>(
    categories: ReadSignal<Vec<Category>>,
    on_delete: F,
) -> impl IntoView 
where
    F: Fn(String) + 'static + Clone + Send + Sync,
{
    let on_delete_clone = on_delete.clone();

    view! {
        <div class="space-y-4">
            <Text 
                variant=TextVariant::Heading 
                size=Size::Lg 
                weight=FontWeight::Semibold
            >
                "Categories"
            </Text>
            
            {move || {
                let cats = categories.get();
                if cats.is_empty() {
                    view! {
                        <Alert intent=Intent::Info size=Size::Md>
                            "No categories yet. Add one above to get started."
                        </Alert>
                    }.into_any()
                } else {
                    cats.into_iter().map(|category| {
                        let category_id = category.id.clone();
                        let on_delete_inner = on_delete_clone.clone();
                        view! {
                            <Card variant=CardVariant::Default>
                                <div class="flex items-center justify-between">
                                    <div class="space-y-1">
                                        <Text 
                                            variant=TextVariant::Body 
                                            size=Size::Md 
                                            weight=FontWeight::Medium
                                        >
                                            {move || category.name.clone()}
                                        </Text>
                                        <Text 
                                            variant=TextVariant::Caption 
                                            size=Size::Sm 
                                            weight=FontWeight::Normal
                                        >
                                            "ID: " {move || category.id.clone()}
                                        </Text>
                                    </div>
                                    <Button
                                        size=Size::Sm
                                        intent=Intent::Danger
                                        on_click=Callback::new(move |_| on_delete_inner(category_id.clone()))
                                    >
                                        "Delete"
                                    </Button>
                                </div>
                            </Card>
                        }
                    }).collect_view().into_any()
                }
            }}
        </div>
    }
}
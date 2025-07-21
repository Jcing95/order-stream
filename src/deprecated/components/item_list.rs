use leptos::prelude::*;
use crate::common::types::Item;
use crate::frontend::design_system::{
    Card, CardVariant, Text, Alert,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight},
};

#[component]
pub fn ItemList(items: ReadSignal<Vec<Item>>) -> impl IntoView {
    view! {
        <Card variant=CardVariant::Default>
            <div class="space-y-4">
                <Text 
                    variant=TextVariant::Heading 
                    size=Size::Lg 
                    weight=FontWeight::Semibold
                >
                    "Items"
                </Text>
                
                <div class="space-y-2">
                    {move || {
                        let items_list = items.get();
                        if items_list.is_empty() {
                            view! {
                                <Alert intent=Intent::Info size=Size::Md>
                                    "No items yet. Add some items to get started!"
                                </Alert>
                            }.into_any()
                        } else {
                            items_list.into_iter().map(|item| {
                                view! {
                                    <Card variant=CardVariant::Default>
                                        <div class="flex justify-between items-center">
                                            <div class="space-y-1">
                                                <Text 
                                                    variant=TextVariant::Body 
                                                    size=Size::Md 
                                                    weight=FontWeight::Medium
                                                >
                                                    {move || item.name.clone()}
                                                </Text>
                                                <Text 
                                                    variant=TextVariant::Caption 
                                                    size=Size::Sm 
                                                    weight=FontWeight::Normal
                                                >
                                                    {move || item.category_id.clone()}
                                                </Text>
                                            </div>
                                            <div class="text-right space-y-1">
                                                <Text 
                                                    variant=TextVariant::Body 
                                                    size=Size::Md 
                                                    weight=FontWeight::Semibold
                                                >
                                                    "$"{move || format!("{:.2}", item.price)}
                                                </Text>
                                                <Text 
                                                    variant=TextVariant::Caption 
                                                    size=Size::Xs 
                                                    weight=FontWeight::Medium
                                                    intent=if item.active { Intent::Success } else { Intent::Danger }
                                                >
                                                    {move || if item.active { "Active" } else { "Inactive" }}
                                                </Text>
                                            </div>
                                        </div>
                                    </Card>
                                }
                            }).collect_view().into_any()
                        }
                    }}
                </div>
            </div>
        </Card>
    }
}
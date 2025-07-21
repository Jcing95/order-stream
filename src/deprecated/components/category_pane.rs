use leptos::prelude::*;
use crate::common::types::{Category, Item, Order};
use crate::frontend::design_system::{
    atoms::{FontWeight, TextVariant, CardVariant},
    theme::{Intent, Size, ComponentState},
    Text, Card, Button,
};

#[component]
pub fn CategoryPane(
    category: Category,
    items: Vec<Item>,
    on_item_click: Callback<(String, u32)>,
    _current_order: ReadSignal<Option<Order>>,
    is_creating_order: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <Card variant=CardVariant::Default>
            <div class="p-4">
                <Text 
                    variant=TextVariant::Heading 
                    size=Size::Md 
                    weight=FontWeight::Semibold
                    class="mb-4"
                >
                    {category.name}
                </Text>
                
                <div class="grid grid-cols-2 gap-3">
                    {items.into_iter().map(|item| {
                        let item_id = item.id.clone();
                        let item_name = item.name.clone();
                        let item_price = item.price;
                        
                        view! {
                            <Button
                                size=Size::Lg
                                intent=Intent::Secondary
                                state=if is_creating_order.get() { ComponentState::Loading } else { ComponentState::Enabled }
                                on_click=Callback::new(move |_| {
                                    if !is_creating_order.get_untracked() {
                                        on_item_click.run((item_id.clone(), 1));
                                    }
                                })
                            >
                                <div class="flex flex-col items-center justify-center h-16">
                                    <Text 
                                        variant=TextVariant::Body 
                                        size=Size::Sm 
                                        weight=FontWeight::Semibold
                                        class="mb-1"
                                    >
                                        {item_name.clone()}
                                    </Text>
                                    <Text 
                                        variant=TextVariant::Caption 
                                        size=Size::Xs
                                        intent=Intent::Success
                                    >
                                        "$" {format!("{:.2}", item_price)}
                                    </Text>
                                </div>
                            </Button>
                        }
                    }).collect_view()}
                </div>
            </div>
        </Card>
    }
}
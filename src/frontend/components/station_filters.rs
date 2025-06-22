use leptos::prelude::*;
use crate::common::types::{Category, OrderStatus};
use crate::frontend::design_system::{
    Card, CardVariant, Text, Select, SelectOption, Button,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight}
};

#[component]
pub fn StationFilters(
    categories: ReadSignal<Vec<Category>>,
    available_statuses: Vec<OrderStatus>,
    
    // Selected filters
    selected_category_ids: RwSignal<Vec<String>>,
    selected_status: RwSignal<OrderStatus>,
    
    // Optional title
    #[prop(optional)]
    title: Option<&'static str>,
) -> impl IntoView {
    // Convert categories to select options
    let category_options = Signal::derive(move || {
        let mut options = vec![SelectOption::new("", "All Categories")];
        for category in categories.get() {
            options.push(SelectOption::new(category.id.clone(), category.name.clone()));
        }
        options
    });
    
    // Clone available_statuses for use in closures
    let available_statuses_clone = available_statuses.clone();
    
    // Convert statuses to select options using indices
    let status_options = available_statuses.iter().enumerate()
        .map(|(i, status)| {
            let label = match status {
                OrderStatus::Draft => "Draft",
                OrderStatus::Ordered => "Ordered",
                OrderStatus::Ready => "Ready", 
                OrderStatus::Completed => "Completed",
                OrderStatus::Cancelled => "Cancelled",
            };
            SelectOption::new(i.to_string(), label)
        })
        .collect::<Vec<_>>();
    
    // Helper functions for status/index conversion
    let status_to_index = |status: OrderStatus| -> usize {
        available_statuses.iter().position(|&s| s == status).unwrap_or(0)
    };
    
    let index_to_status = {
        let statuses = available_statuses_clone.clone();
        move |index: usize| -> OrderStatus {
            statuses.get(index).copied().unwrap_or(OrderStatus::Draft)
        }
    };
    
    // For now, using single category selection - we may need multi-select later
    let selected_category = RwSignal::new(String::new());
    let selected_status_index = RwSignal::new(status_to_index(selected_status.get()).to_string());
    
    // Update the vec when single category changes
    Effect::new({
        let selected_category_ids = selected_category_ids;
        move |_| {
            let cat_id = selected_category.get();
            if cat_id.is_empty() {
                selected_category_ids.set(Vec::new());
            } else {
                selected_category_ids.set(vec![cat_id]);
            }
        }
    });
    
    // Update status when index changes
    Effect::new({
        let selected_status = selected_status;
        let index_to_status = index_to_status.clone();
        move |_| {
            let index_str = selected_status_index.get();
            if let Ok(index) = index_str.parse::<usize>() {
                selected_status.set(index_to_status(index));
            }
        }
    });

    view! {
        <Card variant=CardVariant::Default>
            <div class="space-y-4">
                {title.map(|t| view! {
                    <Text 
                        variant=TextVariant::Heading 
                        size=Size::Md 
                        weight=FontWeight::Semibold
                    >
                        {t}
                    </Text>
                })}
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    // Category filter
                    <div class="space-y-2">
                        <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                            "Category"
                        </Text>
                        <Select
                            size=Size::Md
                            intent=Intent::Primary
                            options=category_options.get()
                            placeholder="Select category"
                            value=selected_category
                        />
                    </div>
                    
                    // Status filter  
                    <div class="space-y-2">
                        <Text variant=TextVariant::Body size=Size::Sm weight=FontWeight::Medium>
                            "Status"
                        </Text>
                        <Select
                            size=Size::Md
                            intent=Intent::Primary
                            options=status_options
                            value=selected_status_index
                        />
                    </div>
                </div>
                
                // Quick filter buttons for common selections
                <div class="flex flex-wrap gap-2">
                    {
                        let ordered_index = available_statuses_clone.iter().position(|&s| s == OrderStatus::Ordered);
                        let ready_index = available_statuses_clone.iter().position(|&s| s == OrderStatus::Ready);
                        
                        view! {
                            {ordered_index.map(|idx| view! {
                                <Button
                                    size=Size::Sm
                                    intent=Intent::Secondary
                                    on_click=Callback::new(move |_| {
                                        selected_category.set(String::new());
                                        selected_status_index.set(idx.to_string());
                                    })
                                >
                                    "All Ordered"
                                </Button>
                            })}
                            
                            {ready_index.map(|idx| view! {
                                <Button
                                    size=Size::Sm
                                    intent=Intent::Secondary
                                    on_click=Callback::new(move |_| {
                                        selected_category.set(String::new());
                                        selected_status_index.set(idx.to_string());
                                    })
                                >
                                    "All Ready"
                                </Button>
                            })}
                        }
                    }
                </div>
            </div>
        </Card>
    }
}
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
    let available_statuses_for_index = available_statuses.clone();
    let status_to_index = move |status: OrderStatus| -> usize {
        available_statuses_for_index.iter().position(|&s| s == status).unwrap_or(0)
    };
    
    let index_to_status = {
        let statuses = available_statuses_clone.clone();
        move |index: usize| -> OrderStatus {
            statuses.get(index).copied().unwrap_or(OrderStatus::Draft)
        }
    };
    
    // For now, using single category selection - we may need multi-select later
    let selected_category = RwSignal::new(String::new());
    
    // Derive status index from selected status signal instead of separate state
    let selected_status_index = Signal::derive(move || {
        status_to_index(selected_status.get()).to_string()
    });
    
    // Create a settable version for the select component
    let selected_status_index_rw = RwSignal::new(selected_status_index.get_untracked());
    
    // Update the category vec when single category changes
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
    
    // Update status when index changes via the RwSignal
    Effect::new({
        let selected_status = selected_status;
        let index_to_status = index_to_status.clone();
        move |_| {
            let index_str = selected_status_index_rw.get();
            if let Ok(index) = index_str.parse::<usize>() {
                selected_status.set(index_to_status(index));
            }
        }
    });
    
    // Sync derived signal changes back to RwSignal for UI
    Effect::new({
        move |_| {
            selected_status_index_rw.set(selected_status_index.get());
        }
    });
    
    // Optimized quick filter handlers
    let handle_all_ordered = if let Some(idx) = available_statuses_clone.iter().position(|&s| s == OrderStatus::Ordered) {
        Some(Callback::new(move |_: leptos::ev::MouseEvent| {
            selected_category.set(String::new());
            selected_status_index_rw.set(idx.to_string());
        }))
    } else {
        None
    };
    
    let handle_all_ready = if let Some(idx) = available_statuses_clone.iter().position(|&s| s == OrderStatus::Ready) {
        Some(Callback::new(move |_: leptos::ev::MouseEvent| {
            selected_category.set(String::new());
            selected_status_index_rw.set(idx.to_string());
        }))
    } else {
        None
    };

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
                            value=selected_status_index_rw
                        />
                    </div>
                </div>
                
                // Quick filter buttons for common selections
                <div class="flex flex-wrap gap-2">
                    {handle_all_ordered.map(|handler| view! {
                        <Button
                            size=Size::Sm
                            intent=Intent::Secondary
                            on_click=handler
                        >
                            "All Ordered"
                        </Button>
                    })}
                    
                    {handle_all_ready.map(|handler| view! {
                        <Button
                            size=Size::Sm
                            intent=Intent::Secondary
                            on_click=handler
                        >
                            "All Ready"
                        </Button>
                    })}
                </div>
            </div>
        </Card>
    }
}
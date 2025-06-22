use leptos::prelude::*;
use crate::common::types::{Station, Category, OrderStatus};
use crate::frontend::design_system::{
    Card, CardVariant, Button, Text, Alert,
    theme::{Size, Intent},
    atoms::{TextVariant, FontWeight},
};

#[component]
pub fn StationList<F>(
    stations: ReadSignal<Vec<Station>>,
    categories: ReadSignal<Vec<Category>>,
    on_delete: F,
) -> impl IntoView 
where
    F: Fn(String) + 'static + Clone + Send + Sync,
{
    let on_delete_clone = on_delete.clone();

    let status_text = |status: &OrderStatus| -> &'static str {
        match status {
            OrderStatus::Draft => "Draft",
            OrderStatus::Ordered => "Ordered",
            OrderStatus::Ready => "Ready",
            OrderStatus::Completed => "Completed",
            OrderStatus::Cancelled => "Cancelled",
        }
    };


    let get_category_names = move |category_ids: &Vec<String>| -> Vec<String> {
        let cats = categories.get();
        category_ids.iter()
            .filter_map(|id| {
                cats.iter().find(|cat| &cat.id == id).map(|cat| cat.name.clone())
            })
            .collect()
    };

    view! {
        <div class="space-y-4">
            <Text 
                variant=TextVariant::Heading 
                size=Size::Lg 
                weight=FontWeight::Semibold
            >
                "Stations"
            </Text>
            
            {move || {
                let stations_list = stations.get();
                if stations_list.is_empty() {
                    view! {
                        <Alert intent=Intent::Info size=Size::Md>
                            "No stations configured yet. Add one above to get started."
                        </Alert>
                    }.into_any()
                } else {
                    stations_list.into_iter().map(|station| {
                        let station_id = station.id.clone();
                        let station_name = station.name.clone();
                        let station_name_for_url = station.name.clone();
                        let category_names = get_category_names(&station.category_ids);
                        let input_statuses = station.input_statuses.clone();
                        let output_status = station.output_status;
                        let on_delete_inner = on_delete_clone.clone();
                        
                        view! {
                            <Card variant=CardVariant::Default>
                                <div class="space-y-4">
                                    // Header with name and delete button
                                    <div class="flex items-start justify-between">
                                        <div class="space-y-1">
                                            <Text 
                                                variant=TextVariant::Body 
                                                size=Size::Lg 
                                                weight=FontWeight::Bold
                                            >
                                                {station_name.clone()}
                                            </Text>
                                        </div>
                                        <Button
                                            size=Size::Sm
                                            intent=Intent::Danger
                                            on_click=Callback::new(move |_| on_delete_inner(station_id.clone()))
                                        >
                                            "Delete"
                                        </Button>
                                    </div>

                                    // Station details
                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
                                        // Categories
                                        <div class="space-y-1">
                                            <Text 
                                                variant=TextVariant::Body 
                                                size=Size::Sm 
                                                weight=FontWeight::Medium
                                            >
                                                "Categories:"
                                            </Text>
                                            <div class="flex flex-wrap gap-1">
                                                {if category_names.is_empty() {
                                                    view! {
                                                        <Text 
                                                            variant=TextVariant::Caption 
                                                            size=Size::Xs 
                                                            class="text-gray-500"
                                                        >
                                                            "No categories"
                                                        </Text>
                                                    }.into_any()
                                                } else {
                                                    category_names.into_iter().map(|name| {
                                                        view! {
                                                            <Text 
                                                                variant=TextVariant::Caption 
                                                                size=Size::Xs 
                                                                weight=FontWeight::Medium
                                                                class="px-2 py-1 rounded bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300"
                                                            >
                                                                {name}
                                                            </Text>
                                                        }
                                                    }).collect_view().into_any()
                                                }}
                                            </div>
                                        </div>

                                        // Input Statuses  
                                        <div class="space-y-1">
                                            <Text 
                                                variant=TextVariant::Body 
                                                size=Size::Sm 
                                                weight=FontWeight::Medium
                                            >
                                                "Shows Orders:"
                                            </Text>
                                            <div class="flex flex-wrap gap-1">
                                                {input_statuses.into_iter().map(|status| {
                                                    view! {
                                                        <Text 
                                                            variant=TextVariant::Caption 
                                                            size=Size::Xs 
                                                            weight=FontWeight::Medium
                                                            class="px-2 py-1 rounded bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200"
                                                        >
                                                            {status_text(&status)}
                                                        </Text>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </div>

                                        // Output Status
                                        <div class="space-y-1">
                                            <Text 
                                                variant=TextVariant::Body 
                                                size=Size::Sm 
                                                weight=FontWeight::Medium
                                            >
                                                "Updates To:"
                                            </Text>
                                            <Text 
                                                variant=TextVariant::Caption 
                                                size=Size::Xs 
                                                weight=FontWeight::Medium
                                                class="px-2 py-1 rounded bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 inline-block"
                                            >
                                                {status_text(&output_status)}
                                            </Text>
                                        </div>

                                        // Station URL
                                        <div class="space-y-1">
                                            <Text 
                                                variant=TextVariant::Body 
                                                size=Size::Sm 
                                                weight=FontWeight::Medium
                                            >
                                                "Station URL:"
                                            </Text>
                                            <Text 
                                                variant=TextVariant::Caption 
                                                size=Size::Xs 
                                                weight=FontWeight::Normal
                                                class="font-mono text-gray-600 dark:text-gray-400"
                                            >
                                                "/stations/" {station_name_for_url.to_lowercase().replace(" ", "-")}
                                            </Text>
                                        </div>
                                    </div>
                                </div>
                            </Card>
                        }
                    }).collect_view().into_any()
                }
            }}
        </div>
    }
}
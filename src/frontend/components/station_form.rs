use leptos::prelude::*;
use leptos::web_sys;
use crate::common::types::{Category, CreateStationRequest, OrderStatus};
use crate::frontend::design_system::{
    Card, CardVariant, Input, Button, Text, Alert, Select, SelectOption,
    theme::{Size, Intent, ComponentState},
    atoms::{InputType, TextVariant, FontWeight},
};

#[component]
pub fn StationForm<F>(
    categories: ReadSignal<Vec<Category>>,
    on_submit: F
) -> impl IntoView 
where
    F: Fn(CreateStationRequest) + 'static + Clone + Send,
{
    let name = RwSignal::new(String::new());
    let selected_categories = RwSignal::new(Vec::<String>::new());
    let selected_input_statuses = RwSignal::new(vec![OrderStatus::Ordered]);
    let output_status = RwSignal::new(OrderStatus::Ready);
    let error = RwSignal::new(Option::<String>::None);

    // Available statuses for selection
    let all_statuses = vec![
        OrderStatus::Draft,
        OrderStatus::Ordered, 
        OrderStatus::Ready,
        OrderStatus::Completed,
        OrderStatus::Cancelled,
    ];

    let status_text = |status: &OrderStatus| -> &'static str {
        match status {
            OrderStatus::Draft => "Draft",
            OrderStatus::Ordered => "Ordered",
            OrderStatus::Ready => "Ready",
            OrderStatus::Completed => "Completed", 
            OrderStatus::Cancelled => "Cancelled",
        }
    };

    let on_submit_clone = on_submit.clone();
    let submit_form = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        // Clear previous error
        error.set(None);
        
        let request = CreateStationRequest {
            name: name.get().trim().to_string(),
            category_ids: selected_categories.get(),
            input_statuses: selected_input_statuses.get(),
            output_status: output_status.get(),
        };

        // Validate
        if let Err(err) = request.validate() {
            error.set(Some(err));
            return;
        }

        // Submit
        on_submit_clone(request);
        
        // Clear form
        name.set(String::new());
        selected_categories.set(Vec::new());
        selected_input_statuses.set(vec![OrderStatus::Ordered]);
        output_status.set(OrderStatus::Ready);
    };

    view! {
        <Card variant=CardVariant::Default>
            <form on:submit=submit_form class="space-y-6">
                <Text 
                    variant=TextVariant::Heading 
                    size=Size::Lg 
                    weight=FontWeight::Semibold
                >
                    "Add New Station"
                </Text>
                
                {move || error.get().map(|err| view! {
                    <Alert intent=Intent::Danger size=Size::Sm>
                        {err}
                    </Alert>
                })}
                
                // Station Name
                <div class="space-y-2">
                    <Text 
                        variant=TextVariant::Label 
                        size=Size::Sm 
                        weight=FontWeight::Medium
                        as_element="label"
                    >
                        "Station Name"
                    </Text>
                    <Input
                        input_type=InputType::Text
                        size=Size::Md
                        intent=Intent::Primary
                        value=name
                        placeholder="e.g., Bar Station, Kitchen, Pickup Counter"
                        required=true
                        on_input=Callback::new(move |ev| name.set(event_target_value(&ev)))
                    />
                </div>

                // Categories
                <div class="space-y-2">
                    <Text 
                        variant=TextVariant::Label 
                        size=Size::Sm 
                        weight=FontWeight::Medium
                        as_element="label"
                    >
                        "Categories (select which categories this station handles)"
                    </Text>
                    <div class="space-y-2 max-h-32 overflow-y-auto border border-gray-200 dark:border-gray-700 rounded-md p-3">
                        {move || {
                            categories.get().into_iter().map(|category| {
                                let category_id = category.id.clone();
                                let category_id_for_checked = category_id.clone();
                                let category_id_for_change = category_id.clone();
                                let category_name = category.name.clone();
                                
                                view! {
                                    <label class="flex items-center gap-2 cursor-pointer">
                                        <input 
                                            type="checkbox"
                                            checked=move || selected_categories.get().contains(&category_id_for_checked)
                                            on:change=move |ev| {
                                                let checked = event_target_checked(&ev);
                                                let category_id = category_id_for_change.clone();
                                                selected_categories.update(|cats| {
                                                    if checked {
                                                        if !cats.contains(&category_id) {
                                                            cats.push(category_id);
                                                        }
                                                    } else {
                                                        cats.retain(|id| id != &category_id);
                                                    }
                                                });
                                            }
                                            class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                        />
                                        <Text variant=TextVariant::Body size=Size::Sm>
                                            {category_name}
                                        </Text>
                                    </label>
                                }
                            }).collect_view()
                        }}
                    </div>
                </div>
                
                // Input Statuses
                <div class="space-y-2">
                    <Text 
                        variant=TextVariant::Label 
                        size=Size::Sm 
                        weight=FontWeight::Medium
                        as_element="label"
                    >
                        "Show Orders With These Statuses"
                    </Text>
                    <div class="grid grid-cols-2 gap-2">
                        {all_statuses.iter().map(|status| {
                            let status_for_checked = *status;
                            let status_for_change = *status;
                            let status_text_value = status_text(status);
                            view! {
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input 
                                        type="checkbox"
                                        checked=move || selected_input_statuses.get().contains(&status_for_checked)
                                        on:change=move |ev| {
                                            let checked = event_target_checked(&ev);
                                            selected_input_statuses.update(|statuses| {
                                                if checked {
                                                    if !statuses.contains(&status_for_change) {
                                                        statuses.push(status_for_change);
                                                    }
                                                } else {
                                                    statuses.retain(|s| s != &status_for_change);
                                                }
                                            });
                                        }
                                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                    />
                                    <Text variant=TextVariant::Body size=Size::Sm>
                                        {status_text_value}
                                    </Text>
                                </label>
                            }
                        }).collect_view()}
                    </div>
                </div>

                // Output Status
                <div class="space-y-2">
                    <Text 
                        variant=TextVariant::Label 
                        size=Size::Sm 
                        weight=FontWeight::Medium
                        as_element="label"
                    >
                        "Update Items To This Status"
                    </Text>
                    <Select
                        size=Size::Md
                        intent=Intent::Primary
                        options=all_statuses.iter().map(|status| {
                            SelectOption::new(format!("{:?}", status), status_text(status))
                        }).collect()
                        value=RwSignal::new(format!("{:?}", output_status.get()))
                        on_change=Callback::new(move |ev| {
                            let value = event_target_value(&ev);
                            let status = match value.as_str() {
                                "Draft" => OrderStatus::Draft,
                                "Ordered" => OrderStatus::Ordered,
                                "Ready" => OrderStatus::Ready,
                                "Completed" => OrderStatus::Completed,
                                "Cancelled" => OrderStatus::Cancelled,
                                _ => OrderStatus::Ready,
                            };
                            output_status.set(status);
                        })
                    />
                </div>
                
                {move || {
                    view! {
                        <Button
                            size=Size::Md
                            intent=Intent::Primary
                            state=if selected_categories.get().is_empty() { 
                                ComponentState::Disabled 
                            } else { 
                                ComponentState::Enabled 
                            }
                        >
                            "Create Station"
                        </Button>
                    }
                }}
            </form>
        </Card>
    }
}
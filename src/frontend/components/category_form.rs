use leptos::prelude::*;
use leptos::web_sys;
use crate::common::types::CreateCategoryRequest;
use crate::frontend::design_system::{
    Card, CardVariant, Input, Button, Text, Alert,
    theme::{Size, Intent},
    atoms::{InputType, TextVariant, FontWeight},
};

#[component]
pub fn CategoryForm<F>(on_submit: F) -> impl IntoView 
where
    F: Fn(CreateCategoryRequest) + 'static + Clone + Send,
{
    let name = RwSignal::new(String::new());
    let (error, set_error) = signal(Option::<String>::None);

    let on_submit_clone = on_submit.clone();
    let submit_form = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        // Clear previous error
        set_error.set(None);
        
        let request = CreateCategoryRequest {
            name: name.get().trim().to_string(),
        };

        // Validate
        if let Err(err) = request.validate() {
            set_error.set(Some(err));
            return;
        }

        // Submit
        on_submit_clone(request);
        
        // Clear form
        name.set(String::new());
    };

    view! {
        <Card variant=CardVariant::Default>
            <form on:submit=submit_form class="space-y-4">
                <Text 
                    variant=TextVariant::Heading 
                    size=Size::Lg 
                    weight=FontWeight::Semibold
                >
                    "Add New Category"
                </Text>
                
                {move || error.get().map(|err| view! {
                    <Alert intent=Intent::Danger size=Size::Sm>
                        {err}
                    </Alert>
                })}
                
                <div class="space-y-2">
                    <Text 
                        variant=TextVariant::Label 
                        size=Size::Sm 
                        weight=FontWeight::Medium
                        as_element="label"
                    >
                        "Category Name"
                    </Text>
                    <Input
                        input_type=InputType::Text
                        size=Size::Md
                        intent=Intent::Primary
                        value=name
                        placeholder="e.g., Drinks, Food, Snacks"
                        required=true
                        on_input=Callback::new(move |ev| name.set(event_target_value(&ev)))
                    />
                </div>
                
                <Button
                    size=Size::Md
                    intent=Intent::Primary
                    on_click=Callback::new(move |_| {
                        // The form submit will handle this
                    })
                >
                    "Add Category"
                </Button>
            </form>
        </Card>
    }
}
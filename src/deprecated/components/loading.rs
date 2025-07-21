use leptos::prelude::*;
use crate::frontend::design_system::{Spinner, SpinnerVariant};
use crate::frontend::design_system::theme::{Size, Intent};

#[component]
pub fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center p-6">
            <Spinner 
                size=Size::Lg 
                intent=Intent::Primary 
                variant=SpinnerVariant::Circle 
            />
        </div>
    }
}

#[component]
pub fn LoadingCard() -> impl IntoView {
    use crate::frontend::design_system::{Card, CardVariant};
    
    view! {
        <Card variant=CardVariant::Default class="animate-pulse">
            <div class="space-y-4">
                <div class="h-5 bg-gray-200 dark:bg-gray-700 rounded-lg w-3/4"></div>
                <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded-lg w-1/2"></div>
                <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded-lg w-full"></div>
                <div class="flex space-x-3 pt-2">
                    <div class="h-9 bg-gray-300 dark:bg-gray-600 rounded-lg w-24"></div>
                    <div class="h-9 bg-gray-300 dark:bg-gray-600 rounded-lg w-20"></div>
                </div>
            </div>
        </Card>
    }
}

#[component]
pub fn LoadingSkeleton() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
            <div class="space-y-6">
                <LoadingCard />
                <LoadingCard />
                <LoadingCard />
            </div>
            <div class="space-y-6">
                <LoadingCard />
                <LoadingCard />
            </div>
        </div>
    }
}

#[component] 
pub fn LoadingBanner() -> impl IntoView {
    use crate::frontend::design_system::{Alert, Text, TextVariant, FontWeight};
    
    view! {
        <Alert intent=Intent::Info size=Size::Md class="mb-6">
            <div class="flex items-center space-x-3">
                <Spinner size=Size::Sm intent=Intent::Info />
                <Text variant=TextVariant::Body weight=FontWeight::Medium>
                    "Loading data..."
                </Text>
            </div>
        </Alert>
    }
}
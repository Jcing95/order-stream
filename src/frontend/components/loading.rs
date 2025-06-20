use leptos::prelude::*;

#[component]
pub fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center p-6">
            <div class="relative">
                <div class="animate-spin rounded-full h-10 w-10 border-4 border-gray-200 border-t-blue-600 dark:border-gray-600 dark:border-t-blue-400 shadow-lg"></div>
                <div class="absolute inset-0 animate-ping rounded-full h-10 w-10 border-2 border-blue-400 opacity-20"></div>
            </div>
        </div>
    }
}

#[component]
pub fn LoadingCard() -> impl IntoView {
    view! {
        <div class="animate-pulse bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-600 p-6 shadow-lg hover:shadow-xl transition-all duration-300 backdrop-blur-sm">
            <div class="space-y-4">
                <div class="h-5 bg-gradient-to-r from-gray-200 to-gray-300 dark:from-gray-700 dark:to-gray-600 rounded-lg w-3/4"></div>
                <div class="h-4 bg-gradient-to-r from-gray-200 to-gray-300 dark:from-gray-700 dark:to-gray-600 rounded-lg w-1/2"></div>
                <div class="h-4 bg-gradient-to-r from-gray-200 to-gray-300 dark:from-gray-700 dark:to-gray-600 rounded-lg w-full"></div>
                <div class="flex space-x-3 pt-2">
                    <div class="h-9 bg-gradient-to-r from-blue-200 to-blue-300 dark:from-blue-700 dark:to-blue-600 rounded-lg w-24"></div>
                    <div class="h-9 bg-gradient-to-r from-gray-200 to-gray-300 dark:from-gray-700 dark:to-gray-600 rounded-lg w-20"></div>
                </div>
            </div>
        </div>
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
    view! {
        <div class="bg-gradient-to-r from-blue-50 via-indigo-50 to-purple-50 dark:from-blue-900/20 dark:via-indigo-900/20 dark:to-purple-900/20 border border-blue-200 dark:border-blue-700 rounded-xl p-4 mb-6 shadow-sm backdrop-blur-sm">
            <div class="flex items-center space-x-3">
                <div class="relative">
                    <div class="animate-spin rounded-full h-5 w-5 border-2 border-blue-200 border-t-blue-600 dark:border-blue-600 dark:border-t-blue-400"></div>
                    <div class="absolute inset-0 animate-pulse rounded-full h-5 w-5 bg-blue-400/20"></div>
                </div>
                <span class="text-blue-700 dark:text-blue-300 font-medium text-sm bg-white/50 dark:bg-gray-800/50 px-2 py-1 rounded-md">"Loading data..."</span>
            </div>
        </div>
    }
}
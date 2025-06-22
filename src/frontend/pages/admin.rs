use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::frontend::components::{
    tab_nav::TabNav,
    category_section::CategorySection,
    item_section::ItemSection,
    order_section::OrderSection,
    station_section::StationSection,
};
use crate::frontend::design_system::{
    Text, Alert, Spinner, ThemeSwitcher, Icon,
    TextVariant, FontWeight, SpinnerVariant,
    theme::{Size, Intent},
    atoms::IconVariant,
};
use crate::frontend::state::{admin::AdminState, theme::{ThemeState}};

#[component]
pub fn AdminPage() -> impl IntoView {
    let state = AdminState::new();
    let (active_tab, set_active_tab) = signal("categories".to_string());
    let _theme_state = expect_context::<ThemeState>();

    // Load data when component mounts
    Effect::new({
        let state = state.clone();
        move |_| {
            let state = state.clone();
            spawn_local(async move {
                state.load_all().await;
            });
        }
    });


    view! {
        <div class="container mx-auto p-6">
            <div class="flex justify-between items-center mb-8">
                <Text 
                    variant=TextVariant::Heading 
                    size=Size::Xl 
                    weight=FontWeight::Bold
                    as_element="h1"
                >
                    "Admin Panel"
                </Text>
                <ThemeSwitcher />
            </div>
            
            {move || state.error.get().map(|err| view! {
                <div class="mb-6">
                    <Alert intent=Intent::Danger>
                        <div class="flex items-center space-x-2">
                            <Icon name="x-circle" intent=Intent::Danger size=Size::Sm variant=IconVariant::Outline />
                            <span class="font-medium">{err}</span>
                        </div>
                    </Alert>
                </div>
            })}
            
            {move || {
                if state.loading.get() {
                    view! {
                        <div class="mb-6">
                            <Alert intent=Intent::Info>
                                <div class="flex items-center space-x-3">
                                    <Spinner size=Size::Sm variant=SpinnerVariant::Circle />
                                    <span class="font-medium">"Loading admin data..."</span>
                                </div>
                            </Alert>
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
            
            <TabNav 
                active_tab=active_tab.into()
                set_active_tab=set_active_tab
                tabs=vec![
                    ("categories", "Categories"),
                    ("items", "Items"),
                    ("orders", "Orders"),
                    ("stations", "Stations"),
                ]
            />
            
            // Tab content
            {move || {
                let state = state.clone();
                match active_tab.get().as_str() {
                    "categories" => {
                        view! {
                            <CategorySection 
                                categories=state.categories.read_only() 
                                on_submit=move |request| state.create_category(request) 
                                on_delete=move |category_id| state.delete_category(category_id) 
                            />
                        }.into_any()
                    },
                    "items" => {
                        view! {
                            <ItemSection 
                                categories=state.categories.read_only() 
                                items=state.items.read_only()
                                on_submit=move |request| state.create_item(request) 
                            />
                        }.into_any()
                    },
                    "orders" => {
                        view! {
                            <OrderSection 
                                orders=state.orders.read_only() 
                                on_create=move |_| state.create_order() 
                                on_delete=move |order_id| state.delete_order(order_id) 
                            />
                        }.into_any()
                    },
                    "stations" => {
                        view! {
                            <StationSection 
                                stations=state.stations.read_only()
                                categories=state.categories.read_only()
                                on_submit=move |request| state.create_station(request) 
                                on_delete=move |station_id| state.delete_station(station_id) 
                            />
                        }.into_any()
                    },
                    _ => view! {}.into_any()
                }
            }}
        </div>
    }
}
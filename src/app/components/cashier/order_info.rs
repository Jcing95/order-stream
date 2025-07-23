use leptos::prelude::*;
use crate::app::components::atoms::icons;
use crate::common::german_names::generate_german_name;

#[derive(Debug, Clone)]
pub struct OrderInfo {
    pub order_id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct OrderInfoState {
    info: ReadSignal<Option<OrderInfo>>,
    set_info: WriteSignal<Option<OrderInfo>>,
    is_expanded: ReadSignal<bool>,
    set_is_expanded: WriteSignal<bool>,
}

impl OrderInfoState {
    pub fn new() -> Self {
        let (info, set_info) = signal(None);
        let (is_expanded, set_is_expanded) = signal(false);
        
        Self {
            info,
            set_info,
            is_expanded,
            set_is_expanded,
        }
    }
    
    pub fn set_order_created(&self, order_id: String) {
        let name = generate_german_name(&order_id);
        let order_info = OrderInfo {
            order_id: order_id.clone(),
            name,
        };
        self.set_info.set(Some(order_info));
        self.set_is_expanded.set(true);
    }
    
    pub fn clear(&self) {
        self.set_info.set(None);
        self.set_is_expanded.set(false);
    }
    
    pub fn toggle_expanded(&self) {
        self.set_is_expanded.update(|expanded| *expanded = !*expanded);
    }
    
    pub fn collapse(&self) {
        self.set_is_expanded.set(false);
    }
    
    pub fn get_info(&self) -> ReadSignal<Option<OrderInfo>> {
        self.info
    }
    
    pub fn is_expanded(&self) -> ReadSignal<bool> {
        self.is_expanded
    }
}

pub fn provide() -> OrderInfoState {
    let order_info_state = OrderInfoState::new();
    provide_context(order_info_state.clone());
    order_info_state
}

pub fn get() -> OrderInfoState {
    expect_context::<OrderInfoState>()
}

#[component]
pub fn OrderInfoComponent() -> impl IntoView {
    let order_info_state = get();
    let info = order_info_state.get_info();
    let is_expanded = order_info_state.is_expanded();
    
    view! {
        <Show when=move || info.get().is_some()>
            <div class="mb-4">
                <Show
                    when=move || is_expanded.get()
                    fallback=|| view! {
                        // Collapsed state - compact bar
                        <CollapsedOrderInfo />
                    }
                >
                    // Expanded state - full info display
                    <ExpandedOrderInfo info=info />
                </Show>
            </div>
        </Show>
    }
}

#[component] 
fn CollapsedOrderInfo() -> impl IntoView {
    let order_info_state = get();
    
    view! {
        <button
            class="w-full p-3 bg-success/10 border border-success/30 rounded-xl text-left transition-all duration-300 hover:shadow-lg group"
            on:click=move |_| order_info_state.toggle_expanded()
        >
            <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3">
                    <div class="w-2 h-2 bg-success rounded-full animate-pulse"></div>
                    <span class="text-sm font-medium text-text">"Name der letzten Bestellung"</span>
                </div>
                <icons::ChevronDown attr:class="w-4 h-4 text-text-muted group-hover:text-text transition-colors"/>
            </div>
        </button>
    }
}

#[component]
fn ExpandedOrderInfo(info: ReadSignal<Option<OrderInfo>>) -> impl IntoView {
    let order_info_state = get();
    
    view! {
        <button
            class="w-full p-4 bg-success/5 border-2 border-success/30 rounded-xl shadow-lg text-left transition-all duration-300 hover:bg-success/10 group"
            on:click=move |_| order_info_state.collapse()
        >
            <div class="flex items-center justify-between mb-3">
                <div class="flex items-center space-x-3">
                    <div class="w-3 h-3 bg-success rounded-full animate-pulse"></div>
                    <h3 class="text-lg font-bold text-text">"Bestellungsname"</h3>
                </div>
                <icons::ChevronUp attr:class="w-4 h-4 text-text-muted group-hover:text-text transition-colors"/>
            </div>
            
            <div class="space-y-2">
                <div class="p-3 bg-surface-elevated rounded-lg border border-border">
                    <div class="text-2xl font-bold text-primary">
                        {move || info.get().as_ref().map(|i| i.name.clone()).unwrap_or_default()}
                    </div>
                </div>
                
                <div class="text-xs text-text-muted text-center mt-2">
                    "An den anderen Stationen wird dieser Name auch angezeigt!"
                </div>
            </div>
        </button>
    }
}
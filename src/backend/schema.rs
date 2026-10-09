// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
    }
}

diesel::table! {
    items (id) {
        id -> Int4,
        order_id -> Int4,
        product_id -> Int4,
        quantity -> Int4,
        price -> Float8,
        status -> Text,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        event_id -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        category_id -> Int4,
        price -> Float8,
        active -> Bool,
    }
}

diesel::table! {
    sessions (id) {
        id -> Text,
        user_id -> Uuid,
        expiry_date -> Timestamptz,
    }
}

diesel::table! {
    settings (id) {
        id -> Int4,
        active_event_id -> Nullable<Int4>,
    }
}

diesel::table! {
    station_categories (station_id, category_id) {
        station_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    station_input_statuses (station_id, status) {
        station_id -> Int4,
        status -> Text,
    }
}

diesel::table! {
    stations (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        output_status -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        password_hash -> Text,
        role -> Text,
    }
}

diesel::joinable!(items -> orders (order_id));
diesel::joinable!(items -> products (product_id));
diesel::joinable!(orders -> events (event_id));
diesel::joinable!(products -> categories (category_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(settings -> events (active_event_id));
diesel::joinable!(station_categories -> categories (category_id));
diesel::joinable!(station_categories -> stations (station_id));
diesel::joinable!(station_input_statuses -> stations (station_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    events,
    items,
    orders,
    products,
    sessions,
    settings,
    station_categories,
    station_input_statuses,
    stations,
    users,
);

pub mod category {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Create {
        pub name: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Update {
        pub name: Option<String>,
    }
}

pub mod event {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Create {
        pub name: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Update {
        pub name: Option<String>,
    }
}

pub mod order {
    use crate::common::types::OrderStatus;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Update {
        pub status: Option<OrderStatus>,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Create {
        pub event: String,
    }
}

pub mod station {
    use crate::common::types::OrderStatus;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Create {
        pub name: String,
        pub category_ids: Vec<String>,
        pub input_statuses: Vec<OrderStatus>,
        pub output_status: OrderStatus,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Update {
        pub category_ids: Option<Vec<String>>,
        pub input_statuses: Option<Vec<OrderStatus>>,
        pub output_status: Option<OrderStatus>,
    }
}

pub mod product {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Update {
        pub name: Option<String>,
        pub category_id: Option<String>,
        pub price: Option<f64>,
        pub active: Option<bool>,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Create {
        pub name: String,
        pub category_id: String,
        pub price: f64,
        pub active: bool,
    }
}

pub mod item {
    use crate::common::types::OrderStatus;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Create {
        pub order_id: String,
        pub product_id: String, // References the "products" table
        pub quantity: u32,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Update {
        pub product_id: Option<String>, // Allow changing the product (for corrections)
        pub quantity: Option<u32>,
        pub status: Option<OrderStatus>, // Allow updating status
    }
}

pub mod user {
    use crate::common::types::Role;
    
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Create {
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Update {
        pub email: Option<String>,
        pub password: Option<String>,
        pub role: Option<Role>
    }
}

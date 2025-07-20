pub mod category {
    use validator::Validate;
    
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Create {
        #[validate(length(min = 1, max = 64))]
        pub name: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Update {
        #[validate(length(min = 1, max = 64))]
        pub name: Option<String>,
    }
}

pub mod event {
    use validator::Validate;
    
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Create {
        #[validate(length(min = 1, max = 64))]
        pub name: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Update {
        #[validate(length(min = 1, max = 64))]
        pub name: Option<String>,
    }
}

pub mod order {
    use crate::common::types::OrderStatus;
    use validator::Validate;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Update {
        pub status: Option<OrderStatus>,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Create {
        #[validate(length(min = 1))]
        pub event: String,
    }
}

pub mod station {
    use crate::common::types::OrderStatus;
    use validator::Validate;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Create {
        #[validate(length(min = 1, max = 64))]
        pub name: String,
        #[validate(length(min = 1))]
        pub category_ids: Vec<String>,
        #[validate(length(min = 1))]
        pub input_statuses: Vec<OrderStatus>,
        pub output_status: OrderStatus,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Update {
        #[validate(length(min = 1))]
        pub category_ids: Option<Vec<String>>,
        #[validate(length(min = 1))]
        pub input_statuses: Option<Vec<OrderStatus>>,
        pub output_status: Option<OrderStatus>,
    }
}

pub mod product {
    use validator::Validate;
    
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Update {
        #[validate(length(min = 1, max = 100))]
        pub name: Option<String>,
        #[validate(length(min = 1))]
        pub category_id: Option<String>,
        #[validate(range(min = 0.0))]
        pub price: Option<f64>,
        pub active: Option<bool>,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Create {
        #[validate(length(min = 1, max = 100))]
        pub name: String,
        #[validate(length(min = 1))]
        pub category_id: String,
        #[validate(range(min = 0.0))]
        pub price: f64,
        pub active: bool,
    }
}

pub mod item {
    use crate::common::types::OrderStatus;
    use validator::Validate;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Create {
        #[validate(length(min = 1))]
        pub order_id: String,
        #[validate(length(min = 1))]
        pub product_id: String, // References the "products" table
        #[validate(range(min = 1))]
        pub quantity: u32,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Update {
        #[validate(length(min = 1))]
        pub product_id: Option<String>, // Allow changing the product (for corrections)
        #[validate(range(min = 1))]
        pub quantity: Option<u32>,
        pub status: Option<OrderStatus>, // Allow updating status
    }
}

pub mod user {
    use crate::common::types::Role;
    use validator::Validate;
    
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Create {
        #[validate(email)]
        pub email: String,
        #[validate(length(min = 8))]
        pub password: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
    pub struct Update {
        #[validate(email)]
        pub email: Option<String>,
        #[validate(length(min = 8))]
        pub password: Option<String>,
        pub role: Option<Role>
    }
}

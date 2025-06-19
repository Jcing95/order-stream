# Phase 1 - Order & OrderItem Models with Categories

## Overview
Build on Phase 0's solid foundation by adding proper category management and core order functionality. This phase establishes the data relationships and CRUD operations needed for order processing.

## Goals
- Type-safe category system with foreign key relationships
- Simplified but extensible Order and OrderItem models
- Sequential order numbering for customer-facing IDs
- Clean data model with minimal duplication
- Foundation for future filtering and state management

## Dependencies
No new cargo dependencies required - using existing SurrealDB and Leptos setup.

## Data Model Design

### 1. Category Model
```rust
// Common types (frontend/backend shared)
pub struct Category {
    pub id: String,
    pub name: String,
}

pub struct CreateCategoryRequest {
    pub name: String,
}

pub struct UpdateCategoryRequest {
    pub name: Option<String>,
}

// Database-specific record (backend only)
pub struct CategoryRecord {
    pub id: Thing,
    pub name: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    // Future expansion: color, icon, display_order, description, etc.
}
```

### 2. Updated Item Model
```rust
// Updated common Item type
pub struct Item {
    pub id: String,
    pub name: String,
    pub category_id: String,  // Foreign key to Category
    pub price: f64,
    pub active: bool,
}

// Updated ItemRecord with category_id
pub struct ItemRecord {
    pub id: Thing,
    pub name: String,
    pub category_id: String,  // FK relationship
    pub price: f64,
    pub active: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
```

### 3. Order Model
```rust
// Common types
pub struct Order {
    pub id: String,
    pub sequential_id: u32,    // Customer-facing order number (#001, #002, etc.)
    pub total_price: f64,      // Calculated from OrderItems
    pub status: OrderStatus,
}

pub enum OrderStatus {
    Draft,      // Being built, not yet ordered
    Ordered,    // Paid and submitted
    Ready,      // All items ready for pickup
    Completed,  // Handed out to customer
    Cancelled,  // Cancelled before completion
}

pub struct UpdateOrderRequest {
    pub status: Option<OrderStatus>,
}

// Database-specific record (backend only)
pub struct OrderRecord {
    pub id: Thing,
    pub sequential_id: u32,    // Auto-generated incremental ID
    pub event_id: String,      // Backend metadata for future multi-event support
    pub total_price: f64,
    pub status: OrderStatus,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
```

### 4. OrderItem Model
```rust
// Common types
pub struct OrderItem {
    pub id: String,
    pub order_id: String,      // Reference to Order
    pub item_id: String,       // Reference to Item
    pub quantity: u32,
    pub price: f64,           // Unit price when ordered (historical snapshot)
}

pub struct CreateOrderItemRequest {
    pub order_id: String,
    pub item_id: String,
    pub quantity: u32,
}

pub struct UpdateOrderItemRequest {
    pub item_id: Option<String>,  // Allow changing the item (for corrections)
    pub quantity: Option<u32>,
}

// Database-specific record (backend only)
pub struct OrderItemRecord {
    pub id: Thing,
    pub order_id: String,     // FK to OrderRecord
    pub item_id: String,      // FK to ItemRecord  
    pub quantity: u32,
    pub price: f64,          // Snapshot of item price at order time
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
```

## Detailed Action Plan

### 1. Categories Implementation
- [ ] Create `src/common/types.rs` entries for Category types
- [ ] Create `src/backend/database/model/category.rs` with CategoryRecord
- [ ] Create `src/backend/database/dao/category.rs` with full CRUD operations
- [ ] Create `src/backend/services/categories.rs` for business logic
- [ ] Create `src/backend/api/categories.rs` with server functions:
  - `get_categories() -> Vec<Category>`
  - `create_category(CreateCategoryRequest) -> Category`
  - `update_category(id, UpdateCategoryRequest) -> Category`
  - `delete_category(id) -> ()`
- [ ] Update admin interface with category management section

### 2. Update Items for Categories
- [ ] Update `Item` and `ItemRecord` to include `category_id` field
- [ ] Update item DAO operations to handle category relationships
- [ ] Update item server functions to work with categories
- [ ] Update admin item form with category dropdown selection
- [ ] Add validation to ensure category exists when creating/updating items

### 3. Orders Implementation
- [ ] Create Order types in `src/common/types.rs`
- [ ] Create `src/backend/database/model/order.rs` with OrderRecord
- [ ] Create `src/backend/database/dao/order.rs` with CRUD operations
- [ ] Implement sequential ID generation logic in DAO
- [ ] Create `src/backend/services/orders.rs` for business logic
- [ ] Create `src/backend/api/orders.rs` with server functions:
  - `get_orders() -> Vec<Order>`
  - `create_order() -> Order` (creates empty draft order)
  - `get_order(id) -> Option<Order>`
  - `update_order(id, UpdateOrderRequest) -> Order`
  - `delete_order(id) -> ()`

### 4. OrderItems Implementation
- [ ] Create OrderItem types in `src/common/types.rs`
- [ ] Create `src/backend/database/model/order_item.rs` with OrderItemRecord
- [ ] Create `src/backend/database/dao/order_item.rs` with CRUD operations
- [ ] Create `src/backend/services/order_items.rs` for business logic
- [ ] Create `src/backend/api/order_items.rs` with server functions:
  - `get_order_items(order_id) -> Vec<OrderItem>`
  - `create_order_item(CreateOrderItemRequest) -> OrderItem`
  - `update_order_item(id, UpdateOrderItemRequest) -> OrderItem`
  - `delete_order_item(id) -> ()`
- [ ] Implement order total calculation logic

### 5. Admin Interface Extensions
- [ ] Create `src/frontend/components/category_form.rs`
- [ ] Create `src/frontend/components/category_list.rs`
- [ ] Create `src/frontend/components/order_form.rs`
- [ ] Create `src/frontend/components/order_list.rs`
- [ ] Create `src/frontend/components/order_detail.rs`
- [ ] Update admin page to include all new management sections
- [ ] Add navigation between different admin sections

### 6. Business Logic & Validation
- [ ] Order total calculation from OrderItems
- [ ] Sequential ID generation and uniqueness
- [ ] Order status transition validation
- [ ] Category foreign key validation
- [ ] Prevent deletion of categories with associated items
- [ ] Prevent deletion of items with associated order items
- [ ] Allow item_id changes only for Draft status orders

## Success Criteria
- [ ] Can create, read, update, delete categories through admin interface
- [ ] Items properly reference categories via foreign keys
- [ ] Can create empty draft orders with auto-generated sequential IDs
- [ ] Can add/remove items to/from orders
- [ ] Can change item_id and quantity in draft orders
- [ ] Order totals calculate correctly from order items
- [ ] All CRUD operations work through server functions
- [ ] Type-safe relationships prevent orphaned data
- [ ] Clean admin interface for managing all entities

## Database Relationships
```
Category 1:N Item (category_id FK)
Order 1:N OrderItem (order_id FK)  
Item 1:N OrderItem (item_id FK)
```

## Key Architecture Decisions

### Order Creation Workflow
- `create_order()` creates empty draft order (no request body needed)
- Orders get populated by adding OrderItems via `create_order_item()`
- Status transitions: Draft → Ordered (payment) → Ready → Completed

### OrderItem Flexibility
- Allow changing `item_id` and `quantity` for corrections
- Should be restricted to Draft status orders only
- Price updates automatically when item_id changes

### Sequential Order IDs
- Use auto-incrementing counter in database
- Start from 1, increment for each new order
- Display to customers as "#001", "#002", etc.
- Internal SurrealDB Thing IDs for system use

### Price Snapshotting
- OrderItems capture item price at order time
- Ensures historical accuracy if menu prices change
- Order totals remain consistent over time
- Price updates when item_id is changed

### Simplified OrderItems
- Minimal data model - most info derived from relationships
- item_id FK allows querying current item details
- price field preserves historical pricing

### Category Extensibility
- CategoryRecord designed for future expansion
- Common Category type stays minimal
- Room for color, icon, ordering, etc. later

## Testing Strategy
- [ ] Test category CRUD operations
- [ ] Test item-category relationships
- [ ] Test order creation (empty draft)
- [ ] Test order item addition/removal
- [ ] Test order item corrections (item_id and quantity changes)
- [ ] Test order total calculations
- [ ] Test foreign key constraint validation
- [ ] Test business logic restrictions (Draft-only edits)
- [ ] Test admin interface workflows

## Notes
- Orders start as empty drafts - no create request needed
- OrderItems can be corrected (item_id/quantity) in Draft orders only
- Focus on data relationships and type safety
- Sequential IDs for customer-facing order numbers
- Price snapshotting for historical accuracy
- Minimal but complete CRUD operations for all entities
- Clean separation between common types and database records
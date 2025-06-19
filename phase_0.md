# Phase 0 - Infrastructure Setup

## Overview
Set up the basic project structure, database connection, and simple item management to establish a solid foundation.

## Goals
- Get everything compiling and connecting to SurrealDB
- Basic project structure with defined modules
- Simple admin interface for item management
- Foundation for all future phases

## Dependencies
Add using cargo commands:
```bash
cargo add surrealdb --optional --features json
cargo add serde --features derive
cargo add chrono --features serde
cargo add uuid --optional --features v4,serde
```

## Detailed Action Plan

### 1. Project Structure Setup
- [ ] Create module directories:
  - `src/common/`
  - `src/frontend/pages/`
  - `src/frontend/components/`
  - `src/frontend/state/`
  - `src/backend/api/`
  - `src/backend/services/`
  - `src/database/model/`
  - `src/database/service/`
- [ ] Create `mod.rs` files in each module
- [ ] Update `main.rs` and `lib.rs` to import new modules

### 2. Configuration
- [ ] Create `.env` file with database configuration
- [ ] Create `src/backend/config.rs` to parse environment variables

### 3. Data Models
- [ ] Create `src/common/types.rs` with `Item` struct and request/response types
- [ ] Create `src/common/errors.rs` with custom error types
- [ ] Create `src/database/model/item.rs` with SurrealDB schema

### 4. Database Layer
- [ ] Create `src/database/service/connection.rs` for DB connection setup
- [ ] Create `src/database/service/item.rs` with CRUD operations:
  - `create_item(item: CreateItemRequest) -> Result<Item>`
  - `get_items() -> Result<Vec<Item>>`
  - `get_item(id: String) -> Result<Option<Item>>`
  - `update_item(id: String, item: UpdateItemRequest) -> Result<Item>`
  - `delete_item(id: String) -> Result<()>`

### 5. Backend API
- [ ] Create `src/backend/api/items.rs` with HTTP endpoints:
  - `GET /api/items` - List all items
  - `POST /api/items` - Create new item
  - `GET /api/items/:id` - Get specific item
  - `PUT /api/items/:id` - Update item
  - `DELETE /api/items/:id` - Delete item
- [ ] Add routes to `main.rs` Axum router
- [ ] Connect API endpoints to database service via backend services

### 6. Backend Services
- [ ] Create `src/backend/services/items.rs` for business logic layer
- [ ] Connect backend services to database services

### 7. Frontend
- [ ] Create `src/frontend/components/item_form.rs` for adding/editing items
- [ ] Create `src/frontend/components/item_list.rs` for displaying items
- [ ] Create `src/frontend/pages/admin.rs` combining both components (single admin page)
- [ ] Add admin route to `src/app.rs`

### 8. Integration & Testing
- [ ] Test database connection
- [ ] Test API endpoints (create, read, update, delete items)
- [ ] Test frontend admin interface
- [ ] Verify end-to-end workflow: add item via UI → save to DB → display in list

## Success Criteria
- [ ] Project compiles without errors
- [ ] Can connect to SurrealDB successfully
- [ ] Can create, read, update, and delete items via admin interface
- [ ] All modules properly structured and importable
- [ ] Clean separation of concerns between layers

## Data Model Details

### Item Struct
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,           // SurrealDB format: "item:uuid"
    pub name: String,
    pub category: String,
    pub price: f64,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub category: String,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub price: Option<f64>,
    pub active: Option<bool>,
}
```

## Environment Configuration
`.env` file should contain:
```env
SURREAL_URL=ws://127.0.0.1:8000/rpc
SURREAL_USER=root
SURREAL_PASS=root
SURREAL_DB=orderstream
SURREAL_NS=production
```

## Notes
- Keep categories as simple strings for now (will become separate model in later phases)
- Focus on getting basic functionality working before optimization
- Use basic error handling (will be improved in later phases)
- Admin interface should be functional but doesn't need to be pretty yet
- Use SurrealDB's `item:uuid` ID format throughout
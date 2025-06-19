# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Order Stream** is a simple and efficient tool for streamlining food and drink logistics at small events. Built as a Leptos full-stack web application using the Axum web server.

### Purpose
Order Stream helps event organizers manage food and beverage orders efficiently by providing a streamlined interface for order taking, tracking, and fulfillment at small events like parties, meetups, or community gatherings.

### Tech Stack
- **Frontend**: Leptos with WebAssembly compilation
- **Backend**: Axum server 
- **Database**: SurrealDB (relational)
- **Real-time**: WebSockets for live order updates
- **Styling**: Tailwind CSS
- **Testing**: Playwright for end-to-end tests
- **Build Tool**: cargo-leptos

## Architecture

### Station-Based Workflow
Order Stream is designed around stationary devices at different stations:

**Order Flow:**
1. **Cash Register** (1-2 tablets): Staff takes orders, marks items as ordered after payment
2. **Preparation Stations** (Bar/Food/Kitchen/Drinks): See real-time orders filtered by category
3. **Fulfillment**: Staff mark items as "ready" then "completed" when handed out

**Key Features:**
- Real-time order synchronization via WebSockets
- Category-based filtering (customizable: Drinks, Food, etc.)
- Trust-based system (no customer order numbers displayed)
- Order tracking with backend order numbers for future expansion
- Role-optimized UI views for each station type
- Order grouping for customer context and basic verification

**User Roles:**
- **Admin**: Full system configuration, menu management, and all permissions
- **Cashier**: Create/cancel orders, mark orders as paid
- **Staff**: View orders, update order states (ready/completed)

**Account Types:**
- **Personal accounts**: Individual login on personal devices
- **Stationary accounts**: Shared accounts for station tablets

**Order States:**
`draft` (synced for offline resilience) → `ordered` (after payment) → `ready` → `completed` (or `cancelled`)

**Order Completion:**
- Per-category completion (drinks complete separately from food)
- Full order completion when all categories are done
- Category-specific visibility (completed items disappear from relevant stations)

**Device Management:**
- Account-based login for role assignment
- URL-based view switching (/cashier, /bar, /kitchen, etc.)
- Event-based organization with EventID tracking

**Data Model:**
- **Orders**: Sequential ID, EventID, timestamp, total price, status
- **Items**: Name, category, current price (admin-managed only)
- **OrderItems**: Links orders to items with quantity, paid price, and category-specific status
- **Categories**: Customizable (Drinks, Food, etc.)
- **Users**: Role-based accounts (Admin/Cashier/Staff), personal or stationary
- **Events**: For historical tracking and multi-event support
- **StateChanges**: Audit trail tracking when order states change (for analytics)

**Station Views:**
- Filtered by category but show order context
- Expandable order cards to reveal full order details
- Visual grouping by order for customer verification

**Admin Configuration:**
- User accounts (roles, permissions, personal vs stationary)
- Menu items and pricing (exclusive admin control)
- Categories and station types
- Station layout (which categories each station sees)
- Order display settings (completion timeout, visual preferences)
- Event management and settings
- System monitoring (connected devices, connection status)
- Backup and export (order data, analytics, configuration)
- Order history and analytics (sales, pricing, state change timestamps)

**Offline Resilience:**
- Local state management for unstable connections
- Queue pending actions when offline
- Conflict resolution for concurrent edits
- Progressive sync when connection restored

### Code Structure

**Modular Organization:**
```
src/
├── common/           # Shared types and error definitions
├── frontend/
│   ├── pages/        # Route components (/cashier, /bar, /admin, etc.)
│   ├── components/   # Reusable UI components
│   └── state/        # Frontend state management and hydration
├── backend/
│   ├── api/          # HTTP endpoints and WebSocket handlers
│   ├── services/     # Business logic (order processing, auth, etc.)
│   └── config.rs     # App configuration and environment variables
└── database/
    ├── model/        # Schema definitions
    └── service/      # Database operations (importable by backend services)
```

**Entry Points:**
- `src/main.rs`: Axum server entry point for SSR builds
- `src/lib.rs`: WebAssembly hydration entry point for client builds
- `src/app.rs`: Main application component with routing and UI
- `end2end/`: Playwright test suite

**Configuration:**
- `.env` file for secrets and environment variables
- `backend/config.rs` for structured configuration management

The application uses Leptos's SSR (Server-Side Rendering) with hydration, where the server renders the initial HTML and the client takes over with WASM for interactivity. Leptos feature flags (`ssr` vs `hydrate`) handle compilation differences automatically.

## Development Phases

**MVP Phase 0 - Infrastructure (First Priority):**
- Basic project structure with defined modules
- SurrealDB connection and configuration
- Simple `Item` model (name, price, category)
- Basic CRUD operations for items (admin only)  
- One simple admin page to add/list items
- Get everything compiling and connecting to database

**Future Planned Phases:**
- **Phase 1**: Add `Order` and `OrderItem` models with basic CRUD
- **Phase 2**: Add cashier interface for order creation
- **Phase 3**: Add basic staff interface for order viewing
- **Phase 4**: Implement WebSocket real-time synchronization
- **Phase 5**: Add user authentication and role management
- **Phase 6**: Implement offline resilience features
- **Phase 7**: Add advanced admin features and analytics

*Goal: Maintain working state after each phase with minimal viable increments*

**Phase Documentation:**
Each phase has detailed implementation plans in separate files (e.g., `phase_0.md`, `phase_1.md`, etc.)

## Development Guidelines

### Dependency Management
- **NEVER** edit `Cargo.toml` manually for dependencies
- **ALWAYS** use `cargo add` command to add dependencies
- **ALWAYS** use `cargo remove` command to remove dependencies
- Use cargo flags for proper configuration: `--optional`, `--features`, `--no-default-features`
- This ensures latest compatible versions and proper feature handling

Examples:
```bash
cargo add surrealdb --features json
cargo add serde --features derive
cargo add tokio --optional --features full
cargo remove old-dependency
```

## Development Commands

### Installation Requirements
```bash
cargo install cargo-leptos --locked
rustup toolchain install nightly --allow-downgrade
rustup target add wasm32-unknown-unknown
npm install -g sass
cd end2end && npm install
```

### Database Setup
**Note: These steps should be performed by the user, not Claude Code**

```bash
# Install SurrealDB
curl -sSf https://install.surrealdb.com | sh

# Start development instance (memory-based, fast restarts)
surreal start --log debug --user root --pass root memory
```

**Environment Configuration:**
Add to `.env` file:
```env
SURREAL_URL=ws://127.0.0.1:8000/rpc
SURREAL_USER=root
SURREAL_PASS=root
SURREAL_DB=orderstream
SURREAL_NS=production
```

### Development
```bash
cargo leptos watch  # Start development server with hot reload
```

### Building
```bash
cargo leptos build --release  # Production build
```

### Testing
```bash
cargo leptos end-to-end        # Run Playwright tests in development
cargo leptos end-to-end --release  # Run Playwright tests against production build
```

## Key Configuration

- Uses Rust nightly toolchain (specified in `rust-toolchain.toml`)
- Tailwind CSS configured via `input.css`
- Server runs on `127.0.0.1:3000` by default
- Reload port: `3001`
- Build artifacts: `target/site/` and `target/server/`

## Features System

The project uses Cargo features to control compilation:
- `ssr`: Server-side rendering dependencies (Axum, Tokio, etc.)
- `hydrate`: Client-side WebAssembly dependencies
# 🍽️ OrderStream

**A real-time ordering and kitchen management system for small events**

OrderStream is a specialized platform designed for smaller events. It streamlines the entire food service workflow from order taking to kitchen preparation with real-time coordination.

## 🎯 What is OrderStream?

OrderStream is perfect for **event organizers** who need:

- **Real-time order coordination** for temporary or changing kitchen setups
- **Simple cashier interface** for volunteer staff or event workers
- **Flexible station management** for pop-up kitchen configurations
- **Event-based setups** that can be quickly set up and torn down
- **Live updates** On the fly Menu updates

## ✨ Key Features
### 👨‍🍳 **Flexible Kitchen Stations**
- Adaptable station views for temporary kitchen setups
- Orders with fun and memorable names for easy identification
- Simple status workflow: Ordered → Ready → Completed (will be customizable)
- Perfect for volunteer kitchen staff with minimal training

### 🏪 **Cashier View**
- Quick product selection organized by categories
- Real-time cart building with automatic price overview
- Simple "Alles Bezahlt!" (All Paid!) workflow
- Event-specific menus and pricing


### 🎪 **Event Administration**
- Quick event setup and menu configuration
- Temporary station assignments for pop-up kitchens
- User management for event staff and volunteers
- Event-specific settings and real-time configuration changes

### 🔄 **Real-time Event Coordination**
- WebSocket-powered live updates across all devices
- Instant order notifications to kitchen stations
- Real-time status updates perfect for fast-paced small events
- Seamless coordination for small, agile teams

## 🎨 Event-Friendly Design

- **Clean Interface**: Easy to use under event pressure
- **Mobile-First**: Works on any device your staff brings
- **Large Touch Targets**: Perfect for busy volunteers
- **Dark/Light Themes**: Adapts to any event environment
- **High Contrast**: Readable in various lighting conditions


## 👥 User Roles for Events
Will be customizable in the future

| Role | Perfect For | Capabilities |
|------|-------------|--------------|
| **Visitor** | Event Volunteers | Starting role on signup |
| **Staff** | Kitchen Staff | Process orders at assigned stations |
| **Cashier** | Event Cashiers | Take orders and manage payment |
| **Admin** | Event Organizers | Complete event setup and management |

## 📖 How Your Event Works

### 1. **Pre-Event Setup**
Event organizers quickly configure:
- Create your event menu with categories and pricing
- Set up kitchen stations based on your temporary setup
- Add event staff and assign roles

### 2. **Event Order Taking**
Cashiers or volunteers use the simple interface to:
- Select items from your event menu
- Build customer orders with quantities
- Process payments and submit orders
- Orders instantly appear at relevant kitchen stations

### 3. **Kitchen Operations**
Kitchen volunteers see exactly what they need:
- Station-specific views showing only relevant items
- Orders with fun and memorable names for easy identification
- Simple status updates as food is prepared
- Real-time coordination with other stations

### 4. **Live Event Coordination**
The system keeps everyone synchronized:
- New orders appear instantly at kitchen stations
- Status updates flow in real-time
- Event organizers can monitor overall progress
- No confusion, no missed orders, no chaos

## 🚀 Project setup

### Prerequisites

Install [Rust](https://www.rust-lang.org/tools/install), [Leptos](https://book.leptos.dev/getting_started/) + [TailwindCLI](https://tailwindcss.com/docs/installation/tailwind-cli) and [SurrealDB](https://surrealdb.com/install) as Database

create .env file with Database Connection info in project root:
```Example env
SURREAL_URL=127.0.0.1:8000
SURREAL_USER=root
SURREAL_PASS=root
SURREAL_DB=orderstream
SURREAL_NS=dev
```

## Development usage
```bash
cargo leptos watch              # Start development server
```

## 📄 License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.


---

## TODO:
* [ ] Automatic websocket reconnection
* [ ] Product sorting in admin panel
* [ ] Ressource groups + event based products and stations
* [ ] Customizable order states
* [ ] Comment feature for orders in cashier view
* [ ] Product descriptions
* [ ] Order name collision handling
* [ ] Sort orders by creation date
* [ ] Prevent product deletion but allow hiding
* [ ] RBAC + Role customization view/edit/create
* [ ] Internationalization
* [ ] Last order overview for Cashier view
---


Built with ❤️ for amazing events using Rust and Leptos.
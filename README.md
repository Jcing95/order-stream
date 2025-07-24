# 🍽️ OrderStream

**A real-time ordering and kitchen management system for small events**

OrderStream is a specialized platform designed for small events like community gatherings, private parties, festivals, and pop-up dining experiences. It streamlines the entire food service workflow from order taking to kitchen preparation with real-time coordination.

## 🎯 What is OrderStream?

OrderStream is perfect for **small event organizers** who need:

- **Real-time order coordination** for temporary kitchen setups
- **Simple cashier interface** for volunteer staff or event workers
- **Flexible station management** for pop-up kitchen configurations
- **Event-based operations** that can be quickly set up and torn down
- **Live updates** that keep small teams perfectly synchronized

## ✨ Key Features

### 🏪 **Event Cashier System**
- Quick product selection organized by categories
- Real-time cart building with automatic pricing
- Simple "Alles Bezahlt!" (All Paid!) workflow
- Event-specific menus and pricing

### 👨‍🍳 **Flexible Kitchen Stations**
- Adaptable station views for temporary kitchen setups
- Orders with fun German names for easy identification (e.g., "Bestellung 'Günther'")
- Simple status workflow: Draft → Ordered → Ready → Completed
- Perfect for volunteer kitchen staff with minimal training

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

## 👥 User Roles for Events

| Role | Perfect For | Capabilities |
|------|-------------|--------------|
| **Visitor** | Event Attendees | Basic system access (if needed) |
| **Staff** | Kitchen Volunteers | Process orders at assigned stations |
| **Cashier** | Event Cashiers | Take orders and monitor kitchen progress |
| **Admin** | Event Organizers | Complete event setup and management |

## 🎪 Perfect Use Cases

- **Community Events**: Church dinners, school fundraisers, local festivals
- **Private Parties**: Wedding receptions, birthday celebrations, corporate events
- **Pop-up Dining**: Food trucks, temporary restaurants, catering operations
- **Small Festivals**: Music festivals, craft fairs, farmers markets
- **Volunteer Operations**: Charity events, community kitchens, disaster relief

## 🚀 Quick Event Setup

### Prerequisites

```bash
# Install Rust nightly
rustup toolchain install nightly --allow-downgrade

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install cargo-leptos
cargo install cargo-leptos --locked

# Install Sass (for styling)
npm install -g sass
```

### Getting Started for Your Event

1. **Clone and setup**
   ```bash
   git clone <repository-url>
   cd order-stream
   npm install
   cd end2end && npm install && cd ..
   ```

2. **Configure for your event**
   ```bash
   # Set up your event configuration
   cp .env.example .env
   # Edit .env with your event details
   ```

3. **Start your event system**
   ```bash
   cargo leptos watch
   ```

   Access at `http://127.0.0.1:3000`

## 🏗️ Event-Focused Architecture

### Core Event Components
- **Events**: Time-based periods (breakfast, lunch, dinner service)
- **Orders**: Individual customer orders with unique German names
- **Items**: Menu items within orders with quantity and preparation status
- **Products**: Event menu items with pricing
- **Categories**: Menu organization (appetizers, mains, desserts, drinks)
- **Stations**: Flexible kitchen workstations for event setup
- **Users**: Event staff with role-based access

### Optimized for Small Events
- **Quick Setup**: Get running in minutes, not hours
- **Volunteer-Friendly**: Intuitive interface requiring minimal training
- **Flexible Configuration**: Adapt to any event size or kitchen setup
- **Real-time Updates**: Perfect for fast-paced event environments
- **German Name Fun**: Orders get memorable names like "Bestellung 'Brunhilde'" for easy kitchen communication

## 📖 How Your Event Works

### 1. **Pre-Event Setup** (5-10 minutes)
Event organizers quickly configure:
- Create your event menu with categories and pricing
- Set up kitchen stations based on your temporary setup
- Add event staff and assign roles
- Configure the active event period

### 2. **Event Order Taking**
Cashiers or volunteers use the simple interface to:
- Select items from your event menu
- Build customer orders with quantities
- Process payments and submit orders
- Orders instantly appear at relevant kitchen stations

### 3. **Kitchen Operations**
Kitchen volunteers see exactly what they need:
- Station-specific views showing only relevant items
- Orders with fun German names for easy identification
- Simple status updates as food is prepared
- Real-time coordination with other stations

### 4. **Live Event Coordination**
The system keeps everyone synchronized:
- New orders appear instantly at kitchen stations
- Status updates flow in real-time
- Event organizers can monitor overall progress
- No confusion, no missed orders, no chaos

## 🎯 Why OrderStream for Small Events?

- **⚡ Fast Setup**: From zero to serving in under 10 minutes
- **👥 Volunteer-Friendly**: Intuitive enough for untrained staff
- **📱 Mobile-Ready**: Works on phones, tablets, and computers
- **🔄 Real-Time**: Perfect for dynamic event environments
- **🎪 Event-Focused**: Built specifically for temporary operations
- **💝 German Fun**: Memorable order names add charm to your event

## 🔧 Available Commands

```bash
# Development
cargo leptos watch              # Start development server
cargo leptos build --release    # Build for production

# Testing
cargo leptos end-to-end         # Test your setup before the event
```

## 🚀 Production Deployment for Events

### Quick Production Build

```bash
cargo leptos build --release
```

### Environment Setup

```sh
export LEPTOS_OUTPUT_NAME="order-stream"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="0.0.0.0:3000"

# Event configuration
export DATABASE_URL="your-database-url"
export SESSION_SECRET="your-session-secret"
```

## 🎨 Event-Friendly Design

- **Clean Interface**: Easy to use under event pressure
- **Large Touch Targets**: Perfect for busy volunteers
- **Dark/Light Themes**: Adapts to any event environment
- **Mobile-First**: Works on any device your staff brings
- **High Contrast**: Readable in various lighting conditions

## 🧪 Testing Before Your Event

```bash
# Test everything works before your event
cargo leptos end-to-end
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

Perfect for your next event! For questions:

- **Issues**: [GitHub Issues](../../issues)
- **Documentation**: Check `/docs` for event setup guides
- **Community**: [GitHub Discussions](../../discussions)

---

Built with ❤️ for amazing small events using Rust and Leptos.
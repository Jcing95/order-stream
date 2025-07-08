# Authentication Implementation Plan

## Core Requirements
1. **Route Protection**: Immediate redirect to `/signin` for unauthenticated users
2. **Session Strategy**: Server-side session cookies only (no localStorage)
3. **Error Handling**: Auto-refresh session, fallback to forced re-login
4. **Role-Based UI**: Navbar shows/hides options based on user role
5. **Loading States**: Proper loading components during auth checks

## Implementation Phases

**Phase 1: Core Auth State** ✅
- ✅ Create `src/frontend/state/auth.rs` with reactive auth context
- ✅ Auto-initialize auth on app startup with loading state
- ✅ Session refresh logic with fallback to logout (basic implementation)
- ✅ Context provider/consumer hooks
- ✅ Login page integration with auth context

**Phase 2: Route Guards** ✅
- ✅ Create route protection wrapper components (implemented as direct protected components)
- ✅ Immediate redirect logic for protected routes
- ✅ Public route allowlist (signin, home)
- ✅ Protected route wrapper that checks auth before rendering
- ✅ Role-based access control for admin/cashier routes
- ✅ Loading spinners during auth checks

**Phase 3: Role-Based Navigation** ✅
- ✅ Update navbar with role-based visibility:
  - Admin: Admin, Cashier, Stations
  - Cashier: Cashier, Stations  
  - Staff: Stations only
- ✅ Login/logout buttons based on auth state
- ✅ User info display in navbar (email + role)
- ✅ App brand/logo in navbar
- ✅ Logout functionality with redirect to home page

**Phase 4: Enhanced Login Flow** ❌
- Redirect authenticated users away from login page
- Post-login redirect to appropriate role dashboard
- Better error messaging and validation
- Handle login/logout state transitions

**Phase 5: Loading & Error States** ❌
- Loading spinners during auth checks
- Graceful session expiry handling
- Network error recovery
- Retry mechanisms for failed auth requests

## Role-Based Route Access Matrix
```
Admin -> /admin, /cashier, /stations/*
Cashier -> /cashier, /stations/*  
Staff -> /stations/*
Everyone -> /signin (when not authenticated)
Public -> / (home page - no auth required)
```

## Session Refresh Strategy
- Check session validity on route changes
- Auto-refresh if session is close to expiry (TODO: implement expiry detection)
- Force logout if refresh fails
- Show loading during refresh attempts
- Handle network errors gracefully

## Current Status
- **Phase**: Implementation Complete - Fixing Hydration Issues
- **Next Step**: End-to-end testing
- **Last Updated**: Fixed hydration mismatches and infinite loading issues

## Recent Fixes
- ✅ Fixed SSR vs hydration mismatches by making auth initialization client-side only
- ✅ Removed Resource calls from pages that caused hydration errors
- ✅ Updated pages to use auth context instead of direct server function calls
- ✅ Fixed home page to show different content based on auth status

## Implementation Notes
Keep this file updated as we progress through each phase. Mark completed phases with ✅ and current work with 🚧.
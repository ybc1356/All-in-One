# Budget Dashboard Implementation Plan

## Overview
Implement a budget dashboard feature with category breakdown visualization, monthly time period view. Backend-first approach.

---

## Phase 1: Backend (Rust)

### Step 1: Database Schema
**Goal:** Define PostgreSQL/SQLite tables for the data model
**Files:**
- `backend/src/db/schema.rs`

**Tables:**
- `users` - id, email, password_hash, created_at
- `categories` - id, name, icon, color, user_id
- `budgets` - id, user_id, category_id, amount_limit, period (monthly), created_at
- `transactions` - id, user_id, category_id, amount, description, date, merchant

### Step 2: Models
**Goal:** Create Rust structs with serialization
**Files:**
- `backend/src/models/mod.rs`
- `backend/src/models/user.rs`
- `backend/src/models/budget.rs`
- `backend/src/models/category.rs`
- `backend/src/models/transaction.rs`

### Step 3: Budget CRUD
**Goal:** Full CRUD operations for budgets
**Files:**
- `backend/src/services/budget_service.rs`
- `backend/src/api/routes/budgets.rs`

**Endpoints:**
- `GET /budgets` - List all budgets for user
- `POST /budgets` - Create budget
- `PUT /budgets/{id}` - Update budget
- `DELETE /budgets/{id}` - Delete budget

### Step 4: Category Endpoints
**Goal:** List categories (needed for breakdown)
**Files:**
- `backend/src/services/category_service.rs`
- `backend/src/api/routes/categories.rs`

**Endpoints:**
- `GET /categories` - List categories

### Step 5: Dashboard Aggregation
**Goal:** Calculate spending by category for a given month
**Files:**
- `backend/src/services/dashboard_service.rs`
- `backend/src/api/routes/dashboard.rs`

**Endpoints:**
- `GET /dashboard?month=YYYY-MM` - Get spending breakdown by category

**Response shape:**
```json
{
  "month": "2024-01",
  "categories": [
    {
      "category_id": "uuid",
      "category_name": "Food",
      "budget_limit": 500.00,
      "spent": 320.50,
      "remaining": 179.50
    }
  ],
  "total_budget": 2000.00,
  "total_spent": 1200.00
}
```

---

## Phase 2: iOS Dashboard UI

### Step 1: Models
**Goal:** Swift Codable models mirroring backend
**Files:**
- `ios/AllIn1/Models/Budget.swift`
- `ios/AllIn1/Models/Category.swift`
- `ios/AllIn1/Models/Transaction.swift`

### Step 2: API Client
**Goal:** Network layer to call backend
**Files:**
- `ios/AllIn1/Services/APIClient.swift`

### Step 3: Budget ViewModel
**Goal:** State management for dashboard
**Files:**
- `ios/AllIn1/ViewModels/BudgetViewModel.swift`

### Step 4: Dashboard View
**Goal:** Ring chart showing category breakdown
**Files:**
- `ios/AllIn1/Views/Dashboard/DashboardView.swift`

**Components:**
- Month selector (previous/next)
- Ring chart (Swift Charts)
- Category rows with progress bars
- Total summary at top

---

## Phase 3: Connect & Test

### Step 1: Wire iOS to Backend
- Update APIClient with correct base URL
- Add auth headers handling

### Step 2: Seed Data
- Create sample categories
- Create sample budgets
- Add sample transactions

### Step 3: Build & Verify
- `cargo build` for backend
- `xcodebuild` for iOS
- Manual testing of full flow

---

## Dependencies

### Backend
- axum (HTTP framework)
- sqlx (Database)
- serde (Serialization)
- chrono (Date handling)
- uuid (IDs)

### iOS
- Swift Charts (Ring chart)
- Foundation (Networking)

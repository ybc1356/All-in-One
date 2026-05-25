# All-in-1: Financial Management App Architecture

## Overview

This document outlines the architecture for an all-in-one financial management application consisting of:
1. **Backend** (Rust) - API server with integrated ML classification service
2. **iOS Frontend** (Swift/SwiftUI) - Native budget management with LLM insights

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           iOS Client (SwiftUI)                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐│
│  │   Budgets   │  │ Transactions│  │  Insights   │  │   LLM Assistant    ││
│  │    View     │  │    List     │  │   (Charts)  │  │   (OpenAI/Anthropic)││
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘│
└─────────┼────────────────┼────────────────┼────────────────────┼───────────┘
          │                │                │                    │
          └────────────────┴────────────────┴────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Backend (Rust - API + ML)                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐  │
│  │  Auth       │  │  Budget     │  │ Transaction │  │  CSV Upload     │  │
│  │  Middleware │  │  Routes     │  │  Routes     │  │  Handler        │  │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └────────┬────────┘  │
│         │                │                │                  │           │
│         └────────────────┴────────────────┴──────────────────┘           │
│                                    │                                       │
│                          ┌─────────┴─────────┐                            │
│                          │   Service Layer   │                            │
│                          │  - Budget Service │                            │
│                          │  - Transaction    │                            │
│                          │    Service        │                            │
│                          └─────────┬─────────┘                            │
│                                    │                                       │
│  ┌─────────────────────────────────┴───────────────────────────────────┐  │
│  │                       ML Module (Burn)                              │  │
│  │  ┌────────────────────────────┐  ┌──────────────────────────────┐ │  │
│  │  │  CSV Parser                │  │  ML Classifier               │ │  │
│  │  │  - Israeli CC formats      │  │  - Category prediction      │ │  │
│  │  │  - Generic format          │  │  - Merchant identification  │ │  │
│  │  └────────────────────────────┘  └──────────────────────────────┘ │  │
│  └────────────────────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│           Database (PostgreSQL)                                             │
│   ┌─────────┐ ┌─────────┐ ┌─────────┐                                     │
│   │ Users   │ │Budgets  │ │Transactions│                                    │
│   └─────────┘ └─────────┘ └─────────┘                                     │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Project Structure

```
All-in-1/
├── backend/                       # Rust Backend (API + ML)
│   ├── src/
│   │   ├── main.rs               # Entry point, server setup
│   │   ├── lib.rs               # Library root
│   │   ├── ml/                   # ML Classification Module
│   │   │   ├── mod.rs           # ML module root
│   │   │   ├── parser.rs         # CSV parsing (Israeli CC formats)
│   │   │   ├── classifier.rs    # ML model inference
│   │   │   └── models.rs         # Transaction/Category structs
│   │   ├── api/
│   │   │   ├── mod.rs            # API module
│   │   │   ├── routes/
│   │   │   │   ├── mod.rs        # Routes module
│   │   │   │   ├── auth.rs       # Authentication routes
│   │   │   │   ├── budgets.rs    # Budget CRUD routes
│   │   │   │   ├── transactions.rs # Transaction routes
│   │   │   │   └── upload.rs     # CSV upload routes
│   │   │   └── middleware/
│   │   │       ├── mod.rs        # Middleware module
│   │   │       └── auth.rs       # JWT/auth middleware
│   │   ├── services/
│   │   │   ├── mod.rs            # Services module
│   │   │   ├── auth_service.rs   # Auth business logic
│   │   │   ├── budget_service.rs # Budget business logic
│   │   │   ├── transaction_service.rs # Transaction logic
│   │   │   └── llm_client.rs     # External LLM client
│   │   ├── models/
│   │   │   ├── mod.rs            # Models module
│   │   │   ├── user.rs           # User model
│   │   │   ├── budget.rs         # Budget model
│   │   │   └── transaction.rs    # Transaction model
│   │   ├── db/
│   │   │   ├── mod.rs            # Database module
│   │   │   ├── connection.rs     # DB connection
│   │   │   ├── migrations/       # SQL migrations
│   │   │   └── schema.rs         # Table definitions
│   │   └── config.rs             # Configuration
│   ├── Cargo.toml
│   └── README.md
│
├── ios/                          # iOS Swift Application
│   ├── AllIn1/
│   │   ├── App/
│   │   │   └── AllIn1App.swift   # App entry point
│   │   ├── Views/
│   │   │   ├── ContentView.swift # Main navigation
│   │   │   ├── Budgets/
│   │   │   │   ├── BudgetListView.swift
│   │   │   │   ├── BudgetDetailView.swift
│   │   │   │   └── CreateBudgetView.swift
│   │   │   ├── Transactions/
│   │   │   │   ├── TransactionListView.swift
│   │   │   │   └── TransactionRow.swift
│   │   │   ├── Insights/
│   │   │   │   └── InsightsView.swift
│   │   │   └── Chat/
│   │   │       └── LLMChatView.swift
│   │   ├── ViewModels/
│   │   │   ├── BudgetViewModel.swift
│   │   │   ├── TransactionViewModel.swift
│   │   │   └── ChatViewModel.swift
│   │   ├── Models/
│   │   │   ├── Budget.swift
│   │   │   ├── Transaction.swift
│   │   │   └── Category.swift
│   │   ├── Services/
│   │   │   ├── APIClient.swift   # Backend communication
│   │   │   ├── LLMService.swift  # LLM integration
│   │   │   └── AuthService.swift  # Authentication
│   │   └── Resources/
│   │       └── Assets.xcassets
│   ├── AllIn1.xcodeproj
│   └── README.md
│
└── README.md                     # Main README
```

---

## Backend (Rust - API + ML)

| File | Description |
|------|-------------|
| `main.rs` | Application entry point that initializes the server, loads configuration, and handles graceful shutdown |
| `lib.rs` | Library root that re-exports public modules and types for the crate |
| `ml/mod.rs` | ML module root |
| `ml/parser.rs` | CSV parsing for Israeli credit card formats (Cal4, Max, Isracard) and generic |
| `ml/classifier.rs` | ML model inference for transaction categorization and merchant identification |
| `ml/models.rs` | Transaction and Category data structures |
| `api/routes/auth.rs` | OAuth callback handlers for Google and Apple Sign-In, JWT token generation/refresh, user profile retrieval, logout |
| `api/routes/budgets.rs` | CRUD operations for budget management including create, read, update, and delete |
| `api/routes/transactions.rs` | Transaction listing, searching, and retrieval endpoints |
| `api/routes/upload.rs` | CSV file upload handler that processes data through ML classification |
| `api/middleware/auth.rs` | JWT token validation middleware that protects authenticated routes |
| `services/auth_service.rs` | Authentication business logic for OAuth token exchange, user lookup and creation |
| `services/budget_service.rs` | Business logic for budget operations including creation, updates, and spending calculations |
| `services/transaction_service.rs` | Business logic for transaction CRUD operations and filtering |
| `services/llm_client.rs` | HTTP client for calling external LLM APIs (OpenAI/Anthropic) for insights |
| `models/user.rs` | User data model with id, tz (Israeli ID), email, name, OAuth provider fields, and timestamps |
| `models/budget.rs` | Budget data model with amount limits and category associations |
| `models/transaction.rs` | Transaction data model with categorized spending information |
| `db/connection.rs` | Database connection pool management for PostgreSQL |
| `db/migrations/` | SQL migration files for creating and updating database schema |
| `db/schema.rs` | Table definitions and schema structure for the database |
| `config.rs` | Environment variable loading and application configuration management |

---

## iOS Frontend (Swift/SwiftUI)

| File | Description |
|------|-------------|
| `App/AllIn1App.swift` | SwiftUI application entry point that initializes the app and view hierarchy |
| `Views/ContentView.swift` | Main navigation container with tab-based routing between app sections |
| `Views/Budgets/BudgetListView.swift` | Displays a list of all user budgets with summary information |
| `Views/Budgets/BudgetDetailView.swift` | Shows detailed budget information including spending progress |
| `Views/Budgets/CreateBudgetView.swift` | Form for creating a new budget with category and limit selection |
| `Views/Transactions/TransactionListView.swift` | Scrollable list of all transactions with filtering options |
| `Views/Transactions/TransactionRow.swift` | Individual transaction display component with category and amount |
| `Views/Insights/InsightsView.swift` | Charts and analytics showing spending patterns and trends |
| `Views/Chat/LLMChatView.swift` | Chat interface for the AI-powered financial assistant |
| `ViewModels/BudgetViewModel.swift` | State management for budget-related operations and data |
| `ViewModels/TransactionViewModel.swift` | State management for transaction loading and filtering |
| `ViewModels/ChatViewModel.swift` | State management for LLM chat conversation handling |
| `Models/Budget.swift` | Budget data model conforming to Codable protocol |
| `Models/Transaction.swift` | Transaction data model conforming to Codable protocol |
| `Models/Category.swift` | Category data model for transaction classification |
| `Services/APIClient.swift` | Network client for communicating with the Rust backend API |
| `Services/LLMService.swift` | Service for handling LLM API calls and responses |
| `Services/AuthService.swift` | Service for managing authentication tokens and user sessions |

---

## Data Flow

**CSV Upload Flow:**
```
iOS → Rust Upload → ML Classifier → Category Prediction 
     → Return categorized transactions → Save to DB → Return to iOS
```

**Budget Creation Flow:**
```
iOS → Rust API → Validate → Save to DB → Return confirmation
```

**LLM Insights Flow:**
```
iOS → Rust API → Gather transaction data → Send to LLM → Return insights
```

---

## Technology Stack

| Layer | Technology | Reason |
|-------|------------|--------|
| Backend (API + ML) | Rust | Unified codebase, performance, safety |
| ML Framework | Burn | Rust training + inference via Burn (tch backend) |
| Database | PostgreSQL | Reliable, proven in production |
| iOS | Swift + SwiftUI | Native iOS, modern UI framework |
| LLM | External API (OpenAI/Anthropic) | User preference |

---

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/auth/google` | POST | Google OAuth callback |
| `/auth/apple` | POST | Apple OAuth callback |
| `/auth/refresh` | POST | Refresh JWT token |
| `/auth/me` | GET | Get current user profile |
| `/auth/logout` | POST | Invalidate token |
| `/budgets` | GET | List budgets |
| `/budgets` | POST | Create budget |
| `/budgets/{id}` | PUT | Update budget |
| `/budgets/{id}` | DELETE | Delete budget |
| `/transactions` | GET | List transactions |
| `/transactions/upload` | POST | Upload CSV for classification |
| `/insights` | POST | Get AI-powered insights via LLM |

---

## Future Enhancements

- [ ] Multi-currency support
- [ ] Investment portfolio tracking
- [ ] Bill reminders
- [ ] Export to PDF/Excel
- [ ] On-device ML for privacy
- [ ] Apple Watch companion app

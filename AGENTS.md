# AGENTS.md - All-in-1 Financial Management App

This document provides guidelines for agents working in this repository.

## Project Overview

Multi-component financial management application:
- **Backend**: Rust - API server with integrated ML classification service
- **iOS Frontend**: Swift/SwiftUI - Native budget management with LLM insights

## Project Structure

```
All-in-1/
├── backend/              # Rust Backend (API + ML)
│   └── src/
│       ├── ml/           # ML classification module (Burn)
│       ├── api/          # HTTP routes
│       ├── services/     # Business logic
│       ├── models/       # Data models
│       └── db/           # Database layer
└── ios/                  # iOS Swift Application (SwiftUI)
```

---

## Agent Behavior Rules

- **Keep changes small and modular**: Make focused, incremental changes. Avoid large refactors or sweeping modifications in a single task.
- **Stay within scope**: Only address the specific request. Do not fix unrelated issues, refactor neighboring code, or add unplanned features.
- **Ask before going out of scope**: If additional changes are needed (bug fixes, improvements, related issues), stop and ask for permission before proceeding.
- **One task at a time**: Complete the current task before moving to related work, even if you spot other issues.
- **Minimal edits**: When editing, make the smallest change necessary to achieve the goal.

---

## Build/Lint/Test Commands

### Rust Backend

```bash
# Build
cargo build                    # Debug build
cargo build --release         # Production build

# Run
cargo run                     # Start server
cargo run -- --config .env    # With config

# Testing
cargo test                    # Run all tests
cargo test <TEST_NAME>        # Run specific test
cargo test -- --test-threads=1  # Run tests sequentially

# Linting & Formatting
cargo fmt                     # Format code
cargo fmt --check             # Check formatting (CI)
cargo clippy                  # Lint with clippy
cargo clippy -- -D warnings   # Strict linting

# Dependencies
cargo check                   # Check without full build
cargo update                  # Update dependencies
```

### iOS Application

```bash
# Building (Xcode)
xcodebuild -scheme AllIn1 -configuration Debug build
xcodebuild -scheme AllIn1 -configuration Release build

# Testing
xcodebuild test -scheme AllIn1 -destination 'platform=iOS Simulator,name=iPhone 15'
xcodebuild test -scheme AllIn1 -only-testing:AllIn1Tests/<TEST_CLASS>/<TEST_METHOD>

# Linting
swiftlint                                # Lint project
swiftlint --fix                          # Auto-fix issues
```

---

## Code Style Guidelines

### Rust Backend

**Formatting & Style**
- Use `cargo fmt` with default settings
- 4-space indentation
- Maximum line length: 100 characters
- No semicolons after expressions (let bindings)
- Use `?` operator for error propagation

**Naming Conventions**
- Snake_case for variables, functions, modules: `get_user_by_id`
- CamelCase for types and traits: `UserService`, `TransactionModel`
- SCREAMING_SNAKE_CASE for constants: `MAX_RETRY_COUNT`
- Prefix enums variants without prefix: `Status::Active`, not `Status::EActive`

**Imports**
```rust
use std::collections::HashMap;
use crate::api::routes::auth;
use crate::models::{User, Budget};
```

**Error Handling**
- Use `Result<T, Error>` for fallible operations
- Use `anyhow::Result<T>` for application code (rich context)
- Use `thiserror` for library code (typed errors)
- Never use `unwrap()` in production code; use `?` or `.context()`
- Wrap errors with context: `some_func().context("Failed to load user")?`

**Structs & Traits**
- Use `#[derive(Debug, Clone, Serialize, Deserialize)]` for DTOs
- Use builder pattern for complex construction
- Group fields logically, not alphabetically

**Async**
- Use `async fn` for handlers
- Use `tokio` runtime
- Clone large data before `.await` to avoid borrow issues

---

### Swift/iOS

**Formatting**
- Use SwiftFormat or conform to Swift API Design Guidelines
- 4-space indentation
- Maximum line length: 120 characters
- Use trailing commas in multi-line collections

**Naming**
- PascalCase for types, protocols, enums: `TransactionRow`, `BudgetService`
- camelCase for functions, variables: `fetchTransactions`, `isLoading`
- Lowercase for protocols: `Identifiable`, `Codable`
- Prefix delegates: `TransactionListViewDelegate`
- Use descriptive names; avoid abbreviations except `id`, `url`, `api`

**Types**
- Use structs for data models (Swift is value-semantic by default)
- Use classes sparingly (ViewModels, delegates, shared mutable state)
- Always specify types for properties: `var items: [Transaction] = []`
- Prefer protocols over concrete types: `some: Collection` not `some: Array`

**SwiftUI**
- Keep Views thin; push logic to ViewModels
- Use `@State`, `@Binding`, `@StateObject`, `@EnvironmentObject` appropriately
- Prefer `let` over `var` for computed properties
- Use `some View` for function return types

**Error Handling**
- Use `Result<T, Error>` for synchronous operations
- Use `async/await` with `throws` for asynchronous operations
- Never force unwrap (`!`); use `guard let` or `if let`
- Provide user-friendly error messages

**Imports**
```swift
import Foundation
import SwiftUI
import Combine
```

---

## Architecture Patterns

### Rust Backend (Layered Architecture)
```
api/routes/     → HTTP handlers (thin, delegate to services)
services/       → Business logic
models/         → Domain models & DTOs
ml/             → ML classification (parser, classifier, models)
db/             → Database access layer
middleware/     → Auth, logging, CORS
```

### iOS (MVVM)
```
Views/          → SwiftUI views (presentation)
ViewModels/     → ObservableObject classes
Models/         → Data models (Codable)
Services/       → API clients, business logic
```

---

## Current Implementation Plan

**Reference:** `plan.md` - Contains the active implementation roadmap.

Before starting any feature work, agents should:
1. Read `plan.md` to understand current priorities
2. Check if the feature is already planned or in progress
3. Follow the phases/steps outlined in the plan
4. Update `plan.md` when completing a step

---

## Git Conventions

- Branch naming: `feat/<feature>`, `fix/<issue>`, `chore/<task>`
- Commit format: `<type>(<scope>): <description>`
  - Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
- PR requirements: Tests pass, clippy clean, fmt checked

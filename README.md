# RustAPI 🦀

> **FastAPI-inspired REST framework for Rust**

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

RustAPI brings the developer experience of FastAPI and NestJS to Rust, combining:

- 🎯 **Route Macros** - FastAPI-style endpoint definitions
- 💉 **Dependency Injection** - Type-safe service container
- ⚡ **Performance** - Built on Axum + Tokio
- 🔒 **Type Safety** - Leverage Rust's type system
- 📝 **Future: Auto OpenAPI** - Documentation that stays in sync (coming soon)

**Status**: 🚧 Active Development | Not yet production-ready

## Quick Start

```rust
use rustapi::prelude::*;

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
}

#[get("/")]
async fn hello() -> &'static str {
    "Hello, rustapi!"
}

#[get("/users/{id}")]
async fn get_user(Path(id): Path<String>) -> Json<User> {
    Json(User {
        id: id.clone(),
        name: format!("User {}", id)
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(__hello_route.0, __hello_route.1())
        .route(__get_user_route.0, __get_user_route.1());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

## Features

### ✅ Implemented

- **Route Macros**: `#[get]`, `#[post]`, `#[put]`, `#[delete]`, `#[patch]`
- **DI Container**: Type-safe service registration and resolution
- **Prelude Module**: One import for everything you need
- **Examples**: Working hello_world and full-featured examples

### 🚧 Coming Soon

- **`Inject<T>` Extractor**: Automatic dependency injection in handlers
- **Validation**: `#[derive(Validate)]` with automatic error responses
- **OpenAPI Generation**: Auto-generated Swagger docs
- **Request-Scoped Services**: Per-request service instances
- **Testing Utilities**: Easy integration testing

## Examples

Run the examples to see the framework in action:

```bash
# Minimal hello world
cargo run --example hello_world

# Full-featured example
cargo run --example with_macros

# Demo app with DI
cargo run
```

Then test the endpoints:

```bash
curl http://localhost:3000/
curl http://localhost:3000/users/42
```

## Architecture

```
rustapi/
├── crates/
│   ├── rustapi-core/      # DI container, app builder
│   ├── rustapi-macros/    # Route macros (#[get], etc.)
│   └── rustapi/           # Facade crate (main export)
├── examples/
│   ├── hello_world.rs         # Minimal example
│   └── with_macros.rs         # Full-featured example
└── src/
    ├── lib.rs                 # Framework facade
    └── main.rs                # Demo application
```

## Comparison

| Feature         | rustapi | axum | actix-web | poem | rocket |
| --------------- | ------- | ---- | --------- | ---- | ------ |
| Route Macros    | ✅      | ❌   | ❌        | ❌   | ✅     |
| Built-in DI     | ✅      | ❌   | ✅        | ❌   | ❌     |
| Auto OpenAPI    | 🚧      | ❌   | ❌        | ✅   | ❌     |
| FastAPI-like DX | ✅      | ❌   | ❌        | ~    | ~      |
| Performance     | ⚡      | ⚡   | ⚡        | ⚡   | ⚡     |

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) - Complete architectural vision
- [PROGRESS.md](PROGRESS.md) - Development progress
- [TODO.md](TODO.md) - Detailed roadmap
- [examples/](examples/) - Working code examples

## Roadmap

**Phase 1: Core** ✅

- [x] DI Container
- [x] Route Macros
- [x] Examples

**Phase 2: DX Improvements** 🚧

- [ ] `Inject<T>` extractor
- [ ] Better route registration
- [ ] Macro-generated app builder

**Phase 3: Validation** 📋

- [ ] `#[derive(Validate)]`
- [ ] Automatic validation
- [ ] Structured error responses

**Phase 4: OpenAPI** 📋

- [ ] Schema generation
- [ ] Swagger UI
- [ ] ReDoc support

## Why RustAPI?

**Python/FastAPI developers** get Rust performance with familiar patterns.

**TypeScript/NestJS developers** get dependency injection in Rust.

**Rust developers** get FastAPI-level developer experience.

## Contributing

This is currently in active development. Contributions welcome!

## License

This project is licensed under either of:

- Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (http://opensource.org/licenses/MIT)

at your option.

## Inspiration

- **FastAPI** (Python) - Amazing DX, automatic docs
- **NestJS** (TypeScript) - Dependency injection, modules
- **Axum** (Rust) - Performance, type safety

---

Built with ❤️ using Rust, Axum, and Tokio.

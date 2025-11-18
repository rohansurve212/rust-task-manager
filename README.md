# 🚀 Rust Task Management Application

A modern, high-performance task management application built with Rust, demonstrating the power of compile-time safety, zero-cost abstractions, and true async parallelism.

## 📋 Overview

This project is a learning-oriented implementation of a full-stack task management system using cutting-edge Rust technologies:

- **gRPC** - Efficient, type-safe backend communication
- **HTMX** - Reactive frontend without JavaScript frameworks
- **Warp** - Fast, composable web framework
- **SQLite** - Lightweight, embedded database
- **Tokio** - High-performance async runtime
- **Cargo Workspace** - Multi-crate project organization

## 🎯 Learning Goals

This project demonstrates Rust's advantages over Python:

### Type Safety & Compile-Time Guarantees
- Invalid states are unrepresentable
- API changes caught at compile time, not in production
- No null pointer exceptions (Option<T> makes nullability explicit)
- Protocol Buffer changes verified at build time

### Performance
- 10-100x faster than equivalent Python code
- True async parallelism (no GIL)
- Zero-cost abstractions
- Predictable memory usage

### Developer Experience
- Comprehensive error messages
- Built-in testing framework
- Auto-generated documentation
- Excellent tooling (cargo, clippy, rustfmt)

## 🏗️ Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                     Cargo Workspace                         │
│                                                             │
│  ┌────────────────┐  ┌────────────────┐  ┌──────────────┐   │
│  │  grpc-service  │  │  web-service   │  │    shared    │   │
│  │                │  │                │  │              │   │
│  │  - Business    │  │  - HTTP Server │  │  - Proto     │   │
│  │    Logic       │  │  - HTMX UI     │  │  - Models    │   │
│  │  - gRPC Server │  │  - gRPC Client │  │  - Types     │   │
│  │  - Database    │  │  - Templates   │  │  - Errors    │   │
│  │                │  │                │  │              │   │
│  │  Port: 50051   │  │  Port: 3000    │  │  (Library)   │   │
│  └────────────────┘  └────────────────┘  └──────────────┘   │
│         │                     │                   ▲         │
│         │                     │                   │         │
│         └─────────────────────┴───────────────────┘         │
│                     Shared Types & Proto                    │
└─────────────────────────────────────────────────────────────┘
```

### Service Communication
```
User Browser → [HTTP/HTMX] → Web Service → [gRPC]  → gRPC Service → SQLite
             ← [HTML]      ←             ← [Proto] ←              ←
```

## 📦 Project Structure
```
rust-task-manager/
├── Cargo.toml              # Workspace configuration
├── Makefile.toml           # Build automation (cargo-make)
├── README.md               # This file
├── .gitignore             # Git ignore patterns
│
├── grpc-service/          # Backend service (Port 50051)
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   └── build.rs           # Proto code generation (Phase 2)
│
├── web-service/           # Frontend service (Port 3000)
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   ├── templates/         # Askama templates (Phase 3)
│   └── static/            # CSS, images (Phase 3)
│
└── shared/                # Shared library
    ├── Cargo.toml
    ├── src/
    │   └── lib.rs
    ├── proto/             # Protocol Buffer definitions (Phase 2)
    └── migrations/        # Database migrations (Phase 1)
```

## 🚀 Getting Started

### Prerequisites

1. **Rust** (latest stable)
```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Protocol Buffers Compiler**
   - macOS: `brew install protobuf`
   - Linux: `sudo apt install protobuf-compiler`
   - Windows: `choco install protoc`

3. **Cargo-make** (build automation)
```bash
   cargo install cargo-make
```

4. **SQLx CLI** (database migrations)
```bash
   cargo install sqlx-cli --no-default-features --features sqlite
```

### Quick Start
```bash
# Clone the repository
git clone <your-repo-url>
cd rust-task-manager

# Install dependencies and tools
cargo make setup

# Build all services (debug mode)
cargo make build

# Run all services
cargo make dev

# Or run services individually
cargo make grpc    # Run gRPC service only
cargo make web     # Run web service only
```

## 🛠️ Available Commands

Run `cargo make` or `cargo make help` to see all available commands.

### Development
```bash
cargo make build          # Build all services (debug)
cargo make build-release  # Build with optimizations
cargo make dev            # Run all services
cargo make clean          # Remove build artifacts
```

### Code Quality
```bash
cargo make check          # Type-check without building
cargo make test           # Run all tests
cargo make fmt            # Format code
cargo make lint           # Run clippy linter
cargo make doc            # Generate and open documentation
```

### Database
```bash
cargo make db-setup       # Initialize database
cargo make db-migrate     # Run migrations
cargo make db-reset       # Reset database (destructive!)
```

### Utilities
```bash
cargo make watch          # Watch for changes and rebuild
cargo make tree           # Show dependency tree
cargo make outdated       # Check for outdated dependencies
cargo make audit          # Security vulnerability check
```

## 📚 Implementation Phases

This project is built in phases, each introducing new Rust concepts:

### ✅ Phase 0: Project Foundation & Setup (Current)
- [x] Cargo workspace configuration
- [x] Build automation with cargo-make
- [x] Project structure and placeholders
- [x] Basic logging and async runtime setup

**Key Rust Concepts**: Cargo workspaces, async/await, macros, module system

### 🔄 Phase 1: Database Layer (Next)
- [ ] SQLite integration with sqlx
- [ ] Database models and migrations
- [ ] Repository pattern implementation
- [ ] Error handling with Result<T, E>

**Key Rust Concepts**: Ownership, borrowing, lifetimes, traits, error handling

### 🔄 Phase 2: gRPC Service
- [ ] Protocol Buffer definitions
- [ ] Code generation from .proto files
- [ ] gRPC server implementation
- [ ] Async request handling

**Key Rust Concepts**: Async/await deep dive, code generation, type safety

### 🔄 Phase 3: Web Service & HTMX
- [ ] Warp web server
- [ ] Askama template engine
- [ ] HTMX integration
- [ ] gRPC client implementation

**Key Rust Concepts**: Filter composition, compile-time templates, closures

### 🔄 Phase 4: Domain Models & Validation
- [ ] Type-driven design
- [ ] Newtype pattern
- [ ] Builder pattern
- [ ] Business logic

**Key Rust Concepts**: Type-state pattern, smart constructors, zero-cost abstractions

### 🔄 Phase 5: Authentication & Authorization
- [ ] User registration and login
- [ ] Password hashing
- [ ] Session management
- [ ] Authorization middleware

**Key Rust Concepts**: Security, middleware patterns, type safety for auth

### 🔄 Phase 6: Advanced Features
- [ ] Filtering and search
- [ ] Sorting and pagination
- [ ] Performance optimization
- [ ] Benchmarking

**Key Rust Concepts**: Iterators, closures, performance profiling

### 🔄 Phase 7: Testing
- [ ] Unit tests
- [ ] Integration tests
- [ ] Property-based testing
- [ ] Error recovery

**Key Rust Concepts**: Testing framework, mocking, panic handling

### 🔄 Phase 8: Deployment
- [ ] Configuration management
- [ ] Docker containers
- [ ] Logging and metrics
- [ ] Production readiness

**Key Rust Concepts**: Release builds, cross-compilation, observability

### 🔄 Phase 9: Polish
- [ ] Documentation
- [ ] Code quality
- [ ] CI/CD pipeline
- [ ] Performance benchmarks

**Key Rust Concepts**: Documentation tests, tooling, best practices

## 🔧 Development Workflow

### Adding a New Feature

1. **Write the code**
```bash
   # Edit files in your editor
```

2. **Check for errors**
```bash
   cargo make check
```

3. **Format code**
```bash
   cargo make fmt
```

4. **Run linter**
```bash
   cargo make lint
```

5. **Run tests**
```bash
   cargo make test
```

6. **Commit changes**
```bash
   git add .
   git commit -m "feat: add new feature"
```

### Running in Development
```bash
# Terminal 1: Run gRPC service
cargo make grpc

# Terminal 2: Run web service
cargo make web

# Or run both together (when implemented in Phase 2-3)
cargo make dev
```

## 📊 Performance Expectations

Based on similar Rust applications:

| Metric | Python (Flask/FastAPI) | Rust (Warp/Tokio) |
|--------|----------------------|-------------------|
| Requests/sec | ~1,000 | ~100,000 |
| Memory Usage | ~50-100 MB | ~5-10 MB |
| Startup Time | ~1 second | ~10 milliseconds |
| Binary Size | ~50 MB + interpreter | ~5-10 MB standalone |
| CPU Usage | High (GIL bottleneck) | Low (true parallelism) |

## 🎓 Learning Resources

### Rust Fundamentals
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

### Async Rust
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

### Web Development
- [Warp Documentation](https://docs.rs/warp/)
- [Askama Documentation](https://docs.rs/askama/)

### gRPC
- [Tonic Documentation](https://docs.rs/tonic/)
- [Protocol Buffers Guide](https://protobuf.dev/)

## 🐛 Troubleshooting

### Build Errors

**Problem**: Compile errors about missing dependencies
```bash
# Solution: Update dependencies
cargo update
cargo make build
```

**Problem**: protoc not found
```bash
# Solution: Install Protocol Buffers compiler
# macOS
brew install protobuf

# Linux
sudo apt install protobuf-compiler

# Windows
choco install protoc
```

### Runtime Errors

**Problem**: Port already in use
```bash
# Solution: Kill process using the port
# macOS/Linux
lsof -ti:3000 | xargs kill -9
lsof -ti:50051 | xargs kill -9

# Or change port in config (Phase 1)
```

**Problem**: Database locked
```bash
# Solution: Close all connections and restart
cargo make db-reset
```

## 🤝 Contributing

This is a learning project! Contributions, suggestions, and improvements are welcome.

### Code Style

- Run `cargo make fmt` before committing
- Ensure `cargo make lint` passes
- Write tests for new features
- Add documentation comments for public items

## 📝 License

MIT License - See LICENSE file for details

## 🙏 Acknowledgments

- Rust community for excellent documentation
- Tokio team for the async runtime
- All open-source contributors to the dependencies used

## 📞 Support

For questions or issues:
1. Check the documentation: `cargo make doc`
2. Review implementation phases above
3. Open an issue on GitHub

---

**Current Status**: Phase 0 Complete ✅

**Next Steps**: Proceed to Phase 1 - Database Layer

Happy Coding! 🦀
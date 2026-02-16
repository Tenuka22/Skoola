# Rust Code Style Guidelines

These guidelines promote consistent, readable, and maintainable Rust code within the project.

## Formatting

*   **`rustfmt`:** Always use `rustfmt` with default settings.
*   **Line Length:** Aim for a maximum of 100 characters per line.
*   **Indentation:** Use 4 spaces for indentation.

## Naming Conventions

*   **`snake_case`:** Use `snake_case` for function names, variable names, and module names.
*   **`PascalCase`:** Use `PascalCase` for type names (structs, enums, traits).
*   **`SCREAMING_SNAKE_CASE`:** Use `SCREAMING_SNAKE_CASE` for constants.

## Comments and Documentation

*   **Doc Comments:** Use `///` for documentation comments on public items (functions, structs, enums, traits, modules).
*   **Inline Comments:** Use `//` for inline comments, explaining *why* something is done, not *what* it does.
*   **Clarity:** Comments should be clear, concise, and up-to-date.

## Error Handling

*   **`Result` and `Option`:** Prefer `Result<T, E>` for recoverable errors and `Option<T>` for potentially absent values.
*   **`?` Operator:** Utilize the `?` operator for concise error propagation.
*   **Custom Error Types:** Define custom error types for domain-specific errors.
*   **`anyhow` / `thiserror`:** Consider using error handling crates like `anyhow` for application-level errors and `thiserror` for library-level errors to simplify error management.

## Concurrency

*   **Standard Library:** Prefer standard library concurrency primitives (e.g., `std::sync`, `std::thread`) when appropriate.
*   **`tokio` / `async-std`:** For asynchronous programming, consistently use either `tokio` or `async-std` throughout the project.
*   **`Arc` and `Mutex`:** Use `Arc` for shared ownership and `Mutex` for mutable shared state, being mindful of potential deadlocks.

## Testing

*   **Unit Tests:** Place unit tests in the same file as the code they test, within a `#[cfg(test)] mod tests { ... }` block.
*   **Integration Tests:** Place integration tests in the `tests/` directory at the crate root.
*   **Doc Tests:** Use documentation comments as tests (`/// ````).

## Linting and Static Analysis

*   **`clippy`:** Run `clippy` regularly and address warnings.
*   **`cargo check`:** Ensure `cargo check` passes without warnings or errors.

## Dependencies

*   **Minimal:** Keep dependencies to a minimum.
*   **Audited:** Prefer well-maintained and audited crates.
*   **Version Pinning:** Use semantic versioning and consider pinning dependencies for stability.

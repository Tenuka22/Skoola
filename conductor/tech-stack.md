# Tech Stack: Skoola

This document outlines the core technologies and architectural patterns employed in the Skoola project.

## Backend

*   **Language:** Rust
    *   **Rationale:** Chosen for its performance, memory safety, and concurrency, making it ideal for building a robust and scalable backend API.
*   **ORM (Object-Relational Mapper):** Diesel
    *   **Rationale:** Provides a safe, type-checked way to interact with the database in Rust, ensuring data integrity and developer productivity.
*   **Database:** SQLite
    *   **Rationale:** Selected for its serverless, embedded nature, suitable for initial development and potentially for smaller deployments or local development. It offers ease of setup and management.
*   **Architecture:** Modular, function-based API with Handlers, Services, and Models
    *   **Rationale:** Promotes clear separation of concerns, modularity, and idiomatic Rust patterns by favoring standalone functions over class/struct-based service implementations.
        *   **Models:** Define the data structures and database schema.
        *   **Services:** Organize business logic into modular functions that interact with repositories/models.
        *   **Handlers:** Manage incoming requests, call services, and return responses.

## Frontend

*   **Language:** TypeScript / JavaScript
    *   **Rationale:** TypeScript enhances code quality, maintainability, and developer experience through static typing, crucial for a complex application.
*   **Framework:** React
    *   **Rationale:** A popular and powerful library for building interactive user interfaces, known for its component-based architecture and extensive ecosystem.
    *   **Routing:** Tanstack Router - Provides a robust and type-safe routing solution for single-page applications.
    *   **State Management / Data Fetching:** Tanstack Query (React Query) - Manages server state, caching, and data synchronization, simplifying complex data fetching patterns.
*   **Package Manager:** Bun
    *   **Rationale:** Selected for its speed and efficiency in package management and script execution, optimizing the frontend development workflow.
*   **UI Library:** Custom Component Library (Likely based on Shadcn UI)
    *   **Rationale:** Ensures a consistent, modern, and accessible user interface across the application, accelerating UI development.
*   **API Client Generation:** OpenAPI/Swagger Code Generation
    *   **Rationale:** Automates the creation of a type-safe API client from an OpenAPI specification, reducing manual errors and keeping frontend and backend APIs synchronized.
*   **ORM (for Schema Definition):** Drizzle ORM
    *   **Rationale:** Used for defining database schemas, potentially sharing schema definitions between frontend (e.g., for validation) and backend, ensuring consistency.

## Overall Architecture

*   **Type:** Monorepo
    *   **Rationale:** Facilitates code sharing, consistent tooling, and streamlined development across both frontend and backend projects within a single repository.
*   **Structure:** Client-Server Architecture
    *   **Rationale:** The backend (Rust API) serves data and business logic, while the frontend (React/TypeScript application) consumes these APIs to provide the user interface, allowing for independent scaling and development of each component.

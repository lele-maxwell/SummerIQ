# SummerIQ

SummerIQ is a project aimed at providing intelligent analysis and insights.

## Project Structure

The project consists of two main parts:

1.  **Frontend**: Built with React, TypeScript, Vite, shadcn-ui, and Tailwind CSS.
2.  **Backend**: A Rust application using axum, sqlx, and PostgreSQL for data storage.

## Setup and Running Locally

To set up and run SummerIQ locally, you need to have the following installed:

- Node.js & npm (or bun)
- Rust and Cargo
- PostgreSQL
- sqlx-cli

### Backend Setup

1.  **Install Rust and Cargo**: Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
2.  **Install sqlx-cli**: `cargo install sqlx-cli --no-default-features --features native-tls,postgres`
3.  **Set up PostgreSQL**: Ensure you have a PostgreSQL server running and a database created for SummerIQ. Make note of the database connection URL.
4.  **Set Environment Variables**: Create a `.env` file in the `summeriq-backend` directory with your database connection URL. For example:

    ```dotenv
    DATABASE_URL=postgres://user:password@host:port/database_name
    ```

5.  **Run Migrations**: Navigate to the `summeriq-backend` directory and run the migrations.

    ```bash
    cd summeriq-backend
    sqlx migrate run
    ```

6.  **Run the Backend**: Stay in the `summeriq-backend` directory and run the backend server.

    ```bash
    cargo run
    ```

### Frontend Setup

1.  **Navigate to the frontend directory**: `cd ..` (if you are in `summeriq-backend`) or ensure you are in the project root.
2.  **Install dependencies**: `npm install` (or `bun install`)
3.  **Run the Frontend**: Start the development server.

    ```bash
    npm run dev
    ```

    The frontend should be available at `http://localhost:8080/` or a similar port.

## How can I contribute?

...

## Technologies Used

**Frontend**:

- React
- TypeScript
- Vite
- shadcn-ui
- Tailwind CSS

**Backend**:

- Rust
- axum
- sqlx
- PostgreSQL

# SOLUX | Premium Smart NFC Card System

SOLUX is a high-performance, digital-first networking ecosystem built with **Rust**. It simulates the experience of a smart NFC business card (like Avici Card), allowing users to share their professional identity through a single "tap" or scan.



## Overview

The SOLUX system consists of a robust backend server that handles "tap" interactions, a dynamic profile rendering engine, and a premium web interface. While the hardware component involves NFC chips, this simulation focuses on the software architecture required to serve digital profiles with speed and security.

## Key Features

* **Lightning Fast Profile Serving:** Built on the Axum framework for sub-millisecond response times.
* **Dynamic QR Code Generation:** Automatically generates a terminal-based QR code for the simulation URL upon startup.
* **Premium UI/UX:** A sleek, dark-mode digital landing page designed for professional impact.
* **Modular Architecture:** Separated logic for storage, handling, and utilities to ensure scalability.
* **CI/CD Ready:** Includes GitHub Actions workflows for automated testing and building.

## Tech Stack

- **Language:** [Rust](https://www.rust-lang.org/) (Edition 2021)
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
- **Async Runtime:** [Tokio](https://tokio.rs/)
- **Data Handling:** [Serde](https://serde.rs/)
- **Utilities:** `qrcode` & `image` for visual identification.

## 📂 Project Structure

```text
solux/
├── .github/workflows/    # Automated CI/CD (rust.yml)
├── src/                  # Core Application Logic
│   ├── main.rs           # Entry point & Server config
│   ├── models.rs         # Data structures (Profiles/Users)
│   ├── handler.rs        # HTTP Request processing
│   ├── storage.rs        # Mock database layer
│   └── utils.rs          # Helper functions (QR Generation)
├── static/               # Frontend Assets
│   ├── index.html        # Base profile template
│   └── style.css         # Premium SOLUX styling
├── tests/                # Integration & Unit Tests
├── .gitignore            # Git exclusion rules
├── Cargo.toml            # Dependency & Project Manifest
└── .env                  # Environment Variables


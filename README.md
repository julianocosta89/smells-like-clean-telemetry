# Smells Like Clean Telemetry

This repository contains practical examples demonstrating different OpenTelemetry instrumentation approaches across multiple programming languages. The code was used during the talk "Smells Like Clean Telemetry" and showcases clean telemetry practices for modern applications.

Feel free to use, share, and send improvements/fixes/suggestions.

## Overview

This project implements the same Music service using different OpenTelemetry instrumentation patterns:

- **Auto-instrumentation**
  - **(Java/Spring Boot)** - Zero-code instrumentation using JVM agents
  - **(Node.js/Express)** - Automatic instrumentation for Node.js
- **Instrumentation Library (Rust/Actix)** - Using language-specific OpenTelemetry libraries
- **Manual Instrumentation**
  - **(Node.js/Express)** - Hand-crafted spans with semantic conventions
  - **(Python/Flask)** - Manual instrumentation with logging and traces
- **OpenTelemetry Weaver** - Code generation and semantic convention validation

Each service provides a REST API for retrieving music data, demonstrating how telemetry data flows through different instrumentation approaches.

## Architecture

```txt
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Client App    │───▶│      Music       │───▶│   PostgreSQL    │
│                 │    │     Service      │    │    Database     │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌──────────────────┐    ┌─────────────────┐
                       │  OpenTelemetry   │───▶│     Datadog     │
                       │    Collector     │    │  and/or Jaeger  │
                       └──────────────────┘    └─────────────────┘
```

All services expose the same API:

- `GET /songs/:title/:artist` - Get song metadata for a title/artist pair
- Returns cached data from PostgreSQL or fetches from MusicBrainz API

## Prerequisites

- Docker and Docker Compose
- Configured environment variables (see `.env` file for observability backend setup)

## Quick Start

1. **Clone the repository**

   ```bash
   git clone git@github.com:julianocosta89/smells-like-clean-telemetry.git
   cd smells-like-clean-telemetry
   ```

2. **Configure the observability backend**

    All services send data to the OpenTelemetry Collector, which can forward to different backends.

    Edit the [`.env`](.env) file to configure your observability backend:

    - Set `OBSERVABILITY_BACKEND=jaeger` for Jaeger only
    - Set `OBSERVABILITY_BACKEND=datadog` for both Datadog and Jaeger (requires API key)

    **Important:** Do not commit your Datadog credentials to version control.

3. **Choose and run a service** (see sections below)

## Service Examples

### Auto-instrumentation (Java/Spring Boot)

Demonstrates zero-code instrumentation using OpenTelemetry Java agent.

**Technology Stack:**

- Java 21 + Spring Boot 3.5.3
- PostgreSQL with JPA
- OpenTelemetry Java Agent

**Run the service:**

```shell
docker compose --profile auto build
docker compose --profile auto up
```

**Access:** <http://localhost:8080/songs/smells%20like%20teen%20spirit/nirvana>

### Instrumentation Library (Rust/Actix)

Shows how to integrate OpenTelemetry using instrumentation libraries.

**Technology Stack:**

- Rust + Actix Web
- Tokio PostgreSQL
- OpenTelemetry Actix Web instrumentation

**Run the service:**

```shell
docker compose --profile instrumentation-lib build
docker compose --profile instrumentation-lib up
```

**Access:** <http://localhost:8081/songs/smells%20like%20teen%20spirit/nirvana>

### Manual Instrumentation (Node.js/Express)

Demonstrates hand-crafted instrumentation with full control over telemetry data.

**Technology Stack:**

- Node.js + Express
- PostgreSQL with `pg` driver
- OpenTelemetry Node.js SDK + API

**Run the service:**

```shell
docker compose --profile manual build
docker compose --profile manual up
```

**Access:** <http://localhost:3000/songs/smells%20like%20teen%20spirit/nirvana>

### Express Auto-instrumentation (Node.js/Express)

Shows "noisy" automatic instrumentation for Node.js Express applications.

**Technology Stack:**

- Node.js + Express
- PostgreSQL with pg driver
- OpenTelemetry Auto-instrumentation

**Run the service:**

```shell
docker compose --profile express-auto build
docker compose --profile express-auto up
```

**Access:** <http://localhost:3000/songs/smells%20like%20teen%20spirit/nirvana>

### Manual Instrumentation (Python/Flask)

Demonstrates manual instrumentation with logging integration using Python and Flask.

**Technology Stack:**

- Python + Flask
- OpenTelemetry Python SDK + API
- Manual span and logging instrumentation

**Run the service:**

```shell
docker compose --profile manual-python build
docker compose --profile manual-python up
```

**Access:** <http://localhost:5000/songs/smells%20like%20teen%20spirit/nirvana>

## OpenTelemetry Weaver

This repository includes examples using OpenTelemetry Weaver for code generation and semantic convention validation. Weaver generates type-safe code artifacts and documentation from YAML schema definitions.

### Features

- **Code Generation** - Generates constants and types for multiple programming languages (Java, Python, Rust)
- **Documentation Generation** - Creates markdown documentation from semantic convention schemas
- **Schema Validation** - Validates YAML schema definitions for correctness
- **Live Validation** - Real-time validation of telemetry against defined semantic conventions

### Weaver Profiles

**Generate and emit example telemetry:**

```shell
docker compose --profile weaver-emit up
```

View the emitted telemetry at <http://localhost:16686> (Jaeger UI)

**Validate telemetry against semantic conventions:**

```shell
docker compose --profile weaver-check up
```

Then trigger telemetry:

```shell
curl localhost:5000/songs/smells%20like%20teen%20spirit/nirvana
```

Check the logs to see validation results for compliance with your semantic conventions.

For detailed Weaver usage instructions, see the [Weaver Guide](weaver/README.md).

## Troubleshooting

### Common Issues

**Services not starting:**

- Ensure Docker is running
- Check `.env` file exists and has correct values
- Verify no port conflicts (8080, 8081, 3000, 5000, 5432, 16686)

**No telemetry data:**

- Uncomment `verbosity` level on the [Collector config](./otel-collector/)
- Re-run the example
- Check OpenTelemetry collector logs: `docker compose logs otel-collector`
- Verify `OTEL_EXPORTER_OTLP_ENDPOINT` points to the collector

**Database connection errors:**

- Wait for PostgreSQL to fully initialize (check logs)
- Verify database credentials in environment variables
- Ensure `songs-db` service is running

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is open source and available under the Apache License 2.0.

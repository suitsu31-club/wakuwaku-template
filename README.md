# wakuwaku template

An agent-friendly Rust microservices template built on the
[`wakuwaku`](https://crates.io/crates/wakuwaku) and
[`kanau`](https://crates.io/crates/kanau) frameworks.

It gives you a modular, gRPC-first backend workspace with PostgreSQL, Redis, and
AMQP wired in, a clean layered module layout, and enough structure that both
humans and coding agents can add features without guessing where things go.

## Use this template

Click **“Use this template”** at the top-right of the GitHub repository page to
create your own repository from it, then start filling in the modules. Nothing
here is example-specific — it is an empty, ready-to-extend skeleton.

## Why "agent friendly"

- **One layout, repeated everywhere.** Every module has the same shape
  (`entities`, `services`, `events`, `hooks`, `rpc`, `config`, `utils`), so
  there is exactly one correct place for each kind of code.
- **One core abstraction.** All logic is a `kanau` `Processor`
  (*state + `async fn(Input) -> Result<Output, Error>`*), from SQL queries to
  business services to queue consumers.
- **Documented conventions.** Each crate carries module-level doc comments with
  copy-pasteable examples, and [`AGENTS.md`](AGENTS.md) spells out the rules for
  extending the codebase.
- **Compile-time safety.** `sqlx` checks queries against the database and
  protobuf contracts are generated, so whole classes of mistakes fail the build.

## Architecture

```
bin/
├── app-server/     # main server binary; runs the modules behind pluggable workers
└── manage-tool/    # CLI for migrations, config seeding, and admin tasks
lib/
└── app_protobuf/   # generated gRPC/protobuf types shared across the workspace
modules/
└── base/           # foundational + template module; copy its layout for new features
proto/              # protobuf service/message definitions
migrations/         # SQLx database migrations (.up.sql / .down.sql)
```

### Tech stack

- **Rust** (edition 2024) async on **Tokio**
- **wakuwaku** — AMQP / Redis / SQLx backend utilities and the shared error type
- **kanau** — the `Processor` abstraction and message-passing tools
- **gRPC + Tonic** for type-safe APIs
- **PostgreSQL + SQLx** for storage with compile-time-checked queries
- **Redis** for caching and ephemeral state
- **AMQP** for asynchronous inter-module events
- **OpenTelemetry** for tracing and metrics

### How the pieces fit

`bin/app-server` selects a *worker* (gRPC server, AMQP consumer, cron executor,
REST/webhook gateway) at startup, builds shared dependencies, and hands control
to the modules. Each module owns its slice of behaviour and talks to the others
only through gRPC calls or AMQP events — never by reaching into their internals.

## Getting started

1. Create your repo with **“Use this template”**.
2. Rename the workspace and the `base` module to suit your project.
3. Add your tables/views under `migrations/`, your API under `proto/`, and your
   logic under `modules/`.
4. Read [`AGENTS.md`](AGENTS.md) before adding code — it describes exactly how to
   organise each layer.

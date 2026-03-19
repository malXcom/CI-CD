# CI-CD

A RESTful CRUD API built with **Rust** and **Axum**, following MVC architecture with in-memory storage.

[![CI](https://github.com/malXcom/CI-CD/actions/workflows/ci.yml/badge.svg)](https://github.com/malXcom/CI-CD/actions/workflows/ci.yml)

---

## Stack

| Layer | Technology |
|---|---|
| HTTP framework | [Axum 0.8](https://github.com/tokio-rs/axum) |
| Async runtime | [Tokio](https://tokio.rs) |
| Serialization | [Serde](https://serde.rs) |
| Linting | Clippy + Rustfmt |
| Testing | cargo test + cargo-tarpaulin |
| CI/CD | GitHub Actions |

---

## Getting started

```bash
git clone https://github.com/malXcom/CI-CD.git
cd CI-CD
cargo run
# Server running on http://127.0.0.1:3000
```

Run the tests :

```bash
cargo test
```

Run linting :

```bash
cargo lint
```

---

## Project structure

```
src/
  main.rs                  # Entry point
  lib.rs                   # Module exports
  models/mod.rs            # Data structs
  store/mod.rs             # In-memory store
  errors/mod.rs            # AppError + HTTP mapping
  services/students.rs     # Business logic
  controllers/students.rs  # HTTP handlers
  routes/mod.rs            # Router assembly
tests/
  common.rs                # Shared test helpers
  unittests_test.rs        # Entry point
  students/
    get_test.rs
    post_test.rs
    put_test.rs
    delete_test.rs
    stats_search_test.rs
    service_test.rs
.github/workflows/ci.yml   # CI/CD pipeline
```

---

## Data model

### Student

| Property | Type | Required | Constraints |
|---|---|---|---|
| `id` | integer | auto | Unique, auto-incremented |
| `firstName` | string | yes | Min 2 characters |
| `lastName` | string | yes | Min 2 characters |
| `email` | string | yes | Valid email format, unique |
| `grade` | number | yes | Between 0 and 20 |
| `field` | string | yes | `informatique`, `mathematiques`, `physique`, `chimie` |

---

## API reference

Base URL : `http://127.0.0.1:3000`

All request and response bodies use `application/json`.

---

### GET /students

Returns all students.

**Request**

```http
GET /students HTTP/1.1
```

**Response** `200 OK`

```json
[
  {
    "id": 1,
    "firstName": "John",
    "lastName": "Doe",
    "email": "john@example.com",
    "grade": 17.5,
    "field": "informatique"
  },
  {
    "id": 2,
    "firstName": "Jane",
    "lastName": "Smith",
    "email": "jane@example.com",
    "grade": 12.0,
    "field": "mathematiques"
  }
]
```

---

### GET /students/:id

Returns a single student by ID.

**Request**

```http
GET /students/1 HTTP/1.1
```

**Response** `200 OK`

```json
{
  "id": 1,
  "firstName": "John",
  "lastName": "Doe",
  "email": "john@example.com",
  "grade": 17.5,
  "field": "informatique"
}
```

**Error responses**

| Status | Cause | Body |
|---|---|---|
| `400 Bad Request` | ID is not a valid integer (e.g. `/students/abc`) | `{ "error": "..." }` |
| `404 Not Found` | No student with this ID | `{ "error": "student 99 not found" }` |

---

### POST /students

Creates a new student.

**Request**

```http
POST /students HTTP/1.1
Content-Type: application/json

{
  "firstName": "Marie",
  "lastName": "Curie",
  "email": "marie@example.com",
  "grade": 19.5,
  "field": "physique"
}
```

**Response** `201 Created`

```json
{
  "id": 3,
  "firstName": "Marie",
  "lastName": "Curie",
  "email": "marie@example.com",
  "grade": 19.5,
  "field": "physique"
}
```

**Error responses**

| Status | Cause | Body |
|---|---|---|
| `400 Bad Request` | Missing required field | `{ "error": "..." }` |
| `409 Conflict` | Email already in use | `{ "error": "email marie@example.com already in use" }` |
| `422 Unprocessable Entity` | Validation failed (e.g. grade > 20, name too short) | `{ "error": "grade must be between 0 and 20" }` |

---

### PUT /students/:id

Updates an existing student. All fields are optional — only provided fields are updated.

**Request**

```http
PUT /students/1 HTTP/1.1
Content-Type: application/json

{
  "grade": 18.0,
  "field": "chimie"
}
```

**Response** `200 OK`

```json
{
  "id": 1,
  "firstName": "John",
  "lastName": "Doe",
  "email": "john@example.com",
  "grade": 18.0,
  "field": "chimie"
}
```

**Error responses**

| Status | Cause | Body |
|---|---|---|
| `404 Not Found` | No student with this ID | `{ "error": "student 99 not found" }` |
| `409 Conflict` | Email already in use by another student | `{ "error": "email x already in use" }` |
| `422 Unprocessable Entity` | Validation failed | `{ "error": "..." }` |

---

### DELETE /students/:id

Deletes a student by ID.

**Request**

```http
DELETE /students/1 HTTP/1.1
```

**Response** `204 No Content`

```
(empty body)
```

**Error responses**

| Status | Cause | Body |
|---|---|---|
| `404 Not Found` | No student with this ID | `{ "error": "student 99 not found" }` |

---

### GET /students/stats

Returns computed statistics across all students.

**Request**

```http
GET /students/stats HTTP/1.1
```

**Response** `200 OK`

```json
{
  "totalStudents": 2,
  "averageGrade": 14.75,
  "studentsByField": {
    "informatique": 1,
    "mathematiques": 1
  },
  "bestStudent": {
    "id": 1,
    "firstName": "John",
    "lastName": "Doe",
    "email": "john@example.com",
    "grade": 17.5,
    "field": "informatique"
  }
}
```

**Error responses**

| Status | Cause | Body |
|---|---|---|
| `404 Not Found` | No students in store | `{ "error": "no students registered" }` |

---

### GET /students/search?q=

Searches students by first name, last name, email or field.

**Request**

```http
GET /students/search?q=john HTTP/1.1
```

**Response** `200 OK`

```json
[
  {
    "id": 1,
    "firstName": "John",
    "lastName": "Doe",
    "email": "john@example.com",
    "grade": 17.5,
    "field": "informatique"
  }
]
```

**Query parameter**

| Parameter | Required | Description |
|---|---|---|
| `q` | no | Search term, min 2 characters. Omit to return all students. |

**Error responses**

| Status | Cause | Body |
|---|---|---|
| `404 Not Found` | No results for this query | `{ "error": "no students found for \"xyz\"" }` |
| `422 Unprocessable Entity` | Query shorter than 2 characters | `{ "error": "query must be at least 2 characters" }` |

---

## Error format

All errors return a JSON body with a single `error` field :

```json
{
  "error": "human readable message"
}
```

---

## CI/CD pipeline

The pipeline runs on every push and pull request to `main`, across two Rust versions (`stable` and `beta`) :

```
Checkout → Setup → Install → Lint → Test → Build
```

| Job | What it does |
|---|---|
| Setup | Verifies toolchain versions |
| Install | Fetches dependencies, runs `cargo audit` |
| Lint | `cargo fmt --check` + `cargo clippy` |
| Test | `cargo test` + `cargo tarpaulin` (min 80% coverage) |
| Build | `cargo build --release` |

Each stage only runs if the previous one passed. A failed test blocks the build.

---

## Pre-commit hook

Auto-format before every commit :

```bash
cp scripts/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

The hook runs `cargo fmt --check` and blocks the commit if any file is not formatted. Run `cargo fmt` to fix.

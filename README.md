# 🔗 Rust URL Shortener (Axum + SQLx + PostgreSQL)

A fast and minimal URL shortening service built using **Rust**, **Axum**, **SQLx**, and **PostgreSQL**.  
It generates short codes for URLs and redirects users to the original link while tracking hit counts.

---

## 🚀 Features
- Generate short URLs (`/shorten`)
- Redirect to original URL (`/{code}`)
- Validates URLs before storing
- Auto-generates unique alphanumeric short codes
- Tracks number of hits per shortened URL
- Uses PostgreSQL with SQLx
- Fully async (Tokio runtime)
- Environment-based configuration

---

## 📁 Project Structure

src/
├── handlers/
│ ├── shorten.rs
│ └── redirect.rs
├── models.rs
├── state.rs
├── main.rs


---

## 🛠️ Technologies Used
- **Rust**
- **Axum** (web framework)
- **SQLx** (async PostgreSQL)
- **Tokio** (async runtime)
- **PostgreSQL**
- **Dotenvy** (environment loader)
- **Tracing** for logging

---

## 📦 API Endpoints

### **POST /shorten**
Creates a shortened URL.

**Request Body**
```json
{
  "url": "https://example.com/some/long/link"
}

**Success Response**
{
  "short_url": "http://localhost:3000/Ab12Xy"
}

# ⚡ Moya Bank: Bitcoin Lightning Bank

A modern Bitcoin Lightning–powered banking web application that allows users to manage accounts, create Lightning invoices, pay invoices, and track their full payment history — all through a fast, secure, and intuitive interface.

---

## 🚀 Features

### **User Management**

- **Signup & Login**
  Secure onboarding with password hashing, session management, or JWT-based authentication.

### **Lightning Operations**

- **Create Invoice**
  Users can generate Lightning invoices for receiving Bitcoin payments.
- **Pay Invoice**
  Users can pay Lightning invoices directly through the app using connected LND (or another LN backend).

### **Payment Management**

- **View Payment History**
  Users see a full list of all successful, pending, or failed Lightning payments.
- **View All Invoices**
  Users can view their created invoices, including status (paid/unpaid/expired).

---

## 🏗 Architecture Overview

Lightning Bank is built with:

- **Backend:** Rust + Axum (or Node.js/NestJS depending on your setup)
- **Lightning Node:** LND (gRPC via `lnrpc`)
- **Database:** PostgreSQL
- **Frontend:** React / Next.js (optional)

The backend communicates with LND to:

- Create invoices
- Decode invoices
- Initiate payments
- Track payment and invoice status

---

## 📦 Installation

### **1. Clone the repository**

```bash
git clone https://github.com/your-username/lightning-bank.git
cd lightning-bank
```

### **2. Set up environment variables**

Create a `.env` file:

```
DATABASE_URL=your_database_connection_string
LND_GRPC_URL=localhost:10009
LND_TLS_CERT=/path/to/tls.cert
LND_MACAROON=/path/to/admin.macaroon
JWT_SECRET=your_jwt_secret
```

### **3. Install dependencies**

For Rust backend:

```bash
cargo build
```

For Node.js:

```bash
npm install
```

### **4. Run the server**

Rust:

```bash
cargo run
```

Node.js:

```bash
npm run start
```

---

## 📚 API Endpoints

### **Auth**

| Method | Endpoint       | Description       |
| ------ | -------------- | ----------------- |
| POST   | `/auth/signup` | Create a new user |
| POST   | `/auth/login`  | User login        |

### **Invoices**

| Method | Endpoint          | Description            |
| ------ | ----------------- | ---------------------- |
| POST   | `/invoice/create` | Generate a LN invoice  |
| GET    | `/invoice/all`    | View all user invoices |

### **Payments**

| Method | Endpoint           | Description             |
| ------ | ------------------ | ----------------------- |
| POST   | `/payment/pay`     | Pay a Lightning invoice |
| GET    | `/payment/history` | View all user payments  |

---

## 🧱 Tech Stack (Recommended)

- **Backend:** Rust + Axum (fast, safe, async)
- **Lightning:** LND (via gRPC)
- **Database:** PostgreSQL
- **Frontend:** Next.js + TailwindCSS
- **Auth:** JWT or session cookies

---

## 🔐 Security Considerations

- All API routes are protected with authentication.
- Access to LND macaroon is strictly locked down.
- Passwords are hashed using Argon2 / bcrypt.
- Invoice and payment data stored securely in DB.

---

## 🧪 Testing

Run backend tests:

```bash
cargo test
```

Frontend tests:

```bash
npm test
```

---

## 📈 Future Improvements

- WebLN integration
- Email/mobile notifications for payments
- Multi-account support
- On-chain fallback payments
- Fiat display conversions (USD/NGN/EUR)

---

## 📝 License

MIT License © 2025 Lightning Bank

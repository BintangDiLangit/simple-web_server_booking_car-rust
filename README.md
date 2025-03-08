# 🚀 Rust Web Server - Simple Booking API

This is a simple **Booking Web Server** built with **Rust** using **Actix Web** and **MongoDB**. The API allows you to manage **bookings, cars, and owners** in a database.

## 📌 Features

- **CRUD operations** for Bookings, Cars, and Owners.
- **Actix Web** for handling HTTP requests.
- **MongoDB** as the database.
- **Async/Await** for efficient performance.

---

## 📦 Project Structure

```
rust-web-server/
├── src/
│   ├── models/          # Data models (Booking, Car, Owner)
│   ├── routes/          # API routes (Booking, Car, Owner)
│   ├── services/        # Database service layer
│   ├── main.rs          # Application entry point
├── Cargo.toml           # Rust dependencies
└── README.md            # Documentation
```

---

## 🛠️ Installation & Setup

### **1️⃣ Prerequisites**

- Install **Rust**: [Rust Installation](https://www.rust-lang.org/tools/install)
- Install **MongoDB**: [MongoDB Installation](https://www.mongodb.com/docs/manual/installation/)

### **2️⃣ Clone the Repository**

```sh
git clone https://github.com/BintangDiLangit/simple-web_server_booking_car-rust
cd rust-web-server
```

### **3️⃣ Install Dependencies**

```sh
cargo build
```

### **4️⃣ Start the MongoDB Server**

Ensure MongoDB is running on `localhost:27017`.

```sh
mongod --dbpath /your/db/path
```

### **5️⃣ Run the Server**

```sh
cargo run
```

The server will start on **`http://localhost:5001`**.

---

## 📡 API Endpoints

### **1️⃣ Test Server**

- `GET /` → Returns `Hello Youtube`

### **2️⃣ Booking Routes**

- `POST /booking` → Create a new booking
- `PUT /booking/{id}/cancel` → Cancel a booking
- `GET /booking` → Get all bookings

### **3️⃣ Car Routes**

- `POST /car` → Add a new car

### **4️⃣ Owner Routes**

- `POST /owner` → Add a new owner

---

## 📝 License

This project is **open-source** and available under the **MIT License**.

🚀 Happy Coding! Let me know if you need any improvements. 😊

# Booklyn

A web application for exploring and reviewing books using Amazon reviews data.


## Prerequisites

Make sure you have the following installed:

* [Docker](https://docs.docker.com/get-docker/) & [docker-compose](https://docs.docker.com/compose/install/)
* Python 3.8+ (for local script execution)

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/Booklyn.git
cd Booklyn
```

### 2. Docker Setup

Build and start the containers:

```bash
docker-compose up --build
```

This will launch two services:

* **frontend**: Vue 3 application at `http://localhost:5173`
* **backend**: Python API connected to PostgreSQL on port `5433`

### 3. Database Initialization

The application requires two CSV files from the [Amazon Books Reviews dataset on Kaggle](https://www.kaggle.com/datasets/mohamedbakhet/amazon-books-reviews).

1. Download and extract the dataset.
2. Copy the following files into the project directory:

   ```
   Booklyn/database/construct_db/dataset/books_data.csv
   Booklyn/database/construct_db/dataset/Books_rating.csv
   ```
3. From the project root, run the database construction script:

   ```bash
   cd database/construct_db
   python constructdb.py
   ```

This script will:

* Create the `booksdb` database schema
* Load raw data into staging tables
* Transform and insert data into production tables

## Usage

* Frontend: Open your browser and navigate to `http://localhost:5173`.
* Backend API: Available at `http://localhost:8000/api` (adjust port if configured).

Example API call:

```bash
curl http://localhost:8000/api/books?limit=10
```

## Project Structure

```
Booklyn/
├── backend/                # Python API service
│   ├── app/                # FastAPI application code
├── frontend/               # Vue 3 + Vite frontend
│   └── src/
├── database/
│   └── construct_db/       # Scripts and dataset for DB init
└── docker-compose.yml      # Service definitions
```
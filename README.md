# Booklyn


## Installation

### 1. Docker Setup

Build and start the containers:

```bash
docker-compose up --build # around 500 s
```

This will launch two services:

* **frontend**: Vue 3 application at `http://localhost:5173`
* **backend**: Python API connected to PostgreSQL on port `5433`

### 2. Database Initialization


1. Download and extract the dataset.

The application requires two CSV files from the [Amazon Books Reviews dataset on Kaggle](https://www.kaggle.com/datasets/mohamedbakhet/amazon-books-reviews).

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

Frontend:

http://localhost:5173

Backend:

http://localhost:8080/swagger-ui/#/




----------------
## For frontend to read data, run （Yoyo part)
.env should contain OPENAI_API_KEY
   ```
   cd frontend
   source ../.env
   npm run generate:data
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
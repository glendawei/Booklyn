# Booklyn


## Installation

### 1. Docker Startup

Build and start the containers:

```bash
docker-compose up --build # around 500 s
```

This will launch two services:

* **frontend**: Vue 3 application at `http://localhost:5173`
* **backend**: HTTP server listening at `http://localhost:8080`

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
   Note: please ensure that you have packages `psycopg2` and `pandas` installed in your Python environment.

### 3. Done

The services are now available at the following locations.

Frontend:

- http://localhost:5173

Backend:

- HTTP server: http://localhost:8080
- API doc: http://localhost:8080/swagger-ui/

## For frontend people

api at 
` frontend/src/api `
if needed

example usage at `frontend/src/generateDataFiles.js`

but basically don't needed, just use the data from `/Users/yl/Booklyn/frontend/src/data`

-> will be updated when service launched

----------------
## For frontend to read data, run (Yoyo part)
.env should contain OPENAI_API_KEY
   ```
   cd frontend
   source ../.env
   npm run generate:data
   ```

## Project Structure

```
Booklyn/
├── backend/                # Actix Web framework to provide RESTful APIs
│   ├── src/
├── frontend/               # Vue 3 + Vite frontend
│   └── src/
├── database/
│   └── construct_db/       # Scripts and dataset for DB init
└── docker-compose.yml      # Service definitions
```

#  Booklyn

A book-centric social platform that integrates reviews, AI-powered insights, and personalized recommendations.

## Project Links

*  **Website**: [http://40.81.200.89:5173](http://40.81.200.89:5173)
*  **Report**: [Google Docs Report](https://docs.google.com/document/d/1TOZOnQRjgVhokk_2AjmXlz0nUZ_3nyptOAf1_2pEXk8/edit?tab=t.0#heading=h.5tlvz7le0vsy)

##  Installation Guide (For Developers)

### Step 1: Start Up with Docker

Build and launch the containers:

```bash
docker-compose up --build 
```

This starts two services:

*  **Frontend**: Vue 3 app at `http://localhost:5173`
*  **Backend**: Actix Web HTTP server at `http://localhost:8080`

###  Step 2: Database Initialization

1. **Download datasets**

   Go to [Amazon Books Reviews dataset on Kaggle](https://www.kaggle.com/datasets/mohamedbakhet/amazon-books-reviews) and extract:

   * `books_data.csv`
   * `Books_rating.csv`

3. **Place files in the project**

   ```
   Booklyn/database/construct_db/dataset/books_data.csv
   Booklyn/database/construct_db/dataset/Books_rating.csv
   ```

4. **Run database construction script**

   ```bash
   cd database/construct_db
   python constructdb.py
   ```

   Note: please make sure packages [pandas](https://pandas.pydata.org/) and [psycopg2](https://pypi.org/project/psycopg2/) are installed in your Python environment before running the commands above.

###  Step 3: Service Verification

*  Frontend: [http://localhost:5173](http://localhost:5173)
*  Backend: [http://localhost:8080](http://localhost:8080)
*  OpenAPI Doc: [http://localhost:8080/swagger-ui/](http://localhost:8080/swagger-ui/)

##  Frontend Developer Notes

###  API Directory

API logic can be found in:

```
frontend/src/api/
```

Usage examples: `frontend/src/generateDataFiles.js`

**Note:** You usually don't need to fetch manually. Use mock data from:

```
frontend/src/data/
```

### Generate Frontend Data (Yoyo's Part)

1. Add your OpenAI API key to `.env`:

   ```
   OPENAI_API_KEY=your_key_here
   ```

2. Run:

   ```bash
   cd frontend
   source ../.env
   npm run generate:data
   ```

##  Project Structure

```
Booklyn/
├── backend/                # Actix Web + SQLx backend
│   └── src/
├── frontend/               # Vue 3 + Vite frontend
│   └── src/                # Frontend components and assets
├── database/
│   └── construct_db/       # DB setup scripts and dataset
└── docker-compose.yml      # Docker service configuration
```



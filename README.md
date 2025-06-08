Here's a visually enhanced and more organized version of your `README.md` for **Booklyn**, following best practices for structure, hierarchy, and readability:

---

#  Booklyn

> A book-centric social platform that integrates reviews, AI-powered insights, and personalized recommendations.

---

## Project Links

*  **Website**: [http://40.81.200.89:5173](http://40.81.200.89:5173)
*  **Report**: [Google Docs Report](https://docs.google.com/document/d/1TOZOnQRjgVhokk_2AjmXlz0nUZ_3nyptOAf1_2pEXk8/edit?tab=t.0#heading=h.5tlvz7le0vsy)

---

##  Installation Guide (For Developers)

### Step 1: Start with Docker

Build and launch the containers:

```bash
docker-compose up --build 
```

This starts two services:

*  **Frontend**: Vue 3 app at `http://localhost:5173`
*  **Backend**: Actix HTTP API at `http://localhost:8080`

---

### 🗃️ Step 2: Database Initialization

1. **Download dataset**
   Go to the [Amazon Books Reviews dataset on Kaggle](https://www.kaggle.com/datasets/mohamedbakhet/amazon-books-reviews) and extract:

   * `books_data.csv`
   * `Books_rating.csv`

2. **Place files in the project**

   ```
   Booklyn/database/construct_db/dataset/books_data.csv
   Booklyn/database/construct_db/dataset/Books_rating.csv
   ```

3. **Run database construction script**

   ```bash
   cd database/construct_db
   python constructdb.py
   ```

   > ⚠️ Make sure `pandas` and `psycopg2` are installed.

---

### ✅ Step 3: Service Verification

*  Frontend: [http://localhost:5173](http://localhost:5173)
*  Backend API: [http://localhost:8080](http://localhost:8080)
*  API Docs (Swagger): [http://localhost:8080/swagger-ui/](http://localhost:8080/swagger-ui/)

---

## 💻 Frontend Developer Notes

### 📁 API Directory

API logic can be found in:

```
frontend/src/api/
```

> Usage examples: `frontend/src/generateDataFiles.js`
> **Note:** You usually don't need to fetch manually. Use mock data from:

```
frontend/src/data/
```

---

### 🤖 Generate Frontend Data (Yoyo's Part)

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

---

##  Project Structure

```
Booklyn/
├── backend/                # Actix Web backend
│   └── src/                # Backend code
├── frontend/               # Vue 3 + Vite frontend
│   └── src/                # Frontend components and assets
├── database/
│   └── construct_db/       # DB setup scripts and dataset
└── docker-compose.yml      # Docker service configuration
```

---

Let me know if you'd like me to add badges, license, contribution guide, or convert this into a downloadable `README.md` file 😌

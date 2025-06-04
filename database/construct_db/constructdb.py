import pandas as pd
from psycopg2.extras import execute_values
from db_connection import get_connection
import random
from datetime import datetime, timedelta
import ast


# Read dataset/books_data.csv
df_books = pd.read_csv('dataset/books_data.csv')
print(df_books.head())
df_ratings = pd.read_csv('dataset/Books_rating.csv')
df_ratings.rename(columns={'Id': 'book_id'}, inplace=True)
print(df_ratings.head())

def parse_date(date_str):
    if pd.isna(date_str):
        return None
    try:
        # 嘗試完整日期
        return datetime.strptime(date_str, "%Y-%m-%d").date()
    except ValueError:
        try:
            # 嘗試年-月格式
            return datetime.strptime(date_str, "%Y-%m").date()
        except ValueError:
            try:
                # 嘗試年份格式
                return datetime.strptime(date_str, "%Y").date()
            except ValueError:
                # 無法解析的格式
                return None
books_df = pd.read_csv('dataset/books_data.csv')
books_df.head()
books_df.columns = [
    'title', 'description', 'authors', 'cover_url', 'preview_link', 
    'publisher', 'published_date', 'info_link', 'categories', 'ratings_count'
]
#Change ratings_count NaN to 0
books_df['ratings_count'] = books_df['ratings_count'].fillna(int(0))
books_df.head()

# Data cleaning
def safe_eval(val):
    try:
        return ast.literal_eval(val) if isinstance(val, str) else []
    except (ValueError, SyntaxError):
        return []

books_df['authors'] = books_df['authors'].apply(safe_eval)
books_df['categories'] = books_df['categories'].apply(safe_eval)

# 移除空白，避免錯誤的名稱出現
books_df['authors'] = books_df['authors'].apply(lambda x: [author.strip() for author in x])
books_df['categories'] = books_df['categories'].apply(lambda x: [cat.strip() for cat in x])

books_df.head()
books_df['published_date'] = books_df['published_date'].apply(parse_date)
insert_books = """
    INSERT INTO books (
        title, description, cover_url, preview_link, info_link,
        publisher, published_date, categories, ratings_count, ai_summary
    ) VALUES %s
    ON CONFLICT (title) DO NOTHING;
"""

book_values = [(
    row.title, row.description, row.cover_url, row.preview_link, row.info_link,
    row.publisher, row.published_date, row.categories, row.ratings_count, row.ai_summary
) for _, row in books_df.iterrows()]

with get_connection() as conn:
    with conn.cursor() as cur:
        execute_values(cur, insert_books, book_values)
        conn.commit()
    print("Books 資料已成功匯入")

# Start inserting authors

with get_connection() as conn:
    with conn.cursor() as cur:
        cur.execute("SELECT name, author_id FROM authors;")
        existing_authors = {name: author_id for name, author_id in cur.fetchall()}           
all_authors = set()
for authors in books_df['authors']:
    all_authors.update(authors)

new_authors = [(author,) for author in all_authors if author not in existing_authors]

if new_authors:
    insert_authors = """
        INSERT INTO authors (name)
        VALUES %s
        ON CONFLICT (name) DO NOTHING;
    """

    with get_connection() as conn:
        with conn.cursor() as cur:
            execute_values(cur, insert_authors, new_authors)
            conn.commit()
        print(f"{len(new_authors)} 位新作者已成功匯入到 authors 表格")

with get_connection() as conn:
    with conn.cursor() as cur:
        cur.execute("SELECT name, author_id FROM authors;")
        existing_authors = {name: author_id for name, author_id in cur.fetchall()}

def insert_book_authors():
    with get_connection() as conn:
        with conn.cursor() as cur:
            # 取得書名到 book_id 的 mapping
            cur.execute("SELECT book_id, title FROM books;")
            book_map = {title: book_id for book_id, title in cur.fetchall()}

            # 插入 book_authors 
            authors_data = []
            for _, row in books_df.iterrows():
                book_id = book_map.get(row['title'])
                if book_id:
                    for seq, author in enumerate(row['authors']):
                        author_id = existing_authors.get(author)
                        if author_id:
                            authors_data.append((book_id, author_id, seq))

            execute_values(cur, """
                INSERT INTO book_authors (book_id, author_id, seq)
                VALUES %s
                ON CONFLICT DO NOTHING;
            """, authors_data)
            conn.commit()
        print(f"{len(authors_data)} 筆資料已成功匯入到 book_authors 關聯表")
        
insert_book_authors()

reviews_df = pd.read_csv('dataset/Books_rating.csv')

harry_potter_reviews = reviews_df[reviews_df['Title'].str.contains('Harry Potter', case=False, na=False)]

reviews_df = reviews_df[~reviews_df['Title'].str.contains('Harry Potter', case=False, na=False)]

# 抽取 10% 的其他 reviews
reviews_df = reviews_df.sample(frac=0.1, random_state=42)

# 合併兩個 DataFrame
reviews_df = pd.concat([harry_potter_reviews, reviews_df], ignore_index=True)

# 檢查結果
print(f"Harry Potter reviews: {len(harry_potter_reviews)}")
print(f"Other reviews: {len(reviews_df)}")

# 轉換 Unix Timestamp 到日期格式
reviews_df['review/time'] = pd.to_datetime(reviews_df['review/time'], unit='s')

# 隨機化時間（加上隨機的時、分、秒）
def randomize_time(date):
    random_hours = random.randint(0, 23)
    random_minutes = random.randint(0, 59)
    random_seconds = random.randint(0, 59)
    return date + timedelta(hours=random_hours, minutes=random_minutes, seconds=random_seconds)

# 套用隨機時間
reviews_df['review/time'] = reviews_df['review/time'].apply(randomize_time)
reviews_df.head()
reviews_df.columns = [
    'id', 'title', 'price', 'user_id_src', 'profile_name', 
    'review_helpfulness', 'review_score', 'review_time', 
    'review_summary', 'review_text'
]
reviews_df[['helpful_yes', 'helpful_total']] = reviews_df['review_helpfulness'].str.split('/', expand=True).fillna(0)
reviews_df['helpful_yes'] = reviews_df['helpful_yes'].astype(int)
reviews_df['helpful_total'] = reviews_df['helpful_total'].astype(int)
reviews_df['price'] = pd.to_numeric(reviews_df['price'], errors='coerce').fillna(0)
reviews_df.head()
with get_connection() as conn:
    with conn.cursor() as cur:
        cur.execute("SELECT title, book_id FROM books;")
        book_map = {title: book_id for title, book_id in cur.fetchall()}
review_values = []
for _, row in reviews_df.iterrows():
    book_id = book_map.get(row['title'])
    if not book_id:
        print(f"找不到對應的書籍: {row['title']}，跳過...")
        continue

    review_values.append((
        book_id,                             # 書籍ID
        None,                                # 使用者ID，因為是外部來源
        True,                                # 外部評論
        "external",                          # 來源平台 (external)
        row['user_id_src'],                  # 外部平台使用者 ID
        row['profile_name'],                 # 使用者名稱
        row['review_score'],                 # 評分
        row['price'],                        # 價格
        row['helpful_yes'],                  # 有幫助的票數
        row['helpful_total'],                # 總票數
        row['review_time'],                  # 評論時間
        row['review_summary'],               # 評論摘要
        row['review_text']                   # 評論內容
    ))

insert_reviews = """
    INSERT INTO reviews (
        book_id, user_id, is_external, source, 
        user_id_src, profile_name, rating, price, 
        helpful_yes, helpful_total, review_time, summary, content
    ) VALUES %s;
"""
batch_size = 1000  # 每次 1000 筆
total_records = len(review_values)

for i in range(0, total_records, batch_size):
    batch = review_values[i:i + batch_size]
    with get_connection() as conn:
        with conn.cursor() as cur:
            execute_values(cur, insert_reviews, batch)
            conn.commit()
    print(f"Inserted batch {i} - {i + batch_size}")
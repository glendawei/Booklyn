import psycopg2
from psycopg2.extras import execute_values

# 修改成你的連線資料
DB_CONFIG = {
    'dbname': 'booksdb',
    'user': 'postgres',
    'password': 'pass',
    'host': 'localhost',
    'port': 5434  # 你的 Docker 是 5434
}

def get_connection():
    conn = psycopg2.connect(**DB_CONFIG)
    return conn

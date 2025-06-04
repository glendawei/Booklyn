CREATE TABLE IF NOT EXISTS users (
    user_id BIGSERIAL PRIMARY KEY,                          -- 使用者ID
    email TEXT UNIQUE,                                      -- 使用者註冊的 email  
    password_hash TEXT,                                     -- 密碼 hash value
    display_name TEXT NOT NULL,                             -- username             
    role TEXT CHECK (role IN ('reader', 'author', 'admin')), -- 使用者身份 (reader, author, or admin)
    bio TEXT,                                               -- 簡介             
    avatar TEXT,                                            -- 頭像連結
    website TEXT,                                           -- 個人網站連結                          
    created_at TIMESTAMPTZ DEFAULT now()                    -- 註冊時間
);

CREATE TABLE IF NOT EXISTS authors (
    author_id BIGSERIAL PRIMARY KEY,                        -- 作者 ID，自動生成
    name TEXT UNIQUE NOT NULL,                              -- 作者名字
    user_id BIGINT REFERENCES users(user_id) ON DELETE SET NULL, -- 使用者 ID (如果是平台使用者)
    bio TEXT,                                               -- 作者簡介
    website TEXT,                                           -- 個人網站連結
    created_at TIMESTAMPTZ DEFAULT now()                    -- 建立時間
);

CREATE TABLE IF NOT EXISTS books (
    book_id BIGSERIAL PRIMARY KEY,                          -- 書籍ID，自動生成
    title TEXT NOT NULL UNIQUE,                             -- 書名 (books_data->Title)
    description TEXT,                                       -- 書籍描述 (books_data->description)
    cover_url TEXT,                                         -- 封面圖片連結 (books_data->image)
    preview_link TEXT,                                      -- Google Books 的預覽連結 (books_data->previewLink)
    info_link TEXT,                                         -- 更多資訊的連結 (books_data->infoLink)
    publisher TEXT,                                         -- 出版商 (books_data->publisher)
    published_date DATE,                                    -- 出版日期 (books_data->publishedDate)
    categories TEXT[],                                      -- 書籍分類 (books_data->categories)
    ratings_count INT DEFAULT 0,                            -- 評分次數 (books_data->ratingsCount)
    ai_summary TEXT,                                         -- AI 生成的書籍摘要
    isbn_13 CHAR(13) UNIQUE,                                -- ISBN 碼，唯一
    created_at TIMESTAMPTZ DEFAULT now()                    -- 書籍資料建立時間
);

CREATE TABLE IF NOT EXISTS book_authors (
    book_id BIGINT REFERENCES books(book_id) ON DELETE CASCADE, -- 書籍ID
    author_id BIGINT REFERENCES authors(author_id) ON DELETE CASCADE, -- 作者ID (books_data->authors)
    seq INT,                                                    -- 作者順序
    PRIMARY KEY (book_id, author_id)
);

CREATE TABLE IF NOT EXISTS reviews (
    review_id BIGSERIAL PRIMARY KEY,                           -- 評論 ID，自動生成
    book_id BIGINT REFERENCES books(book_id) ON DELETE CASCADE,-- 書籍ID (透過 Title 對應取得，不是資料集裡面的 book_is)
    user_id BIGINT REFERENCES users(user_id) ON DELETE SET NULL, -- 使用者ID，若是外部評論則可為 NULL
    is_external BOOLEAN DEFAULT FALSE,                         -- 是否為外部評論，預設是 FALSE
    source TEXT DEFAULT 'local',                               -- 來源平台 (local, Amazon, Goodreads, etc.)
    source_review_id TEXT,                                     -- 來源平台的評論ID（外部評論才有）
    rating SMALLINT CHECK (rating >= 0 AND rating <= 5),       -- 評分範圍 0~5 (Books_rating->review/score)
    price NUMERIC CHECK (price >= 0),                          -- 價格 (Books_rating->Price)
    user_id_src TEXT,                                          -- 使用者在原始平台的 ID (Books_rating->User_id)
    profile_name TEXT,                                         -- 原始平台的使用者名稱 (Books_rating->profileName)
    helpful_yes INT DEFAULT 0 CHECK (helpful_yes >= 0),        -- 有幫助的票數 (Books_rating->review/helpfulness)
    helpful_total INT DEFAULT 0 CHECK (helpful_total >= 0),    -- 總票數 (Books_rating->review/helpfulness)
    review_time TIMESTAMPTZ DEFAULT now(),                     -- 評論時間 (Books_rating->review/time)
    summary TEXT,                                              -- 評論摘要 (Books_rating->review/summary)
    content TEXT                                               -- 評論內容 (Books_rating->review/text)
);

CREATE TABLE IF NOT EXISTS review_ai (
    review_id BIGINT PRIMARY KEY REFERENCES reviews(review_id) ON DELETE CASCADE, -- 參考 reviews 的 review_id
    credibility_score NUMERIC(3, 2) CHECK (credibility_score >= 0 AND credibility_score <= 1), -- AI 分析可信度
    summary_ai TEXT,                                          -- AI 自動產生的摘要
    model_version TEXT,                                       -- AI 模型版本
    analyzed_at TIMESTAMPTZ DEFAULT now()                     -- 分析時間
);

CREATE TABLE IF NOT EXISTS comments (
    comment_id BIGSERIAL PRIMARY KEY,                         -- 留言 ID，自動生成
    review_id BIGINT REFERENCES reviews(review_id) ON DELETE CASCADE, -- 參考評論的 ID
    user_id BIGINT REFERENCES users(user_id) ON DELETE CASCADE, -- 留言的使用者
    parent_id BIGINT REFERENCES comments(comment_id) ON DELETE CASCADE, -- 回覆的留言
    content TEXT NOT NULL,                                    -- 留言內容
    created_at TIMESTAMPTZ DEFAULT now()                      -- 留言時間
);

CREATE TABLE IF NOT EXISTS review_votes (
    review_id BIGINT REFERENCES reviews(review_id) ON DELETE CASCADE, -- 參考評論的 ID
    user_id BIGINT REFERENCES users(user_id) ON DELETE CASCADE,       -- 投票的使用者
    vote BOOLEAN NOT NULL,                                            -- 投票結果 (有幫助/沒幫助)
    PRIMARY KEY (review_id, user_id)
);

CREATE TABLE IF NOT EXISTS reading_list_items (
    item_id BIGSERIAL PRIMARY KEY,                          -- 個人閱讀清單項目 ID
    user_id BIGINT REFERENCES users(user_id) ON DELETE CASCADE, -- 使用者 ID
    book_id BIGINT REFERENCES books(book_id) ON DELETE CASCADE, -- 書籍 ID
    status TEXT NOT NULL,                                    -- 閱讀狀態
    note TEXT,                                               -- 註記
    created_at TIMESTAMPTZ DEFAULT now()                     -- 建立時間
);

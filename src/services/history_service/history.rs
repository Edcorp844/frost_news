use std::sync::Arc;

use rusqlite::{Connection, Result, params};
use sha2::{Digest, Sha256};

use crate::types::{news_article::NewsArticle, persistent_articel::PersistentArticle};

#[derive(Debug)]
pub struct HistoryService {
    conn: Connection,
}

impl HistoryService {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("history.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS article_history (
                    id TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    url TEXT NOT NULL,
                    description TEXT,     
                    content TEXT,       
                    published_at TEXT,   
                    image_url TEXT,
                    visit_time INTEGER   
                )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn save_to_history(&self, article: PersistentArticle) -> rusqlite::Result<()> {
        let now = chrono::Utc::now().timestamp();

        self.conn.execute(
            "INSERT OR REPLACE INTO article_history 
            (id, title, url, description, content, published_at, image_url, visit_time) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                article.id,
                article.title,
                article.url,
                article.description,
                article.content,
                article.published_at,
                article.image_url,
                article.visit_time
            ],
        )?;
        Ok(())
    }

    pub fn get_all_history(&self) -> rusqlite::Result<Vec<Arc<PersistentArticle>>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, url, description, content, published_at, image_url, visit_time
             FROM article_history 
             ORDER BY visit_time DESC 
             LIMIT 10", // Limit for sidebar performance
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(PersistentArticle {
                id: row.get(0)?,
                title: row.get(1)?,
                url: row.get(2)?,
                description: row.get(3)?,
                content: row.get(4)?,
                published_at: row.get(5)?,
                image_url: row.get(6)?,
                visit_time: row.get(7)?,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            if let Ok(article) = row {
                results.push(Arc::new(article));
            }
        }
        Ok(results)
    }

    pub fn delete_entry(&self, id: String) -> rusqlite::Result<()> {
        self.conn
            .execute("DELETE FROM article_history WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn clear_all_history(&self) -> rusqlite::Result<()> {
        self.conn.execute("DELETE FROM article_history", [])?;
        Ok(())
    }
}

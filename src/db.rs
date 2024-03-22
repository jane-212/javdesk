use std::{fs::create_dir_all, path::PathBuf};

use gpui::*;
use rusqlite::Connection;

use crate::app;

pub struct DB {
    connection: Connection,
}

impl DB {
    const PAGE_COUNT: i32 = 10;

    fn data_path() -> PathBuf {
        let username = whoami::username();
        let user_dir = PathBuf::from("/Users").join(username);

        user_dir.join(".local/share").join(app::NAME)
    }

    pub fn init(cx: &mut AppContext) {
        let db_path = Self::data_path();
        create_dir_all(&db_path).ok();
        let connection = Connection::open(db_path.join("db.sqlite")).expect("open db failed");
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS javbus (
                    id TEXT PRIMARY KEY,
                    href TEXT NOT NULL,
                    title TEXT NOT NULL,
                    cover TEXT NOT NULL,
                    date TEXT NOT NULL
                )",
                (),
            )
            .expect("init db failed");

        cx.set_global(Self::new(connection));
    }

    fn new(connection: Connection) -> Self {
        Self { connection }
    }

    pub fn like(&self, like: (&str, &str, &str, &str, &str)) {
        self.connection
            .execute(
                "INSERT INTO javbus (id, href, title, cover, date) VALUES (?1, ?2, ?3, ?4, ?5)",
                (&like.0, &like.1, &like.2, &like.3, &like.4),
            )
            .ok();
    }

    pub fn unlike(&self, id: &str) {
        self.connection
            .execute("DELETE FROM javbus WHERE id = ?1", [(id)])
            .ok();
    }

    pub fn is_liked(&self, id: &str) -> bool {
        self.connection
            .query_row("SELECT COUNT(*) FROM javbus WHERE id = ?1", [id], |row| {
                row.get(0).map(|count: i32| count > 0)
            })
            .unwrap_or(false)
    }

    pub fn likes(&self, page: i32) -> (i32, LikeItems) {
        let likes = Vec::new();
        let Ok(total) = self
            .connection
            .query_row("SELECT COUNT(*) FROM javbus", [], |row| {
                row.get(0).map(|total: i32| {
                    let count = total / Self::PAGE_COUNT;
                    if count * Self::PAGE_COUNT < total {
                        count + 1
                    } else {
                        count
                    }
                })
            })
        else {
            return (1, likes);
        };
        let Ok(mut stmt) = self
            .connection
            .prepare("SELECT id, href, title, cover, date FROM javbus limit ?1, ?2")
        else {
            return (total, likes);
        };

        (
            total,
            stmt.query_map(((page - 1) * Self::PAGE_COUNT, Self::PAGE_COUNT), |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            })
            .ok()
            .map(|likes| likes.into_iter().flat_map(|row| row.ok()).collect())
            .unwrap_or(likes),
        )
    }
}

type LikeItems = Vec<(String, String, String, String, String)>;

impl Global for DB {}

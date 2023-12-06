use serde_json::Value as JsonValue;
use sqlx::SqlitePool;
pub const VER_COLLECTION_NAME: &str = "_ver";

pub struct DndDatabase {
	pool: SqlitePool,
}

/// Generic functions
impl DndDatabase {
	pub async fn new() -> Result<DndDatabase, sqlx::Error> {
		let pool = SqlitePool::connect("sqlite::memory:").await?;
		sqlx::migrate!().run(&pool).await?;
		Ok(Self { pool })
	}

	pub async fn save_collection(
		&self,
		collection: &str,
		values: &Vec<JsonValue>,
	) -> Result<(), sqlx::Error> {
		info!("Saving {}, {}", collection, values.len());
		let mut tr = self.pool.begin().await?;
		sqlx::query("DROP TABLE IF EXISTS ?1")
			.bind(collection)
			.execute(&mut *tr)
			.await?;
		sqlx::query(
			"CREATE TABLE ?1 (
			id ROWID,
			data JSON
		)",
		)
		.bind(collection)
		.execute(&mut *tr)
		.await?;

		for (i, item) in values.iter().enumerate() {
			// TODO: Cache
			sqlx::query("INSERT INTO ?1 (id, data) VALUES (?2, json(?3))")
				.bind(collection)
				.bind(i as u32)
				.bind(item)
				.execute(&mut *tr)
				.await?;
		}
		tr.commit().await?;
		// TODO: Create index
		Ok(())
	}

	pub async fn get_item(&self, collection: &str, id: u32) -> Result<JsonValue, sqlx::Error> {
		sqlx::query_as("SELECT data from ?1 WHERE id = ?2")
			.bind(collection)
			.bind(id)
			.fetch_one(&self.pool)
			.await
			.map(|(r,)| r)
	}

	pub async fn find_one_by(
		&self,
		collection: &str,
		field: &str,
		value: &str,
	) -> Result<JsonValue, sqlx::Error> {
		sqlx::query_as("SELECT data FROM ?1 WHERE data->>'$.?2' = ?3 ORDER BY id DESC LIMIT 1")
			.bind(collection)
			.bind(field)
			.bind(value)
			.fetch_one(&self.pool)
			.await
			.map(|(r,)| r)
	}

	pub async fn get_last(&self, collection: &str) -> Result<JsonValue, sqlx::Error> {
		sqlx::query_as("SELECT data FROM ?1 ORDER BY id DESC LIMIT 1")
			.bind(collection)
			.fetch_one(&self.pool)
			.await
			.map(|(r,)| r)
	}
}

/// Useful extensions
impl DndDatabase {
	pub async fn get_version(&self) -> Result<Option<String>, sqlx::Error> {
		todo!()
	}
}

# dbdemo

This is a demo of how to use [rocket](https://rocket.rs/) and [diesel](https://diesel.rs/) to build a simple web app.

## Import diesel crate

In ubuntu, you need to install sqlite3 dev package first, then install diesel_cli which support sqlite only.

```bash
sudo apt install libsqlite3-dev -y
cargo install diesel_cli --no-default-features --features sqlite
```

Create sqlite3 database file

```bash
diesel setup --database-url db.sqlite3
```

Create a migration file

```bash
diesel migration generate create_products
```

Now you can see 2 migration files (named `up.sql` and `down.sql`) in `migrations` folder, edit it to create a table.

edit `up.sql`

```sql
create table products (
  id integer primary key autoincrement not null,
  name varchar not null,
  description varchar not null,
  create_at timestamp not null default current_timestamp
);
```

edit `down.sql`

```sql
drop table products;
```

Run migration

```bash
diesel migration run --database-url db.sqlite3
```

Now you can see a table named `products` in `db.sqlite3` file and `schema.rs` file is generated.

Create `Rocket.toml` file

```toml
[global.databases]
sqlite_database = { url = "./db.sqlite3" }
```
In `main.rs`, import diesel crate and add a `diesel::SqliteConnection` type to `State` struct.

```rust
use rocket_sync_db_pools::database;

#[database("sqlite_database")]
struct DbConn(diesel::SqliteConnection);
```
Add a `DbConn` type to `main` function.

```diff
@@ -94,6 +98,7 @@ async fn main() -> Result<(), Box<dyn std::error::Error>> {
             ],
         )
         .register("/", catchers![not_found_url])
+        .attach(DbConn::fairing())
         .launch()
         .await?;
     Ok(())
```

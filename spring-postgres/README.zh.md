[![crates.io](https://img.shields.io/crates/v/spring-postgres.svg)](https://crates.io/crates/spring-postgres)
[![Documentation](https://docs.rs/spring-postgres/badge.svg)](https://docs.rs/spring-postgres)

[tokio-postgres](https://github.com/sfackler/rust-postgres)是和sqlx类似的数据库连接工具，和sqlx不同的是它只专注于实现postgresql的数据库连接。

## 依赖

```toml
spring-postgres = { version = "0.1.0" }
```

## 配置项

```toml
[postgres]
connect = "postgres://root:12341234@localhost:5432/myapp_development"  # 要连接的数据库地址
```

## 组件

配置完上述配置项后，插件会自动注册一个[`Postgres`](https://docs.rs/tokio-postgres/latest/tokio_postgres/struct.Client.html)对象。该对象包装了[`tokio_postgres::Client`](https://docs.rs/tokio-postgres/latest/tokio_postgres/struct.Client.html)。

```rust
pub struct Postgres(Arc<tokio_postgres::Client>);
```

## 提取插件注册的Component

`PgPlugin`插件为我们自动注册了一个[`Postgres`](https://docs.rs/tokio-postgres/latest/tokio_postgres/struct.Client.html)对象，我们可以使用`Component`从AppState中提取这个连接池，[`Component`](https://docs.rs/spring-web/latest/spring_web/extractor/struct.Component.html)是一个axum的[extractor](https://docs.rs/axum/latest/axum/extract/index.html)。

```rust
#[get("/postgres")]
async fn hello_postgres(Component(pg): Component<Postgres>) -> Result<impl IntoResponse> {
    let rows = pg
        .query("select version() as version", &[])
        .await
        .context("query postgresql failed")?;

    let version: String = rows[0].get("version");

    Ok(Json(version))
}
```

完整代码参考[`postgres-example`](https://github.com/spring-rs/spring-rs/tree/master/examples/postgres-example)
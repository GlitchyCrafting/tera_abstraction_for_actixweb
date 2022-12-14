# Ex.

```rust
use actix_web::{App, HttpServer, web};
use tera::{Tera, Context};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
      App::new()
        .app_data(web::Data::new(templating::make_instance()))
        .service(index)
  })
  .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

#[get("/")]
async fn index(tera_instance: web::Data<Tera>) -> Result<impl Responder, Error> {
  let mut context = Context::new();
  context.insert("a_key", "a_value");

  templating::render(tera_instance, "template.html".to_string(), context)
}
```

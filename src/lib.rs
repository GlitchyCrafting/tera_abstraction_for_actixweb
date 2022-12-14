use actix_web::{error, Error, web};
use actix_web_lab::respond::Html;
use tera::{Tera, Context};

pub fn make_instance() -> Tera {
    match Tera::new("templates/**/*.html") {
        Ok(template) => template,
        Err(error) => {
            println!("Parsing error(s): {}", error);
            ::std::process::exit(1);
        }
    }
}

pub fn render(tera_instance: web::Data<Tera>, template_name: String, context_data: Context) -> Result<Html, Error> {
    Ok(Html(tera_instance.render(&template_name, &context_data).map_err(|_| error::ErrorInternalServerError("Failed to render template"))?))
}

use lazy_static::lazy_static;
use tera::{Context, Tera};

lazy_static! {
    static ref instance: Tera = {
        let mut tera = match Tera::new("src/templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.add_raw_template("error", "<span id='url-error'>{{ error }}</span>")
            .unwrap();
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

pub fn render_template(template_name: &str, context: &Context) -> String {
    instance.render(template_name, context).unwrap()
}

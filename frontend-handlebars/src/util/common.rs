use handlebars::Handlebars;
use serde::Serialize;
use tide::{http::mime::HTML, Body, Response, StatusCode};

pub struct Tpl<'tpl> {
    pub name: String,
    pub reg: Handlebars<'tpl>,
}

impl<'tpl> Tpl<'tpl> {
    pub async fn new(rel_path: &str) -> Tpl<'tpl> {
        let tpl_name = &rel_path.replace("/", "_");
        let abs_path = format!("./templates/{}.html", rel_path);

        // create the handlebars registry
        let mut hbs_reg = Handlebars::new();
        hbs_reg.register_template_file(tpl_name, abs_path).unwrap();

        Tpl {
            name: tpl_name.to_string(),
            reg: hbs_reg,
        }
    }

    pub async fn render<T>(&self, data: &T) -> tide::Result
        where
            T: Serialize,
    {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_content_type(HTML);
        resp.set_body(Body::from_string(
            self.reg.render(&self.name, data).unwrap(),
        ));

        Ok(resp.into())
    }
}
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    pub error: Option<String>,
}

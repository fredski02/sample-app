use askama::Template;

use crate::{models::sample::Sample, web::auth::SESSION_USER_ID};

#[derive(Template)]
#[template(path = "error_500.html")]
pub struct Error500Tmpl {
    pub ctx: BaseCtx,
    pub message: String,
}

#[derive(Template)]
#[template(path = "error_404.html")]
pub struct Error404Tmpl {
    pub ctx: BaseCtx,
}

#[derive(Template)]
#[template(path = "error_403.html")]
pub struct Error403Tmpl {
    pub ctx: BaseCtx,
}

#[derive(Template)]
#[template(path = "samples_list.html")]
pub struct SamplesListTmpl {
    pub ctx: BaseCtx,
    pub samples: Vec<Sample>,
}

#[derive(Template)]
#[template(path = "sample_form.html")]
pub struct SampleFormTmpl {
    pub ctx: BaseCtx,
    pub s: Option<Sample>,
    pub action: String,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTmpl {
    pub ctx: BaseCtx,
    pub error: bool,
}

#[derive(Clone, Debug, Default)]
pub struct BaseCtx {
    pub is_authenticated: bool,
}

pub async fn base_ctx(session: &tower_sessions::Session) -> BaseCtx {
    let logged_in = session
        .get::<i64>(SESSION_USER_ID)
        .await
        .ok()
        .flatten()
        .is_some();
    BaseCtx {
        is_authenticated: logged_in,
    }
}

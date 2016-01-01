
use std::collections::HashMap;

use rusp::RuspApp;
use nickel::{Request, Response, MiddlewareResult};

pub fn index<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let mut data = HashMap::new();
    data.insert("title", "My first app");
    data.insert("host", RuspApp::get_host());
    res.render("views/web/index.html", &data)
}





use controllers::*;

use nickel::{Nickel, Router, HttpRouter};

pub fn build<'a>() -> Router {
    let mut router = Nickel::router();

    router.get("/", web_controller::index);
    router
}


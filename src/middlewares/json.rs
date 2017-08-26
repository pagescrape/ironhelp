use iron::AfterMiddleware;
use iron::prelude::*;
use iron::headers::ContentType;

pub struct Json;

impl AfterMiddleware for Json {
    fn after(&self, _req: &mut Request, mut resp: Response) -> IronResult<Response> {
        resp.headers.set(ContentType::json());
        Ok(resp)
    }
}

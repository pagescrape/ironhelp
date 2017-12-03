use iron::AfterMiddleware;
use iron::prelude::*;
use iron::headers::AccessControlAllowOrigin;

/// Middleware for injecting allowing access control origin
pub struct ACAO(pub String);

impl AfterMiddleware for ACAO {
    fn after(&self, _req: &mut Request, mut resp: Response) -> IronResult<Response> {
        resp.headers.set(
            AccessControlAllowOrigin::Value(self.0.clone()),
        );
        Ok(resp)
    }
}

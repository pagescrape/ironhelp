use iron::{AfterMiddleware, IronResult, Request, Response, IronError, Set};
use validation::ValidationError;

/// ValidationMiddleware responds with BadRequest and serialized
/// validation errors
pub struct ValidationMiddleware;

impl AfterMiddleware for ValidationMiddleware {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if let Some(e) = err.error.downcast::<ValidationError>() {
            return Ok(err.response.set(e.to_string()));
        }
        Err(err)
    }
}
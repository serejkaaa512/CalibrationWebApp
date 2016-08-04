use std::sync::Arc;

use mysql::conn::OptsBuilder;
use mysql::conn::pool::Pool;
use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use typemap::Key;
use plugin::Extensible;

pub struct MysqlMiddleware {
    pub pool: Arc<Pool>,
}

impl MysqlMiddleware {
    pub fn new(db_name: &str, user: &str, pass: &str) -> MysqlMiddleware {
        let mut builder = OptsBuilder::new();
        builder.db_name(Some(db_name)).user(Some(user)).pass(Some(pass));

        let pool = Pool::new(builder).map_err(|_| "Connection to MySQL failed!").unwrap();

        MysqlMiddleware { pool: Arc::new(pool) }
    }
}

impl Key for MysqlMiddleware {
    type Value = Arc<Pool>;
}

impl<D: 'static> Middleware<D> for MysqlMiddleware {
    fn invoke<'mw, 'conn>(&'mw self,
                          req: &mut Request<'mw, 'conn, D>,
                          res: Response<'mw, D>)
                          -> MiddlewareResult<'mw, D> {
        req.extensions_mut().insert::<MysqlMiddleware>(self.pool.clone());
        Ok(Continue(res))
    }
}

pub trait MysqlRequestExtensions {
    fn db_connection(&self) -> Arc<Pool>;
}

impl<'a, 'b> MysqlRequestExtensions for Request<'a, 'b> {
    fn db_connection(&self) -> Arc<Pool> {
        self.extensions().get::<MysqlMiddleware>().unwrap().clone()
    }
}

use std::default::Default;
use std::sync::Arc;

use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use typemap::Key;
use plugin::{Pluggable, Extensible};

pub struct MysqlMiddleware {
    pub pool: Arc<MyPool>,
}

impl MysqlMiddleware {
    pub fn new(db_name: &str, user: &str, pass: &str) -> MysqlMiddleware {
        let options = MyOpts {
                user: Some(user.into()),
                pass: Some(pass.into()),
                db_name: Some(db_name.into()),
                ..Default::default()
        };
        let pool = MyPool::new(options).unwrap();
        MysqlMiddleware {
            pool: Arc::new(pool),
        }
    }
}

impl Key for MysqlMiddleware { type Value = Arc<MyPool>; }

impl <D: 'static> Middleware<D> for MysqlMiddleware {
    fn invoke<'mw, 'conn>(&'mw self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>)
                          -> MiddlewareResult<'mw, D>  {
        req.extensions_mut().insert::<MysqlMiddleware>(self.pool.clone());
        Ok(Continue(res))
    }    
}

pub trait MysqlRequestExtensions {
    fn db_connection(&self) -> Arc<MyPool>;    
}

impl<'a, 'b> MysqlRequestExtensions for Request<'a, 'b> {
    fn db_connection(&self) -> Arc<MyPool> {
        self.extensions().get::<MysqlMiddleware>().unwrap().clone()
    }    
}

pub mod articles;
pub mod comments;
pub mod profiles;
pub mod users;

#[database("diesel_mysql_pool")]
pub struct Db(diesel::MysqlConnection);

use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;

pub trait OffsetLimit: Sized {
    fn offset_and_limit(self, offset: i64, limit: i64) -> OffsetLimited<Self>;
}

impl<T> OffsetLimit for T {
    fn offset_and_limit(self, offset: i64, limit: i64) -> OffsetLimited<Self> {
        OffsetLimited {
            query: self,
            limit,
            offset,
        }
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct OffsetLimited<T> {
    query: T,
    offset: i64,
    limit: i64,
}

impl<T> OffsetLimited<T> {
    pub fn load_and_count<'a, U>(self, conn: &mut MysqlConnection) -> QueryResult<(Vec<U>, i64)>
    where
        Self: LoadQuery<'a, MysqlConnection, (U, i64)>,
    {
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        Ok((records, total))
    }
}

impl<T: Query> Query for OffsetLimited<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<MysqlConnection> for OffsetLimited<T> {}

impl<T> QueryFragment<Mysql> for OffsetLimited<T>
where
    T: QueryFragment<Mysql>,
{
    fn walk_ast<'a>(&'a self, mut out: AstPass<'_, 'a, Mysql>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.limit)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}

use crate::models::comment::{Comment, CommentJson};
use crate::models::user::User;
use crate::schema::articles;
use crate::schema::comments;
use crate::schema::users;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = comments)]
struct NewComment<'a> {
    body: &'a str,
    author: i32,
    article: i32,
}

pub fn create(conn: &mut MysqlConnection, author: i32, slug: &str, body: &str) -> CommentJson {
    let article_id = articles::table
        .select(articles::id)
        .filter(articles::slug.eq(slug))
        .get_result(conn)
        .expect("Cannot find article id");
    let new_comment = &NewComment {
        body,
        author,
        article: article_id,
    };

    let author = users::table
        .find(author)
        .get_result::<User>(conn)
        .expect("Error loading author");
    let comment = conn.transaction::<Comment, diesel::result::Error, _>(|conn| {
        diesel::insert_into(comments::table)
        .values(new_comment)
        .execute(conn)?;
        // .get_result::<Comment>(conn)
        // .expect("Error creating comment");
        // .attach(author)
        comments::table.order(comments::id.desc()).first::<Comment>(conn)
    });
    comment.expect("error").attach(author)

}

pub fn find_by_slug(conn: &mut MysqlConnection, slug: &str) -> Vec<CommentJson> {
    let result = comments::table
        .inner_join(articles::table)
        .inner_join(users::table)
        .select((comments::all_columns, users::all_columns))
        .filter(articles::slug.eq(slug))
        .get_results::<(Comment, User)>(conn)
        .expect("Cannot load comments");

    result
        .into_iter()
        .map(|(comment, author)| comment.attach(author))
        .collect()
}

pub fn delete(conn: &mut MysqlConnection, author: i32, slug: &str, comment_id: i32) {
    use diesel::dsl::exists;
    use diesel::select;

    let belongs_to_author_result = select(exists(
        articles::table.filter(articles::slug.eq(slug).and(articles::author.eq(author))),
    ))
    .get_result::<bool>(conn);

    if let Err(err) = belongs_to_author_result {
        match err {
            diesel::result::Error::NotFound => return,
            _ => panic!("Cannot find article by author: {}", err),
        }
    }

    let result = diesel::delete(comments::table.filter(comments::id.eq(comment_id))).execute(conn);
    if let Err(err) = result {
        eprintln!("comments::delete: {}", err);
    }
}

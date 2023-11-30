
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    realworld::rocket().launch().await?;
    Ok(())
}
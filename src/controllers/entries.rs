use crate::accounting::{Account, AccountRepository, Repository};
use actix_web::{get, post, web, HttpResponse, Scope};

pub fn services() -> Scope {
    web::scope("/entries").service(index).service(create)
}

#[get("")]
async fn index() -> HttpResponse {
    let repository = AccountRepository::from_file();
    let account = repository.find_all();
    HttpResponse::Ok().json(account)
}

#[post("")]
async fn create(account_json: web::Json<Account>) -> HttpResponse {
    let mut repository = AccountRepository::from_file();
    let account = account_json.0;
    repository.add(account);
    HttpResponse::Ok().json(repository.find_all())
}

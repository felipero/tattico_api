use actix_web::{get, post, web, HttpResponse, Scope};
use crate::accounting::{Account, AccountRepository, Repository};

pub fn services() -> Scope {
    web::scope("/entries").service(index).service(create)
}

// fn build_account(id: String) -> Account {
//     let now = Utc::now();
//
//     let entries = vec![
//         Entry {
//             value: Decimal::new(202, 2),
//             description: Some("Online course".to_string()),
//             date: now,
//             category: None,
//             account: None,
//         },
//         Entry {
//             value: Decimal::new(345, 2),
//             description: Some("Yoga class".to_string()),
//             date: now,
//             category: None,
//             account: None,
//         },
//     ];
//
//     let category = Category {
//         name: "food".to_string(),
//         entries,
//     };
//     Account {
//         id,
//         name: "some_account".to_string(),
//         kind: AccountType::Checking,
//         categories: vec![category],
//     }
// }

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

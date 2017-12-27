use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use schema::transactions;
use models::form_values::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub date: NaiveDate,
    pub account: Uuid,
    pub category: Option<Uuid>,
    pub payee: Option<Uuid>,
    pub parent_transaction: Option<Uuid>,
    pub amount: BigDecimal,
    pub memo: Option<String>,
    pub cleared: bool,
}

#[derive(Insertable)]
#[table_name="transactions"]
pub struct NewTransaction<'a> {
    pub date: NaiveDate,
    pub account: Uuid,
    pub category: Option<Uuid>,
    pub payee: Option<Uuid>,
    pub parent_transaction: Option<Uuid>,
    pub amount: BigDecimal,
    pub memo: Option<&'a str>,
    pub cleared: bool,
}

#[derive(FromForm)]
pub struct FormTransaction {
    pub date: FormDate,
    pub account: FormUuid,
    pub category: Option<FormUuid>,
    pub payee: Option<FormUuid>,
    pub parent_transaction: Option<FormUuid>,
    pub amount: FormDecimal,
    pub memo: Option<String>,
    pub cleared: bool,
}

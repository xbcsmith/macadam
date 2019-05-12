use super::schema::receipts;

#[derive(Queryable)]
pub struct Receipt {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub issued: bool,
}

#[derive(Insertable)]
#[table_name = "receipts"]
pub struct NewReceipt<'a> {
    pub name: &'a str,
    pub status: &'a str,
}

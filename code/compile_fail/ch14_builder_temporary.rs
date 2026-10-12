// harness: compile-fail E0716
//! A `&mut self -> &mut Self` builder returns a borrow of the builder; binding
//! that borrow to a variable keeps it alive after the temporary builder is gone.

pub struct QueryBuilder {
    table: String,
    filters: Vec<String>,
}

impl QueryBuilder {
    pub fn new(table: &str) -> Self {
        QueryBuilder {
            table: table.to_owned(),
            filters: Vec::new(),
        }
    }

    pub fn filter(&mut self, condition: &str) -> &mut Self {
        self.filters.push(condition.to_owned());
        self
    }

    pub fn build(&self) -> String {
        format!(
            "SELECT * FROM {} WHERE {}",
            self.table,
            self.filters.join(" AND ")
        )
    }
}

// ANCHOR: temporary
pub fn caller() -> String {
    let query = QueryBuilder::new("users").filter("age > 30");
    query.filter("active");
    query.build()
}
// ANCHOR_END: temporary

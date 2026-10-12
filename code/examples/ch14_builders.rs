// harness: run
//! Two builder styles: by value (`self -> Self`), which can be chained and
//! stored freely, and by mutable reference (`&mut self -> &mut Self`), which
//! must be bound to a variable before it is configured in several statements.
//!
//! Run with: `cargo run --example ch14_builders`

struct Request {
    url: String,
    headers: Vec<(String, String)>,
    timeout_ms: u64,
}

// ANCHOR: by_value
/// Each method takes the builder by value and returns it.
struct RequestBuilder {
    request: Request,
}

impl RequestBuilder {
    fn new(url: impl Into<String>) -> Self {
        let request = Request {
            url: url.into(),
            headers: Vec::new(),
            timeout_ms: 30_000,
        };
        RequestBuilder { request }
    }

    fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.request.headers.push((name.into(), value.into()));
        self
    }

    fn timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.request.timeout_ms = timeout_ms;
        self
    }

    fn build(self) -> Request {
        self.request
    }
}
// ANCHOR_END: by_value

// ANCHOR: by_ref
/// Each method borrows the builder mutably and returns the same borrow.
struct QueryBuilder {
    table: String,
    filters: Vec<String>,
}

impl QueryBuilder {
    fn new(table: impl Into<String>) -> Self {
        QueryBuilder {
            table: table.into(),
            filters: Vec::new(),
        }
    }

    fn filter(&mut self, condition: impl Into<String>) -> &mut Self {
        self.filters.push(condition.into());
        self
    }

    fn build(&self) -> String {
        if self.filters.is_empty() {
            return format!("SELECT * FROM {}", self.table);
        }
        format!(
            "SELECT * FROM {} WHERE {}",
            self.table,
            self.filters.join(" AND ")
        )
    }
}
// ANCHOR_END: by_ref

fn main() {
    // ANCHOR: main_by_value
    let builder = RequestBuilder::new("https://example.com/items").timeout_ms(500);
    let builder = builder.header("accept", "text/plain"); // stored, then continued
    let request = builder.build();
    println!(
        "{} {:?} {}ms",
        request.url, request.headers, request.timeout_ms
    );
    // ANCHOR_END: main_by_value

    // ANCHOR: main_by_ref
    // One statement: the temporary builder lives until the end of the statement.
    let all = QueryBuilder::new("users").filter("age > 30").build();
    println!("{all}");

    // Several statements: bind the builder to a variable first.
    let only_active = true; // imagine a command-line flag
    let mut query = QueryBuilder::new("users");
    query.filter("age > 30");
    if only_active {
        query.filter("active");
    }
    println!("{}", query.build());
    // ANCHOR_END: main_by_ref
}

use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let date_time = start.checked_add(Duration::seconds(1_000_000_000));
    date_time.unwrap()
}

use time::{PrimitiveDateTime, UtcOffset, OffsetDateTime};
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    /** Solution 1 */
    // let mut date_in_unix = start.assume_offset(UtcOffset::UTC).unix_timestamp();
    
    // date_in_unix += 1_000_000_000;

    // let datetime = OffsetDateTime::from_unix_timestamp(date_in_unix)
    // .expect("Invalid timestamp")
    // .to_offset(time::UtcOffset::UTC);

    // return PrimitiveDateTime::new(datetime.date(), datetime.time());

    // the fucking fast solution, PrimitiveDateTime impl add syntax 
    return start + Duration::seconds(1_000_000_000)
}

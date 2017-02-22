use time::Duration;

pub fn seconds(span: Duration) -> String {
    match span.num_seconds() {
        0 => match span.num_milliseconds() {
            0 => match span.num_microseconds().unwrap_or(-1) {
                -1 => unreachable!(), // it would be caught by an earlier branch
                0 => match span.num_nanoseconds().unwrap_or(-1) {
                    -1 => unreachable!(), // idem
                    n @ _ => format!("{}ns", n)
                },
                u @ _ => format!("{}μs", u)
            },
            m @ _ => format!("{}ms", m)
        },
        s @ _ => format!("{}s", s)
    }
}

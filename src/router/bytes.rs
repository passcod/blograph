pub fn bytes(n: usize) -> String {
    match n {
        b @ 0 ... 999 => format!("{} bytes", b),
        k @ 1_000 ... 1_048_575 => format!("{:.2} KiB", k as f64 / 1024.0),
        m @ _ => format!("{:.2} MiB", m as f64 / 1048576.0)
    }
}

use core::fmt;

struct Report;

impl fmt::Debug for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Report")
    }
}

fn main() -> Result<(), Report> {
    Ok(())
}

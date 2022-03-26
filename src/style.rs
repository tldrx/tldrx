
#[derive(Debug)]
pub(crate) struct Style {
    pub enable: bool,
}

impl Style {
    pub fn eprintln<T: Display>(&self, s: T) {
        eprintln!();
    }
}
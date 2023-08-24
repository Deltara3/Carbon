/* Module metadata information */
pub struct Metadata {
    pub name: &'static str,
    pub version: [usize; 3],
    pub authors: Vec<&'static str>,
    pub consoles: Vec<&'static str>
}
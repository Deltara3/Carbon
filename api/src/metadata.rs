pub struct Metadata {
    pub name: &'static str,
    pub consoles: Vec<&'static str>,
    pub authors: Vec<&'static str>,
    pub version: [usize; 3],
    pub extensions: Vec<&'static str>
}
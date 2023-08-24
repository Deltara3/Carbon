/* List of status responses */
#[derive(PartialEq)]
pub enum Status<'a> {
    Cycle(usize),
    Success,
    Failure(&'a str)
}
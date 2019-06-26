extern crate nom;

mod fields;
mod messages;
pub mod sentence;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

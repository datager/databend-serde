#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[cfg(feature = "d")]
pub mod serde_derive;

#[cfg(feature = "s")]
pub mod serde_std;

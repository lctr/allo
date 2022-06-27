mod ipa;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        println!("{:?}", ipa::LAT_FRICATIVES)
    }
}

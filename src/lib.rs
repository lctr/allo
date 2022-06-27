pub mod graphemes;
pub mod ipa;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lateral_fricatives() {
        let voiceless_lateral_fricative = r"ɬ";
        let voiced_lateral_fricative = r"ɮ";
        assert!(voiceless_lateral_fricative == graphemes::LAT_FRICATIVES[0]);
        assert!(voiced_lateral_fricative == graphemes::LAT_FRICATIVES[1]);
    }
}

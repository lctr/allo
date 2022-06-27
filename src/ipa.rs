#![allow(unused)]

// use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag(u32);

/// Place of articulation at its most general. These are the basis
/// for the columns in the IPA table for (pulmonary) consonants.
///
/// **Note** that the consonants table has *two* column schemes,
/// both of which can be used in place of each other*:
/// * Place
/// * Articulation
///
/// However, note that the only `Place` variant that maps in a 1-1
/// manner with `Articulation` is the `Laryngeal` variant since
/// `Laryngeal` contains the three `Articulation` variants
/// `Pharyngeal`, `Epiglottal`, and `Glottal`. Additionally, the
/// `Pharyngeal` and `Epiglottal` variants take up *the same
/// column*, but this detail is still to be hashed out. For the
/// rest, a given pair of consonants with the same `Articulation`
/// may correspond to *distinct* `Place` variants.
///
/// For example, the consonants `/ç ʝ/` both reside under the
/// column `Palatal`, and thus have `Articulation::Palatal`.
/// On the other hand, /ç/ has a place variant `Place::Coronal`,
/// while `/ʝ/` has `Place::Dorsal`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Place {
    Labial,
    Corona,
    Dorsal,
    Laryngeal,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Corresponds to a column containing consonant pairs, which are
/// further differentiated based on `Voicing`
pub enum Articulation {
    Bilabial,
    Labiodental,
    Linguolabial,
    Dental,
    Alveolar,
    Postalveolar,
    Retroflex,
    Palatal,
    Velar,
    Uvular,
    // TODO: maybe merge pharyngeal and epiglottal since they are the same column in the IPA table?
    Pharyngeal,
    Epiglottal,
    Glottal,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PoA {
    place: Place,
    articulation: Articulation,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Correspond to the rows in the IPA table for (pulmonic)
/// consonants.
pub enum Manner {
    Nasal,
    Plosive,
    Fricative { sibilant: bool },
    Approximant,
    TapFlap,
    Trill,
    // Affricate,
    LatFric,
    LatApprox,
    LatTapFlap,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phonation {
    Voiced,
    Voiceless,
}

/// Graphemes: m̥ m ɱ̊ ɱ n̪̊ n̪ n̥ n ɲ̊ ɲ ŋ̊ ŋ ɴ̥ ɴ
pub const NASALS: [&'static str; 14] = [
    "\u{6D}", "\u{6D}", "\u{271}", "\u{271}", "\u{6E}", "\u{6E}", "\u{6E}", "\u{6E}", "\u{272}",
    "\u{272}", "\u{14B}", "\u{14B}", "\u{274}", "\u{274}",
];

/// Graphemes: p b p̪ b̪ t̪ d̪ t d ʈ ɖ c ɟ k ɡ q ɢ ʡ ʔ
pub const PLOSIVES: [&'static str; 18] = [
    "\u{70}", "\u{62}", "\u{70}", "\u{62}", "\u{74}", "\u{64}", "\u{74}", "\u{64}", "\u{288}",
    "\u{256}", "\u{63}", "\u{25F}", "\u{6B}", "\u{261}", "\u{71}", "\u{262}", "\u{2A1}", "\u{294}",
];

/// Graphemes: ʙ r̥  r ɽ͡r ʀ̥  ʀ ᴙ
pub const TRILLS: [&'static str; 7] = [
    "\u{299}", "\u{72}", "\u{72}", "\u{27D}", "\u{280}", "\u{280}", "\u{1D19}",
];

/// Graphemes: ⱱ̟ ⱱ ɾ̥ ɾ ɽ
pub const TAPS: [&'static str; 5] = ["\u{2C71}", "\u{2C71}", "\u{27E}", "\u{27E}", "\u{27D}"];

/// Graphemes: ɸ β f v θ ð s z ʃ ʒ ɕ ʑ ʂ ʐ ç ʝ x ɣ χ ʁ ħ ʕ ʜ ʢ h ɦ
pub const FRICATIVES: [&'static str; 26] = [
    "\u{278}", "\u{3B2}", "\u{66}", "\u{76}", "\u{3B8}", "\u{F0}", "\u{73}", "\u{7A}", "\u{283}",
    "\u{292}", "\u{255}", "\u{291}", "\u{282}", "\u{290}", "\u{E7}", "\u{29D}", "\u{78}",
    "\u{263}", "\u{3C7}", "\u{281}", "\u{127}", "\u{295}", "\u{29C}", "\u{2A2}", "\u{68}",
    "\u{266}",
];

/// Graphemes: ɬ  ɮ
pub const LAT_FRICATIVES: [&'static str; 2] = ["\u{26C}", "\u{26E}"];

/// Graphemes: l ɭ ʎ ʟ
pub const LAT_APPROX: [&'static str; 4] = ["\u{6C}", "\u{26D}", "\u{28E}", "\u{29F}"];

/// Graphemes: ʋ ɹ ɻ j̊  j ɰ
pub const APPROX: [&'static str; 6] = [
    "\u{28B}", "\u{279}", "\u{27B}", "\u{6A}", "\u{6A}", "\u{270}",
];

/// Graphemes: pf bv p̪f b̪v tθ dð ts dz tʃ dʒ tɕ dʑ ʈʂ ɖʐ cç cç kx ɡɣ qχ ɢʁ
pub const AFFRICATES: [&'static str; 20] = [
    "\u{70}", "\u{62}", "\u{70}", "\u{62}", "\u{74}", "\u{64}", "\u{74}", "\u{64}", "\u{74}",
    "\u{64}", "\u{74}", "\u{64}", "\u{288}", "\u{256}", "\u{63}", "\u{63}", "\u{6B}", "\u{261}",
    "\u{71}", "\u{262}",
];

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

/// Combinator struct holding both `Place` and `Articulation`, since many IPA
/// tables so graciously mixe the two so often.
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

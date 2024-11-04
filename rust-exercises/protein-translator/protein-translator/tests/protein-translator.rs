use protein_translator::ProteinTranslator;

#[test]
fn test_methionine_rna_sequence() {
    let expected = vec!["Methionine".to_string()];
    assert_eq!(ProteinTranslator::translate("AUG"), expected);
}

#[test]
fn test_phenylalanine_rna_sequence1() {
    let expected = vec!["Phenylalanine".to_string()];
    assert_eq!(ProteinTranslator::translate("UUU"), expected);
}

#[test]
fn test_phenylalanine_rna_sequence2() {
    let expected = vec!["Phenylalanine".to_string()];
    assert_eq!(ProteinTranslator::translate("UUC"), expected);
}

#[test]
fn test_leucine_rna_sequence1() {
    let expected = vec!["Leucine".to_string()];
    assert_eq!(ProteinTranslator::translate("UUA"), expected);
}

#[test]
fn test_leucine_rna_sequence2() {
    let expected = vec!["Leucine".to_string()];
    assert_eq!(ProteinTranslator::translate("UUG"), expected);
}

#[test]
fn test_serine_rna_sequence1() {
    let expected = vec!["Serine".to_string()];
    assert_eq!(ProteinTranslator::translate("UCU"), expected);
}

#[test]
fn test_serine_rna_sequence2() {
    let expected = vec!["Serine".to_string()];
    assert_eq!(ProteinTranslator::translate("UCC"), expected);
}

#[test]
fn test_serine_rna_sequence3() {
    let expected = vec!["Serine".to_string()];
    assert_eq!(ProteinTranslator::translate("UCA"), expected);
}

#[test]
fn test_serine_rna_sequence4() {
    let expected = vec!["Serine".to_string()];
    assert_eq!(ProteinTranslator::translate("UCG"), expected);
}

#[test]
fn test_tyrosine_rna_sequence1() {
    let expected = vec!["Tyrosine".to_string()];
    assert_eq!(ProteinTranslator::translate("UAU"), expected);
}

#[test]
fn test_tyrosine_rna_sequence2() {
    let expected = vec!["Tyrosine".to_string()];
    assert_eq!(ProteinTranslator::translate("UAC"), expected);
}

#[test]
fn test_cysteine_rna_sequence1() {
    let expected = vec!["Cysteine".to_string()];
    assert_eq!(ProteinTranslator::translate("UGU"), expected);
}

#[test]
fn test_cysteine_rna_sequence2() {
    let expected = vec!["Cysteine".to_string()];
    assert_eq!(ProteinTranslator::translate("UGC"), expected);
}

#[test]
fn test_tryptophan_rna_sequence1() {
    let expected = vec!["Tryptophan".to_string()];
    assert_eq!(ProteinTranslator::translate("UGG"), expected);
}

#[test]
fn test_stop_rna_sequence1() {
    let expected: Vec<String> = vec![];
    assert_eq!(ProteinTranslator::translate("UAA"), expected);
}

#[test]
fn test_stop_rna_sequence2() {
    let expected: Vec<String> = vec![];
    assert_eq!(ProteinTranslator::translate("UAG"), expected);
}

#[test]
fn test_stop_rna_sequence3() {
    let expected: Vec<String> = vec![];
    assert_eq!(ProteinTranslator::translate("UGA"), expected);
}

#[test]
fn test_translation_of_rna_to_protein_list() {
    let expected = vec!["Methionine".to_string(), "Phenylalanine".to_string(), "Tryptophan".to_string()];
    assert_eq!(ProteinTranslator::translate("AUGUUUUGG"), expected);
}

#[test]
fn test_translation_stops_if_stop_codon_at_beginning() {
    let expected: Vec<String> = vec![];
    assert_eq!(ProteinTranslator::translate("UAGUGG"), expected);
}

#[test]
fn test_translation_stops_if_stop_codon_at_end1() {
    let expected = vec!["Tryptophan".to_string()];
    assert_eq!(ProteinTranslator::translate("UGGUAG"), expected);
}

#[test]
fn test_translation_stops_if_stop_codon_at_end2() {
    let expected = vec!["Methionine".to_string(), "Phenylalanine".to_string()];
    assert_eq!(ProteinTranslator::translate("AUGUUUUAA"), expected);
}

#[test]
fn test_translation_stops_if_stop_codon_in_middle1() {
    let expected = vec!["Tryptophan".to_string()];
    assert_eq!(ProteinTranslator::translate("UGGUAGUGG"), expected);
}

#[test]
fn test_translation_stops_if_stop_codon_in_middle2() {
    let expected = vec!["Tryptophan".to_string(), "Cysteine".to_string(), "Tyrosine".to_string()];
    assert_eq!(ProteinTranslator::translate("UGGUGUUAUUAAUGGUUU"), expected);
}
pub struct ProteinTranslator;

impl ProteinTranslator {
    pub fn old_translate(rna_sequence: &str) -> Vec<String> {
        let mut vec = vec![];
        let mut index = 0;
        while rna_sequence.len() > index * 3 {
            let protein = Self::protein_for(&rna_sequence[index*3..index*3+3]);
            if protein == "STOP" {
                break;
            }
            vec.push(protein);
            index += 1;
        }
        vec
    }

    pub fn translate(rna_sequence: &str) -> Vec<String> {
        (0..rna_sequence.len())
            .step_by(3)
            .map(|i| &rna_sequence[i..i + 3]).fold(vec![], |mut result, codon| {
            result.push(Self::protein_for(codon));
            result
        }).into_iter().take_while(|protein| protein != "STOP").collect()
    }
    fn protein_for(codon: &str) -> String {
        match codon {
            "AUG" => "Methionine".to_string(),
            "UUU" | "UUC" => "Phenylalanine".to_string(),
            "UUA" | "UUG" => "Leucine".to_string(),
            "UCU" | "UCC" | "UCA" | "UCG" => "Serine".to_string(),
            "UAU" | "UAC" => "Tyrosine".to_string(),
            "UGU" | "UGC" => "Cysteine".to_string(),
            "UGG" => "Tryptophan".to_string(),
            _ => "STOP".to_string()
        }
    }
}
use diff_match_patch::Patch;

#[cfg(feature = "serde_support")]
#[test]
pub fn serde_support(){
    let mut dmp = diff_match_patch::Dmp::new();

    let text1 = "The quick brown fox jumps over the lazy dog.";
    let text2 = "That quick brown fox jumped over a lazy dog.";

    let original_patches = dmp.patch_make1(text2, text1);

    let serialized_patches = serde_json::to_string(&original_patches).unwrap();

    let desrialized_patches: Vec<Patch> = serde_json::from_str(&serialized_patches).unwrap();
    
    assert_eq!(original_patches,desrialized_patches);
} 
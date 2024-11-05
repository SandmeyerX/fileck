use std::path::Path;
use std::fs;
use std::io::ErrorKind;

use fileck::helper::{
    Algorithms, Fileck
};


fn read_checklist(target_dir: &Path) -> String {
    // XXX Unsafe code here
    let checklist = target_dir.join("checklist.txt");
    fs::read_to_string(checklist).unwrap()
}

#[test]
fn test_generate_by_md5() {
    let target_dir = Path::new("tests/assets/target");
    let ignore_files = vec!["checklist.txt"];
    
    let app = Fileck::new(
        target_dir, 
        Algorithms::MD5, 
        ignore_files
    );
    assert!(app.gen_checklist().is_ok());

    let content = read_checklist(target_dir);
    assert_eq!(
        content, concat!(
        "001.target;19fc8eff82037f1fc0d8ea1d32b5e339\n",
        "002.target;f9752486885125084e3b54aba073f96b\n",
        "003.target;2b984c8a534efb23e338e55472f69ca8\n")
    );
}

#[test]
fn test_generate_by_sha1() {
    let target_dir = Path::new("tests/assets/target");
    let ignore_files = vec!["checklist.txt"];
    
    let app = Fileck::new(
        target_dir, 
        Algorithms::SHA1, 
        ignore_files
    );
    assert!(app.gen_checklist().is_ok());

    let content = read_checklist(target_dir);
    assert_eq!(
        content, concat!(
        "001.target;eae3734d9ea7394fcbee92adb05994b5083827fc\n",
        "002.target;87ea3084a8ecf4630decab4646ae3c7af95e5d6f\n",
        "003.target;8ff1fb4254f5c0e78b85bf0acf69420eaa0f5108\n")
    );
}

#[test]
fn test_handle_non_existent_target() {
    let target_dir = "non-existent-dir/nice-sandmeyer/";
    let ignore_files = vec!["checklist.txt"];
    
    let app = Fileck::new(
        target_dir, 
        Algorithms::SHA1, 
        ignore_files
    );
    assert!(
        app.gen_checklist().is_err_and(
            |e| e.kind() == ErrorKind::NotFound
    ));
}

// TODO  Add more tests
// Special filename, target is a file...

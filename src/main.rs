mod cli;
mod helper;


fn main() {
    let dir = "statics/";
    let ignore_files = vec!["checklist.txt"];
    let app = helper::Fileck::new(
        dir, 
        helper::Algorithms::SHA1, 
        ignore_files
    );
    println!("{:?}", app.gen_checklist());
}

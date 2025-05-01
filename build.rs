
fn main() {
    let mut contents = "".to_string();
    let targets = vec!["33rem", "47rem", "60rem"];
    for i in 1..9 {
        contents += format!("col-span-{}\n", i).as_str();
        contents += format!("grid-cols-{}\n", i).as_str();
        for target in targets.iter().cloned() {
            contents += format!("min-[{}]:col-span-{}\n", target, i).as_str();
            contents += format!("min-[{}]:grid-cols-{}\n", target, i).as_str();
        }
    }

    std::fs::create_dir_all("assets").ok();
    std::fs::write("assets/extra.txt", contents).expect("Something wrong");
}
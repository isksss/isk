use dirs;

// dotfilesを持ってくる
pub fn clone_dotfiles(repository: &String) {
    let github_path = "https://github.com/".to_string();
    let repository = format!("{}{}.git", github_path, repository);
    let dotfiles_path = dirs::home_dir().unwrap().join("dotfiles");
    println!("dotfiles: {}", &dotfiles_path.display());
    println!("repository: {}", &repository);

    // dotfilesがなければcloneする
    if !dotfiles_path.exists() {
        println!("dotfilesがないのでcloneします");
        let _ = std::process::Command::new("git")
            .args(["clone", &repository, dotfiles_path.to_str().unwrap()])
            .output()
            .expect("failed to execute process");
    } else {
        println!("dotfilesがあるのでcloneしません");
    }
}

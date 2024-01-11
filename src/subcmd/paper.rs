use serde::{Deserialize, Serialize};
use toml;
use std::fs;
use std::io::Write;
use reqwest;
use reqwest::blocking::Response;

// グローバル変数
pub static PAPER_TOML: &str = "paper.toml";
pub static PAPER_URL: &str = "https://api.papermc.io/v2/projects";
static PAPER_PLUGIN_DIR: &str = "plugins";

// 設定ファイルの構造体
#[derive(Serialize,Deserialize)]
struct Config {
   server: Server,
   plugins: Plugins,
}

#[derive(Serialize,Deserialize)]
struct Server {
    project: String,
    version: Option<String>,
    file: Option<String>,
}

#[derive(Serialize,Deserialize)]
struct Plugins {
    enable: bool,
    plugin: Vec<Plugin>,
}

#[derive(Serialize, Deserialize)]
struct Plugin {
    name: String,
    url: String,
}

// paper apiのレスポンス
#[derive(Serialize, Deserialize, Debug)]
struct Project {
    project_id: String,
    project_name: String,
    version_groups: Vec<String>,
    versions: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct ProjectVersion {
    project_id: String,
    project_name: String,
    version: String,
    builds: Vec<u32>,
}

// サブコマンドの実装
pub fn download() {
    if config_exists() {
        let current_dir = std::env::current_dir().unwrap();
        let paper_toml = current_dir.join(PAPER_TOML);
        let conf_b = fs::read_to_string(paper_toml).unwrap();
        let conf: Option<Config> = toml::from_str(&conf_b).unwrap();
        match conf {
            Some(c) => {
                show_config(&c);
                // ダウンロード
                match paper_download(&c) {
                    Ok(_) => {},
                    Err(e) => println!("{}", e),
                }
                // プラグインのダウンロード
                if c.plugins.enable {
                    match plugin_download(&c) {
                        Ok(_) => {},
                        Err(e) => println!("{}", e),
                    }
                }else{
                    println!("プラグインのダウンロードをスキップしました");
                }
            },
            None => println!("パースに失敗しました"),
        }
    } else {
        println!("paper.tomlがありません");
        println!("paper.tomlを作成してください => isk paper init")
    }
}

pub fn create_config() {
    if config_exists() {
        println!("paper.tomlがすでにあります");
    } else {
        println!("paper.tomlを作成します");
        let conf = Config {
            server: Server {
                project: "paper".to_string(),
                version: Some("1.20.2".to_string()),
                file: Some("server.jar".to_string()),
            },
            plugins: Plugins {
                enable: false,
                plugin: vec![
                    Plugin {
                        name: "".to_string(),
                        url: "".to_string(),
                    },
                ],
            },
        };
        let toml = toml::to_string(&conf).unwrap();
        println!("{}", toml);
        std::fs::write(PAPER_TOML, toml).unwrap();
    }
}

// ヘルパー関数
fn config_exists() -> bool {
    let current_dir = std::env::current_dir().unwrap();
    let paper_toml = current_dir.join(PAPER_TOML);
    paper_toml.exists()
}

fn show_config(conf: &Config){
    println!("===== ===== ===== ===== =====");
    println!("project: {}", conf.server.project);
    match &conf.server.version {
        Some(v) => println!("version: {}", v),
        None => println!("version: latest"),
    }
    if conf.plugins.enable {
        println!("plugins: enable");
        for p in &conf.plugins.plugin {
            println!("  name: {}", p.name);
            println!("  url: {}", p.url);
        }
    } else {
        println!("plugins: disable");
    }
    println!("===== ===== ===== ===== =====");
}

// ダウンロード
fn paper_download(conf: &Config) -> Result<(), String> {
    // projectが存在するか確認
    let url = format!("{}/{}", PAPER_URL, conf.server.project);
    let res: Response = reqwest::blocking::get(&url).unwrap();
    if res.status().is_success() {
        let project: Project = res.json().unwrap();
        println!("project: {}", project.project_name);
        // versionが存在するか確認
        let version_len = project.versions.len();
        let version = match &conf.server.version {
            Some(v) => v,
            None => &project.versions[version_len - 1],
        };
        if project.versions.contains(version) {
            // ビルド番号を取得
            let url = format!("{}/{}/versions/{}", PAPER_URL, conf.server.project, version);
            let res: Response = reqwest::blocking::get(&url).unwrap();
            if res.status().is_success() {
                let project_version: ProjectVersion = res.json().unwrap();
                let build = project_version.builds[0];
                println!("version: {}", project_version.version);
                println!("build: {}", build);
                // ダウンロード
                let jar= format!("{}-{}-{}.jar", conf.server.project, project_version.version, build);
                let url = format!("{}/{}/versions/{}/builds/{}/downloads/{}", PAPER_URL, conf.server.project, version, build, jar);
                let mut res: Response = reqwest::blocking::get(&url).unwrap();
                if res.status().is_success() {
                    let filename = match &conf.server.file {
                        Some(f) => f,
                        None => &jar,
                    };
                    let mut file = std::fs::File::create(filename).unwrap();
                    let buf: Vec<u8> = vec![];
                    // ダウンロードしたファイルを書き込む
                    res.copy_to(&mut file).unwrap();
                    // ファイルに書き込む
                    file.write_all(&buf).unwrap();

                    println!("ダウンロードが完了しました");
                } else {
                    println!("ダウンロードに失敗しました");
                    let _ = Err::<(), String>("ダウンロードに失敗しました".to_string());
                }
            } else {
                println!("versionが存在しません");
                let _ = Err::<(), String>("versionが存在しません".to_string());
            }

        } else {
            println!("versionが存在しません");
            let _ = Err::<(), String>("versionが存在しません".to_string());
        }
    } else {
        println!("projectが存在しません");
        let _ = Err::<(), String>("projectが存在しません".to_string());
    }

    Ok(())
}

fn plugin_download(conf: &Config) -> Result<(), String> {
    let current_dir = std::env::current_dir().unwrap();
    let plugins_dir = current_dir.join(PAPER_PLUGIN_DIR);
    if !plugins_dir.exists() {
        std::fs::create_dir(PAPER_PLUGIN_DIR).unwrap();
    }

    for p in &conf.plugins.plugin {
        let url = &p.url;
        let mut res: Response = reqwest::blocking::get(url).unwrap();
        if res.status().is_success() {
            let filename = format!("{}/{}", PAPER_PLUGIN_DIR, &p.name);
            let mut file = std::fs::File::create(filename).unwrap();
            let buf: Vec<u8> = vec![];
            // ダウンロードしたファイルを書き込む
            res.copy_to(&mut file).unwrap();
            // ファイルに書き込む
            file.write_all(&buf).unwrap();
            println!("{}をダウンロードしました", &p.name);
        } else {
            println!("{}のダウンロードに失敗しました", &p.name);
        }
    }
    Ok(())
}

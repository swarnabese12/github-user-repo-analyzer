use clap::Parser;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

/// GitHub Repo/User Analyzer CLI
#[derive(Parser)]
struct Args {
    /// GitHub repo or username (e.g., rust-lang/rust or swarnabese12)
    target: String,

    /// Optional output file (e.g., report.json)
    #[arg(short, long)]
    output: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RepoData {
    name: String,
    description: Option<String>,
    stargazers_count: u32,
    forks_count: u32,
    open_issues_count: u32,
    watchers_count: u32,
    language: Option<String>,
    license: Option<License>,
    html_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct License {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserData {
    login: String,
    name: Option<String>,
    company: Option<String>,
    location: Option<String>,
    public_repos: u32,
    followers: u32,
    following: u32,
    created_at: String,
    html_url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();

    if args.target.contains('/') {
        // Repo analysis
        let url = format!("https://api.github.com/repos/{}", args.target);
        let resp = client
            .get(&url)
            .header("User-Agent", "github-repo-analyzer")
            .send()?
            .json::<RepoData>()?;

        println!("🔍 Analyzing GitHub repo: {}\n", args.target);
        println!("📄 Name       : {}", resp.name);
        println!(
            "💬 Description: {}",
            resp.description.as_ref().unwrap_or(&"".to_string())
        );
        println!("⭐ Stars      : {}", resp.stargazers_count);
        println!("🍴 Forks      : {}", resp.forks_count);
        println!("👀 Watchers   : {}", resp.watchers_count);
        println!("🐛 Issues     : {}", resp.open_issues_count);
        println!(
            "📝 License    : {}",
            resp.license
                .as_ref()
                .map(|l| &l.name)
                .unwrap_or(&"N/A".to_string())
        );
        println!(
            "💻 Language   : {}",
            resp.language.as_ref().unwrap_or(&"Unknown".to_string())
        );
        println!("🔗 URL        : {}", resp.html_url);

        if let Some(file) = args.output {
            let json = serde_json::to_string_pretty(&resp)?;
            let mut f = File::create(&file)?;
            f.write_all(json.as_bytes())?;
            println!("\n✅ Repo output saved to '{}'", file);
        }
    } else {
        // User analysis
        let url = format!("https://api.github.com/users/{}", args.target);
        let user = client
            .get(&url)
            .header("User-Agent", "github-repo-analyzer")
            .send()?
            .json::<UserData>()?;

        println!("👤 GitHub User: {}\n", args.target);
        println!("🧑 Name       : {}", user.name.as_deref().unwrap_or("N/A"));
        println!("🏢 Company    : {}", user.company.as_deref().unwrap_or("N/A"));
        println!("📍 Location   : {}", user.location.as_deref().unwrap_or("N/A"));
        println!("📦 Public Repos: {}", user.public_repos);
        println!("👥 Followers  : {}", user.followers);
        println!("🤝 Following  : {}", user.following);
        println!("🗓️  Joined     : {}", user.created_at);
        println!("🔗 URL        : {}", user.html_url);

        if let Some(file) = args.output {
            let json = serde_json::to_string_pretty(&user)?;
            let mut f = File::create(&file)?;
            f.write_all(json.as_bytes())?;
            println!("\n✅ User output saved to '{}'", file);
        }
    }

    Ok(())
}

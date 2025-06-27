# 📊 GitHub Repo/User Analyzer CLI

A Rust CLI tool to fetch and display detailed GitHub repository or user profile data, with optional JSON output.


## 🧰 Features

- 🔍 Analyze GitHub repositories or user profiles using the GitHub API
- 📄 View stars, forks, issues, license, language, and more
- 💾 Optionally save output in a nicely formatted JSON file
- ⚡ Fast and lightweight, written in Rust

# Example commands:
1) User Details Command: cargo run -- "swarnabese12"
   Output:
    👤 GitHub User: swarnabese12
    
    🧑 Name       : Swarna Bese
    🏢 Company    : N/A
    📍 Location   : N/A
    📦 Public Repos: 15
    👥 Followers  : 2
    🤝 Following  : 3
    🗓️  Joined     : 2024-10-13T05:10:36Z
    🔗 URL        : https://github.com/swarnabese12

2) Repo details command: cargo run -- "swarnabese12/Solana-Bank-dApp"
   Output: 
     Analyzing GitHub repo: swarnabese12/Solana-Bank-dApp
    
    📄 Name       : Solana-Bank-dApp
    💬 Description: A decentralized banking system on the Solana blockchain (SOLs), featuring functionalities for deposits, withdrawals, loan requests, loan repayments, and detailed transaction history tracking, all presented through a stunning and user-friendly dashboard
    ⭐ Stars      : 1
    🍴 Forks      : 0
    👀 Watchers   : 1
    🐛 Issues     : 0
    📝 License    : N/A
    💻 Language   : TypeScript
    🔗 URL        : https://github.com/swarnabese12/Solana-Bank-dApp


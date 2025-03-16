# Rust Crate Recommender - A Complete Guide

## **Project Overview**
This guide will walk you through the creation of a **Rust Crate Recommender**, a command-line tool that fetches and suggests Rust crates based on user input. The tool will:
- Accept a **keyword** from the user.
- Query `crates.io` for matching crates.
- **Filter & rank** results based on criteria (downloads, last updated, etc.).
- Display a **formatted list** of recommended crates.
- Follow **best practices** in Rust programming, including modularity and single responsibility principles.
- Be version-controlled using **Git** for tracking progress.

---

## **Project Goals & Best Practices**
1. **Modular Codebase** – Organize logic into separate modules.
2. **Single Responsibility Principle** – Each function does one job.
3. **Error Handling** – Use `Result` and `Option` types properly.
4. **Good Documentation & Comments** – Explain what each part of the code does.
5. **Command-Line Usability** – Allow users to interact via the terminal.
6. **Git Version Control** – Regular commits, meaningful messages.
7. **Portfolio-Worthy Code** – Clean, readable, and maintainable.

---

## **Selecting the Right Data Structures**
A well-structured program depends on efficient **data structures** for managing input, storing recommendations, and ranking results. Below are the choices made and why they were selected:

### **1. HashMap for Fast Lookup (Alternative Consideration)**
Initially, we considered using a **HashMap** (`std::collections::HashMap`) to store crate details with the crate name as the key. This would allow for **O(1) lookup time**. However, since recommendations are primarily a list rather than a key-value lookup system, we opted against it.

### **2. Vec for Storing and Sorting Results**
We store fetched crate results in a **Vec (vector)** because:
- It maintains **order** (necessary for ranking results).
- Sorting a `Vec` is **straightforward** with `.sort_by()`.
- It provides **fast iteration** over elements.

```rust
#[derive(Debug, serde::Deserialize)]
pub struct CrateInfo {
    pub id: String,
    pub description: Option<String>,
    pub downloads: u64,
}
```

### **3. Struct for Search Queries**
To handle user input consistently, we define:
```rust
pub struct SearchQuery {
    pub keyword: String,
    pub max_results: usize,
}
```
This provides:
- **Encapsulation** of search parameters.
- **Extensibility** to add more search filters later (e.g., categories).

---

## **Algorithms Used in the Project**
Algorithms power the efficiency of this program. We incorporate:

### **1. Sorting Algorithm (Merge Sort or Quick Sort via Rust’s `sort_by` Method)**
To rank results, we sort them **by number of downloads**:
```rust
crates.sort_by(|a, b| b.downloads.cmp(&a.downloads));
```
This uses **Rust’s Timsort (hybrid merge-insertion sort)**, which provides **O(n log n) complexity**.

### **2. Filtering Algorithm (Linear Scan with Retain)**
To remove crates without descriptions, we use `.retain()`, a **linear scan (O(n))**:
```rust
crates.retain(|c| c.description.is_some());
```
This ensures we only display meaningful recommendations.

---

## **Project Roadmap**

### **Step 1: Setting Up the Project**
1. **Install Rust (if not installed)**:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Create a new Rust project**:
   ```sh
   cargo new rust_crate_recommender
   cd rust_crate_recommender
   ```
3. **Initialize a Git repository**:
   ```sh
   git init
   git add .
   git commit -m "Initial commit"
   ```
4. **Add dependencies** to `Cargo.toml`:
   ```toml
   [dependencies]
   reqwest = { version = "0.11", features = ["json"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   clap = { version = "4.0", features = ["derive"] }
   colored = "2.0"
   ```

---

### **Step 2: Implement the CLI Interface**
```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "Rust Crate Recommender")]
#[command(about = "Finds relevant Rust crates based on keywords")]
struct Cli {
    keyword: String,
    #[arg(default_value_t = 5)]
    max_results: usize,
}

fn main() {
    let args = Cli::parse();
    println!("Searching for crates related to: {}", args.keyword);
}
```

---

### **Step 3: Fetch Data from Crates.io API**
```rust
use reqwest;
use serde::Deserialize;
use crate::data::CrateResponse;

pub async fn fetch_crates(keyword: &str) -> Result<Vec<CrateInfo>, reqwest::Error> {
    let url = format!("https://crates.io/api/v1/crates?q={}&sort=downloads", keyword);
    let response = reqwest::get(&url).await?.json::<CrateResponse>().await?;
    Ok(response.crates)
}
```

---

### **Step 4: Process & Rank the Results**
```rust
crates.sort_by(|a, b| b.downloads.cmp(&a.downloads));
```

---

### **Step 5: Display Results Nicely**
```rust
use colored::*;

println!("{} - {} ({})",
    crate_info.id.green(),
    crate_info.description.as_deref().unwrap_or("No description").blue(),
    crate_info.downloads.to_string().yellow()
);
```

---

## **Final Thoughts**
By using:
- **Vec for ordered storage**
- **Sorting via Rust’s built-in Timsort**
- **Filtering with a simple linear scan**
- **Encapsulated structs for clean data handling**

We ensure an efficient, structured, and scalable **Rust application**. 🚀


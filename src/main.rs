use anyhow::Context;
use hickory_resolver::config::*;
use hickory_resolver::AsyncResolver;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = std::env::args().nth(1).expect("no database");

    let domains = tokio::fs::read_to_string(db)
        .await
        .context("Could not read file")?;

    let mut tasks = JoinSet::new();

    for domain in domains.lines() {
        tasks.spawn(process_domain(domain.into()));
    }
    tasks.join_all().await;
    Ok(())
}

async fn process_domain(domain: String) {
    let resolver = AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    let Ok(response) = resolver.mx_lookup(format!("{domain}.")).await else {
        println!("{domain}|ERROR could not get mx record");
        return;
    };

    let count = response.iter().count();
    if count == 0 {
        println!("{domain}|no mx records");
    } else {
        let records: Vec<_> = response.iter().map(|r| r.exchange().to_utf8()).collect();
        println!("{domain}|{count}|{}", records.join("|"));
    }
}

use chrono::{DateTime, Utc};
use colored::*;
use json;
use std::env;
use std::process::Command;

struct GHEvent {
    who: String,
    when: DateTime<Utc>,
    category: String,
    what: String,
    url: String,
}

fn api_call(path: String) -> json::JsonValue {
    let output = Command::new("gh")
        .args([
            "api",
            "-H",
            "Accept: application/vnd.github+json",
            "-F",
            "per_page=100",
            "-f",
            "direction=desc",
            "-f",
            "state=all",
            "--method",
            "GET",
            &path,
        ])
        .output()
        .expect("Failed to execute command");

    let api_content = String::from_utf8(output.stdout).unwrap();

    let json = json::parse(&api_content).unwrap();
    if !json["documentation_url"].is_null() {
        println!(
            "API call for {} is invalid.\nThe repository certainly doesn't exist.",
            &path
        );
        std::process::exit(1);
    }
    return json;
}

fn show(events: Vec<GHEvent>) {
    // we need to track the current date being displayed
    let mut date = String::new();

    for x in events.iter() {
        let current_date = x.when.date().format("%Y-%m-%d").to_string();

        if date != current_date {
            println!(
                "\n{} {} {}",
                "----------".red(),
                current_date.green().bold(),
                "---------".red()
            );
            date = current_date;
        }
        println!(
            "\n{:05} {} {}\n{}",
            x.when.format("%H:%M").to_string().bold().green(),
            x.who.bold().blue(),
            x.category.bold().red(),
            x.url.blue(),
        );

        for line in textwrap::wrap(&x.what, 80) {
            println!("\t{}", line);
        }
    }
}

fn scrape_data(project: String) -> Vec<GHEvent> {
    let mut events: Vec<GHEvent> = Vec::new();

    let issues = api_call(["/repos", &project, "issues"].join("/"));
    let comments = api_call(["/repos", &project, "issues/comments"].join("/"));

    // iterate over issues
    for i in 0..issues.len() {
        let state = issues[i]["state"].to_string();
        let field;

        if state == "open" {
            field = "created_at";
        } else {
            field = "updated_at";
        }

        // create an issue for the title
        let v = GHEvent {
            who: issues[i]["user"]["login"].to_string(),
            when: DateTime::parse_from_rfc3339(&issues[i][field].to_string())
                .unwrap()
                .with_timezone(&Utc),
            what: issues[i]["title"].to_string(),
            category: format!(
                "{} {}",
                state.to_uppercase(),
                &issues[i]["number"].to_string()
            ),
            url: issues[i]["url"].to_string(),
        };
        events.push(v);

        // create an issue with the comment
        let v = GHEvent {
            who: issues[i]["user"]["login"].to_string(),
            when: DateTime::parse_from_rfc3339(&issues[i]["created_at"].to_string())
                .unwrap()
                .with_timezone(&Utc),
            what: issues[i]["body"].to_string(),
            category: format!(
                "{} {}",
                "OPEN".to_string(),
                &issues[i]["number"].to_string()
            ),
            url: issues[i]["url"].to_string(),
        };
        events.push(v);
    }

    // iterate over comments
    for i in 0..comments.len() {
        let text = comments[i]["body"].to_string();

        let issue_url = comments[i]["issue_url"].to_string();
        let issue_number = issue_url.split("/").last().unwrap();

        let v = GHEvent {
            who: comments[i]["user"]["login"].to_string(),
            when: DateTime::parse_from_rfc3339(&comments[i]["updated_at"].to_string())
                .unwrap()
                .with_timezone(&Utc),
            what: text,
            category: format!("{} {}", "comment", &issue_number),
            url: comments[i]["url"].to_string(),
        };
        events.push(v);
    }

    // sort events by date
    events.sort_by(|a, b| a.when.timestamp().partial_cmp(&b.when.timestamp()).unwrap());
    return events;
}

fn generate_json(events: &Vec<GHEvent>) -> String {
    let mut data = json::JsonValue::new_array();
    for x in events.iter() {
        let mut tuple = json::JsonValue::new_object();

        let url = x.url.clone();
        let who = x.who.clone();
        let when = x.when.clone();
        let what = x.what.clone();
        let category = x.category.clone();

        tuple["url"] = url.into();
        tuple["when"] = when.to_string().into();
        tuple["who"] = who.into();
        tuple["category"] = category.into();
        tuple["what"] = what.into();
        data.push(tuple).expect("error creating JSON");
    }
    return data.dump();
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let events: Vec<GHEvent> = scrape_data(args[1].to_string());
        if args.contains(&"--json".to_string()) {
            println!("{}", generate_json(&events));
        } else {
            show(events);
        }
    }
}

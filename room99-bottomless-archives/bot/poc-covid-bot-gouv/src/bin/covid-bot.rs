#![allow(dead_code)]
#[macro_use]
extern crate prettytable;

use std::sync::Arc;
use std::thread;

use gouv_rs::discord::*;
use gouv_rs::{hook, util};
use hyper::{Body, Response};
use prettytable::Table;

type Resp<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

const PASSIVE_WAIT: u64 = 3600;
const WEBHOOK_URL: &str = "https://ptb.discordapp.com/api/webhooks/689444281236455482/HbjqHj5TcFpGAPbx15MSC4LfHN5VOYOqDqYLptfaYuJkeU20r6G3OV8A3lYCY43Pxk9z";
const USER_AGENT: &str = "CovidBot/0.1.1";

#[tokio::main]
async fn main() -> Resp<()> {
    let client = Arc::new(hyper::Client::builder().build(hyper_tls::HttpsConnector::new()));
    hook(
        "https://raw.githubusercontent.com/opencovid19-fr/data/master/dist/chiffres-cles.csv",
        None,
        None,
        client,
        process_body,
    )
    .await?;
    Ok(())
}

async fn process_body(body: Response<Body>) -> Resp<()> {
    let oldhash = util::read_file("./log.txt");

    push_to_webhook(parse_csv(body).await?, oldhash).await?;

    thread::sleep(std::time::Duration::from_secs(PASSIVE_WAIT));
    Ok(())
}

async fn parse_csv(body: Response<Body>) -> Resp<Vec<Record>> {
    let body = hyper::body::to_bytes(body.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;
    let mut doc = csv::Reader::from_reader(body.as_bytes());

    let mut records: Vec<Record> = Vec::new();
    for result in doc.deserialize() {
        let record: csv::Result<Record> = result;
        if let Ok(rec) = record {
            records.push(rec);
        } else {
            eprintln!("Error while reading CSV, {:?}", record);
        }
    }

    Ok(records)
}

#[derive(Debug)]
struct Stats {
    pub nb_cas: String,
    pub nb_morts: String,
}
async fn push_to_webhook(records: Vec<Record>, oldhash: String) -> Resp<()> {
    let date = chrono::Local::now().format("%A %d %B %Y").to_string();
    let yesterday = records.get_by_date(util::date_before_today(1));
    let monde = string_from(yesterday.get_by_gran(Granularite::Monde).first());
    let france = string_from(yesterday.get_by_gran(Granularite::Pays).first());
    let savoie = string_from(yesterday.get_by_code("DEP-73").first());
    let rhone = string_from(yesterday.get_by_code("DEP-69").first());

    let hashbody = base64::encode(format!("{}{}{}{}{}", date, monde, france, savoie, rhone));

    if oldhash != hashbody {
        println!("[{}] Content updated!", util::time_now_formatted());
        util::write_file("./log.txt", &hashbody);

        let embed = DiscordEmbed {
            title: Some(&date),
            color: Some(4_535_472),
            description: Some("Evolution du nombre de cas recensés en France et dans le Monde, ainsi que du nombre de morts (**daté a J-1**).\n\n*Sources : ||Santé Publique France, Agences Régionale de Santé, Préfectures||*"),
            url: None,
            author: Some(DiscordAuthor {
                name: Some("Covid-19 Daily Update"),
                url: Some("https://github.com/joxcat/gouv-rs"),
                icon_url: None,
                proxy_icon_url: None
            }),
            footer: Some(DiscordFooter {
                text: Some("Source des données https://github.com/opencovid19-fr/data"),
                icon_url: None,
                proxy_icon_url: None
            }),
            fields: Some(vec![
                DiscordField {
                    name: "Stats France",
                    value: &france,
                    inline: Some(false)
                },
                DiscordField {
                    name: "Stats Monde",
                    value: &monde,
                    inline: Some(false)
                },
                DiscordField {
                    name: "Stats Rhône",
                    value: &rhone,
                    inline: Some(false)
                },
                DiscordField {
                    name: "Stats Savoie",
                    value: &savoie,
                    inline: Some(false)
                }
            ])
        };
        let discord_msg = DiscordWebhook {
            inner: DiscordInner::Embeds(vec![embed]),
            username: None,
            avatar_url: None,
            tts: None,
            allowed_mentions: None,
        };
        let body = serde_json::to_string(&discord_msg)?;
        let headers = vec![
            ("User-Agent", USER_AGENT),
            ("Content-Type", "application/json"),
        ];

        let client = Arc::new(hyper::Client::builder().build(hyper_tls::HttpsConnector::new()));
        let x: Response<Body> =
            util::post_uri(WEBHOOK_URL, headers.into_hashmap(), &body, client).await?;
        println!(
            "[{}] => Status Code: {}",
            util::time_now_formatted(),
            x.status()
        );
    } else {
        println!("[{}] Content unchanged!", util::time_now_formatted());
    }

    Ok(())
}

fn string_from(el: Option<&Record>) -> String {
    format!("```\n{}```", Table::from(Stats::from(el)).to_string())
}
// Records management
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Record {
    date: String,
    granularite: Granularite,
    maille_code: String,
    maille_nom: String,
    cas_confirmes: Option<u32>,
    deces: Option<u32>,
    source_nom: String,
    source_url: String,
}
impl Default for Record {
    fn default() -> Self {
        Record {
            date: String::from("N/A"),
            granularite: Granularite::NA,
            maille_code: String::from("N/A"),
            maille_nom: String::from("N/A"),
            cas_confirmes: None,
            deces: None,
            source_nom: String::from("N/A"),
            source_url: String::from("N/A"),
        }
    }
}

trait EasyFilter {
    fn get_by_date(&self, date: util::Date) -> Vec<Record>;
    fn get_by_gran(&self, gran: Granularite) -> Vec<Record>;
    fn get_by_src(&self, src: &str) -> Vec<Record>;
    fn get_by_code(&self, dep: &str) -> Vec<Record>;
}

impl EasyFilter for Vec<Record> {
    fn get_by_date(&self, date: util::Date) -> Vec<Record> {
        let today = format!("{}-{}-{}", date.year, date.month, date.day);
        self.iter()
            .filter_map(|el| {
                if el.date == today {
                    Some(el.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Record>>()
    }

    fn get_by_gran(&self, gran: Granularite) -> Vec<Record> {
        self.iter()
            .filter_map(|el| {
                if el.granularite == gran {
                    Some(el.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Record>>()
    }

    fn get_by_src(&self, src: &str) -> Vec<Record> {
        self.iter()
            .filter_map(|el| {
                if el.source_nom == src {
                    Some(el.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Record>>()
    }

    fn get_by_code(&self, dep_code: &str) -> Vec<Record> {
        let dep_code = dep_code.to_uppercase();
        self.iter()
            .filter_map(|el| {
                if el.maille_code == dep_code.as_str() {
                    Some(el.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Record>>()
    }
}

#[serde(rename_all = "kebab-case")]
#[derive(Deserialize, Debug, Clone, PartialEq)]
enum Granularite {
    Region,
    Departement,
    Pays,
    CollectiviteOutremer,
    Monde,
    NA,
}

impl<'a> From<Option<&Record>> for Stats {
    fn from(rec: Option<&Record>) -> Self {
        let rec = match rec {
            Some(r) => r.clone(),
            None => Record::default(),
        };
        Stats {
            nb_cas: match rec.cas_confirmes {
                Some(nb) => nb.to_string(),
                None => String::from("N/A"),
            },
            nb_morts: match rec.deces {
                Some(nb) => nb.to_string(),
                None => String::from("N/A"),
            },
        }
    }
}

use std::collections::HashMap;

trait FromStrVec {
    fn into_hashmap(&self) -> HashMap<String, String>;
}

impl FromStrVec for Vec<(&str, &str)> {
    fn into_hashmap(&self) -> HashMap<String, String> {
        self.iter()
            .map(|el| (el.0.to_string(), el.1.to_string()))
            .collect::<HashMap<String, String>>()
    }
}

impl From<Stats> for Table {
    fn from(stats: Stats) -> Self {
        let mut t = Table::new();
        t.set_format(*prettytable::format::consts::FORMAT_DEFAULT);

        t.add_row(row!["Nombre de cas", "Nombre de morts"]);
        t.add_row(row![&stats.nb_cas, &stats.nb_morts]);

        t
    }
}

trait FormatToDiscord {
    fn format_discord(&self) -> String;
}

impl FormatToDiscord for Table {
    fn format_discord(&self) -> String {
        format!("```markdown\n{}\n```", self.to_string())
    }
}

use std::{env, fmt::Display};

use clap::Parser;
use log::{error, info, debug};

use rusqlite::params;
use types::{Image, Status, Licence};

mod cli;
mod types;

#[tokio::main]
async fn main() {
    let args = cli::Cli::parse();

    env_logger::init();
    let cred_file = args.dir.join("credentials");
    match dotenv::from_path(&cred_file) {
        Ok(_) => debug!("Loaded credentials from {}", cred_file.display()),
        Err(e) => err_and_exit(e),
    };

    let conn = match rusqlite::Connection::open(args.dir.join("images.db")) {
        Ok(c) => c,
        Err(e) => err_and_exit(e),
    };

    let img = get_image_data(&conn);

    let post = generate_post(&img, args).await;
    match post {
        Ok(mut p) => {
            match eggbug::Session::login(
                &env::var("COPOSTR_EMAIL").unwrap_or_default(),
                &env::var("COPOSTR_PASSWORD").unwrap_or_default()
            ).await {
                Ok(session) => {
                    match session.create_post(&env::var("COPOSTR_PROJECT").unwrap_or_default(), &mut p).await {
                        Ok(_) => write_status(&conn, &img.id, Status::Success),
                        Err(e) => {
                            error!("{}", e);
                            write_status(&conn, &img.id, Status::PostFail);
                        }
                    }
                },
                Err(e) => {
                    error!("{}", e);
                    write_status(&conn, &img.id, Status::PostFail);
                }
            };
        },
        Err(e) => write_status(&conn, &img.id, e),
    }
}

fn err_and_exit(e: impl Display) -> ! {
    error!("{}", e);
    std::process::exit(1)
}

fn get_image_data(conn: &rusqlite::Connection) -> Image {
    let mut stmt = conn.prepare("SELECT * FROM images WHERE status = 0 LIMIT 1")
                        .unwrap_or_else(|e| err_and_exit(e));
    let mut imgs = stmt.query_map([], |r| Ok(Image {
                        id: r.get(0)?,
                        title: r.get(1)?,
                        source_url: r.get(2)?,
                        image_url: r.get(3)?,
                        licence: Licence::from(r.get::<_, String>(4)?),
                    })).unwrap_or_else(|e| err_and_exit(e));
    match imgs.next() {
        Some(v) => match v {
            Ok(img) => img,
            Err(e) => err_and_exit(e),
        },
        None => {
            reset_index(conn);
            get_image_data(conn)
        }
    }
}

fn reset_index(conn: &rusqlite::Connection) {
    info!("Resetting index...");
    match conn.execute("UPDATE images SET status = ?1 WHERE status != ?2",
        params![Status::Unposted as u8, Status::ImageTooLarge as u8]) {
        Ok(v) => info!("{} line(s) updated", v),
        Err(e) => err_and_exit(e),
    };
}

fn write_status(conn: &rusqlite::Connection, id: &u64, status: Status) {
    info!("Writing new status for {}", id);
    match conn.execute("UPDATE images SET status = ?1 WHERE id = ?2", (status as u8, id)) {
        Ok(v) => info!("{} line(s) updated", v),
        Err(e) => err_and_exit(e),
    };
}

async fn download_image(url: &String) -> Result<bytes::Bytes, Status> {
    let resp = match reqwest::get(url).await {
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            return Err(Status::DownloadFail);
        },
    };
    match resp.status() {
        reqwest::StatusCode::OK => (),
        v => {
            error!("Request for {} returned HTTP {}", url, v);
            return Err(Status::DownloadFail);
        },
    }
    resp.bytes().await.or(Err(Status::DownloadFail))
}

async fn generate_post(img: &Image, args: cli::Cli) -> Result<eggbug::Post, Status> {
    match download_image(&img.image_url).await {
        Ok(buf) => {
            if !buf.is_empty() && buf.len() < 5_000_000 {
                let post = eggbug::Post {
                    headline: String::from(&img.title),
                    markdown: format!("[Source]({}) (*{}*)", img.source_url, img.licence),
                    attachments: vec![
                        eggbug::Attachment::new(buf, "image.jpg".into(), "image/jpeg".into())
                            .with_alt_text(args.alt_text.unwrap_or_default())
                    ],
                    tags: args.tags.unwrap_or_default(),
                    ..Default::default()
                };
                return Ok(post);
            } else {
                return Err(Status::ImageTooLarge)
            }
        },
        Err(e) => Err(e),
    }
}

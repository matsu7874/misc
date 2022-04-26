#[macro_use]
extern crate lazy_static;

use std::io::Write;

use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use futures_util::TryStreamExt as _;

const ALBUM_ROOT_PATH: &str = "static/albums";
const TEMPLATE_PATH: &str = "templates/**/*";
lazy_static! {
    pub static ref TEMPLATES: tera::Tera = {
        let mut tera = match tera::Tera::new(TEMPLATE_PATH) {
            std::result::Result::Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

fn get_filenames(glob: &str) -> anyhow::Result<Vec<u32>> {
    let paths = std::fs::read_dir(glob)?;
    let mut ids = vec![];
    for e in paths {
        ids.push(
            e.ok()
                .unwrap()
                .file_name()
                .to_string_lossy()
                .parse::<u32>()
                .ok()
                .unwrap_or(0)
        );
    }
    ids.sort_unstable();
    // TODO: .gitkeepなどが0として入ってくるので消す。
    Ok(ids.into_iter().filter(|x|*x>0).collect())
}

fn get_album_ids() -> anyhow::Result<Vec<u32>> {
    get_filenames(ALBUM_ROOT_PATH)
}

fn get_photo_ids(album_id: u32) -> anyhow::Result<Vec<u32>> {
    get_filenames(&format!("{}/{}", ALBUM_ROOT_PATH, album_id))
}

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let mut context = tera::Context::new();
    let album_ids = match web::block(get_album_ids).await? {
        Ok(ids) => ids,
        Err(_) => vec![],
    };
    context.insert("album_ids", &album_ids);

    let body = TEMPLATES.render("index.html", &context).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

#[post("/album")]
async fn create_album(_req_body: String) -> Result<HttpResponse, actix_web::Error> {
    let album_ids = match web::block(get_album_ids).await? {
        Ok(ids) => ids,
        Err(_) => vec![0],
    };

    let max_id = if album_ids.is_empty() {
        0
    } else {
        *album_ids.iter().max().unwrap()
    };
    let next_id = max_id + 1;
    web::block(move || std::fs::create_dir(format!("{}/{}", ALBUM_ROOT_PATH, next_id))).await??;

    Ok(HttpResponse::SeeOther()
        .append_header(("location", format!("/album/{}", next_id)))
        .finish())
}
#[get("/album/{album_id}")]
async fn detail_album(path: web::Path<(u32,)>) -> Result<HttpResponse, actix_web::Error> {
    let album_id = path.into_inner().0;

    let mut context = tera::Context::new();
    let photo_ids = match web::block(move || get_photo_ids(album_id)).await? {
        Ok(ids) => ids,
        Err(_) => vec![],
    };
    context.insert("album_id", &album_id);
    context.insert("photo_ids", &photo_ids);
    let body = TEMPLATES.render("album.html", &context).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

#[post("/album/{album_id}/photo")]
async fn upload_photo(
    mut payload: Multipart,
    path: web::Path<(u32,)>,
) -> Result<HttpResponse, actix_web::Error> {
    let album_id = path.into_inner().0;
    let photo_ids = match web::block(move || get_photo_ids(album_id)).await? {
        Ok(ids) => ids,
        Err(_) => vec![0],
    };
    let max_id = if photo_ids.is_empty() {
        0
    } else {
        *photo_ids.iter().max().unwrap()
    };
    let next_id = max_id + 1;

    // [examples/main.rs at master · actix/examples](https://github.com/actix/examples/blob/master/forms/multipart/src/main.rs)
    while let Some(mut field) = payload.try_next().await? {
        let filepath = format!("{}/{}/{}", ALBUM_ROOT_PATH, album_id, next_id);
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::SeeOther()
        .append_header(("location", format!("/album/{}", album_id)))
        .finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create_album)
            .service(detail_album)
            .service(upload_photo)
            .service(Files::new("/images", "static/albums/").prefer_utf8(true))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

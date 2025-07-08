use leptos::prelude::*;
use leptos::mount::mount_to_body;
use frontend::app::App; // เปลี่ยนชื่อ module ตามของคุณ

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos_meta::MetaTags;
    use leptos_actix::{generate_route_list, LeptosRoutes};

    // โหลด config จาก leptos_options
    let conf = get_configuration(None).expect("Failed to get config");
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        App::new()
            // Serve ไฟล์ static ต่างๆ
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", &site_root))
            .service(favicon)
            // API routes (ถ้ามี)
            //.service(api::insert_review)
            //.service(api::insert_bulk_reviews)
            //.service(api::search_reviews)
            // Leptos routes สำหรับ SSR
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() />
                                <MetaTags />
                            </head>
                            <body>
                                <App />
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.clone()))
    })
    .bind(addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("/favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!("{}/favicon.ico", site_root))?)
}

#[cfg(not(feature = "ssr"))]
fn main() {
    mount_to_body(|| {
        view! { <App /> }
    });
}
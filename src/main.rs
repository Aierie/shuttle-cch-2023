use tide::{Response, StatusCode};

#[shuttle_runtime::main]
async fn tide() -> shuttle_tide::ShuttleTide<()> {
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.at("/-1/error")
        .get(|_| async { Ok(Response::new(StatusCode::InternalServerError)) });

    Ok(app.into())
}

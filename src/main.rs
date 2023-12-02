use std::{error::Error, ops::BitXor};

use tide::{Request, Response, StatusCode};

#[shuttle_runtime::main]
async fn tide() -> shuttle_tide::ShuttleTide<()> {
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.at("/-1/error")
        .get(|_| async { Ok(Response::new(StatusCode::InternalServerError)) });
    app.at("/1/*path").get(|req: Request<()>| async move {
        let calculated = req
            .param("path")?
            .trim_end_matches('/')
            .split('/')
            .map(|s| s.parse::<i32>())
            .reduce(|acc, e| {
                acc.and_then(|acc_value| e.and_then(|e_value| Ok(acc_value.bitxor(e_value))))
            })
            .ok_or(tide::Error::from_str(
                StatusCode::BadRequest,
                "No params supplied",
            ))??
            .pow(3);

        let mut response = Response::new(StatusCode::Ok);
        response.set_body(calculated.to_string());
        Ok(response)
    });

    Ok(app.into())
}

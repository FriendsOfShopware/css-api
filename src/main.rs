use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lightningcss::stylesheet::{StyleSheet, ParserOptions, PrinterOptions, MinifyOptions};
use serde::{Deserialize};
use lightningcss::targets::Browsers;
use serde_json::json;

#[inline]
pub fn bool_true() -> bool {
    true
}

#[derive(Deserialize)]
struct TransformRequest {
    stylesheet: String,

    #[serde(default = "Browsers::default")]
    targets: Browsers,

    #[serde(default)]
    browserlist: String,

    #[serde(default = "bool_true")]
    minify: bool,

    #[serde(default = "bool_true")]
    custom_media_queries: bool,

    #[serde(default = "bool_true")]
    css_nesting: bool,
}

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    if _event.method().to_string() != "POST" {
        return Ok(Response::builder()
            .status(405)
            .header("Content-Type", "application/json")
            .body(json!( {
                "message": "Method not allowed"
            }).to_string().into())
            .expect("failed to render response"));
    }

    let body: Result<TransformRequest, serde_json::Error> = match _event.body() {
        Body::Text(body) => serde_json::from_str(&body),
        Body::Binary(body) => serde_json::from_slice(&body),
        _ => {
            return Ok(Response::builder()
                .status(400)
                .header("Content-Type", "application/json")
                .body(json!( {
                    "message": "Invalid content-type given"
                }).to_string().into())
                .expect("failed to render response"));
        }
    };

    if let Ok(request) = body {
        let mut parser_options = ParserOptions::default();
        parser_options.custom_media = request.custom_media_queries;
        parser_options.nesting = request.css_nesting;

        let stylesheet = StyleSheet::parse(
            &request.stylesheet,
            parser_options
        );

        if let Err(err) = stylesheet {
            return Ok(Response::builder()
                .status(400)
                .header("Content-Type", "application/json")
                .body(json!( {
                    "message": err.to_string()
                }).to_string().into())
                .expect("failed to render response"));
        }


        let mut minify_options = MinifyOptions::default();
        let mut printer_options = PrinterOptions::default();

        if request.browserlist.is_empty() {
            printer_options.targets = Some(request.targets);
            minify_options.targets = Some(request.targets);
        } else {
            let browser_list = Browsers::from_browserslist([&request.browserlist]);

            if let Err(err) = browser_list {
                return Ok(Response::builder()
                    .status(400)
                    .header("Content-Type", "application/json")
                    .body(json!( {
                        "message": err.to_string()
                    }).to_string().into())
                    .expect("failed to render response"));
            }

            let browser_list = browser_list.unwrap();

            printer_options.targets = browser_list;
            minify_options.targets = browser_list;
        }

        let mut stylesheet_unwrapped = stylesheet.unwrap();

        let minify_result = stylesheet_unwrapped.minify(minify_options);

        if let Err(err) = minify_result {
            return Ok(Response::builder()
                    .status(400)
                    .header("Content-Type", "application/json")
                    .body(json!( {
                        "message": err
                    }).to_string().into())
                    .expect("failed to render response"));
        }

        printer_options.minify = request.minify;

        let build = stylesheet_unwrapped.to_css(printer_options);

        if let Err(err) = build {
            return Ok(Response::builder()
                .status(400)
                .header("Content-Type", "application/json")
                .body(json!( {
                    "message": err.to_string()
                }).to_string().into())
                .expect("failed to render response"));
        }

        Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(json!( {
                "compiled": build.unwrap().code,
            }).to_string().into())
            .expect("failed to render response"))
    } else {
        Ok(Response::builder()
            .status(400)
            .header("Content-Type", "application/json")
            .body(json!( {
                "message": body.err().unwrap().to_string()
            }).to_string().into())
            .expect("failed to render response"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

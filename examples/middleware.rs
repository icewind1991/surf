use futures::future::BoxFuture;
use surf::middleware::{HttpClient, Middleware, Next, Request, Response};

struct Printer;

impl Middleware for Printer {
    fn handle<'a>(
        &'a self,
        req: Request,
        client: Box<dyn HttpClient>,
        next: Next<'a>,
    ) -> BoxFuture<'a, Result<Response, surf::Exception>> {
        Box::pin(async move {
            println!("sending a request!");
            let res = next.run(req, client).await?;
            println!("request completed!");
            Ok(res)
        })
    }
}

#[runtime::main]
async fn main() -> Result<(), surf::Exception> {
    femme::start(log::LevelFilter::Info)?;
    surf::get("https://httpbin.org/get")
        .middleware(Printer {})
        .await?;
    Ok(())
}

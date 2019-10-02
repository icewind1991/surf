use crate::http_client::HttpClient;
use crate::Request;
use std::sync::Arc;
use std::fmt;

#[cfg(feature = "native-client")]
use super::http_client::native::NativeClient;

/// An HTTP client, capable of creating new `Request`s.
///
/// # Examples
///
/// ```no_run
/// # #[runtime::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// let client = surf::Client::new();
/// let req1 = client.get("https://httpbin.org/get").recv_string();
/// let req2 = client.get("https://httpbin.org/get").recv_string();
/// let (str1, str2) = futures::future::try_join(req1, req2).await?;
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct Client {
    client: Arc<dyn HttpClient>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client {{}}")
    }
}

#[cfg(feature = "native-client")]
impl Client {
    /// Create a new instance.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let client = surf::Client::new();
    /// # Ok(()) }
    /// ```
    pub fn new() -> Self {
        Self::with_client(Arc::new(NativeClient::new()))
    }
}

impl Client {
    /// Create a new instance with an `http_client::HttpClient` instance.
    // TODO(yw): hidden from docs until we make the traits public.
    #[doc(hidden)]
    #[allow(missing_doc_code_examples)]
    pub fn with_client(client: Arc<dyn HttpClient>) -> Self {
        Self { client }
    }

    /// Perform an HTTP `GET` request using the `Client` connection.
    ///
    /// # Panics
    ///
    /// This will panic if a malformed URL is passed.
    ///
    /// # Errors
    ///
    /// Returns errors from the middleware, http backend, and network sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let client = surf::Client::new();
    /// let string = client.get("https://httpbin.org/get").recv_string().await?;
    /// # Ok(()) }
    /// ```
    pub fn get(&self, uri: impl AsRef<str>) -> Request {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::GET, uri, self.client.clone())
    }

    /// Perform an HTTP `HEAD` request using the `Client` connection.
    ///
    /// # Panics
    ///
    /// This will panic if a malformed URL is passed.
    ///
    /// # Errors
    ///
    /// Returns errors from the middleware, http backend, and network sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let client = surf::Client::new();
    /// let string = client.head("https://httpbin.org/head").recv_string().await?;
    /// # Ok(()) }
    /// ```
    pub fn head(&self, uri: impl AsRef<str>) -> Request {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::HEAD, uri, self.client.clone())
    }

    /// Perform an HTTP `POST` request using the `Client` connection.
    ///
    /// # Panics
    ///
    /// This will panic if a malformed URL is passed.
    ///
    /// # Errors
    ///
    /// Returns errors from the middleware, http backend, and network sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let client = surf::Client::new();
    /// let string = client.post("https://httpbin.org/post").recv_string().await?;
    /// # Ok(()) }
    /// ```
    pub fn post(&self, uri: impl AsRef<str>) -> Request {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::POST, uri, self.client.clone())
    }

    /// Perform an HTTP `PUT` request using the `Client` connection.
    ///
    /// # Panics
    ///
    /// This will panic if a malformed URL is passed.
    ///
    /// # Errors
    ///
    /// Returns errors from the middleware, http backend, and network sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let client = surf::Client::new();
    /// let string = client.put("https://httpbin.org/put").recv_string().await?;
    /// # Ok(()) }
    /// ```
    pub fn put(&self, uri: impl AsRef<str>) -> Request {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::PUT, uri, self.client.clone())
    }

    /// Perform an HTTP `DELETE` request using the `Client` connection.
    ///
    /// # Panics
    ///
    /// This will panic if a malformed URL is passed.
    ///
    /// # Errors
    ///
    /// Returns errors from the middleware, http backend, and network sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let client = surf::Client::new();
    /// let string = client.delete("https://httpbin.org/delete").recv_string().await?;
    /// # Ok(()) }
    /// ```
    pub fn delete(&self, uri: impl AsRef<str>) -> Request {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::DELETE, uri, self.client.clone())
    }

    /// Perform an HTTP `CONNECT` request using the `Client` connection.
    ///
    /// # Panics
    ///
    /// This will panic if a malformed URL is passed.
    ///
    /// # Errors
    ///
    /// Returns errors from the middleware, http backend, and network sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let client = surf::Client::new();
    /// let string = client.connect("https://httpbin.org/connect").recv_string().await?;
    /// # Ok(()) }
    /// ```
    pub fn connect(&self, uri: impl AsRef<str>) -> Request {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::CONNECT, uri, self.client.clone())
    }

    /// Perform an HTTP `OPTIONS` request using the `Client` connection.
    ///
    /// # Panics
    ///
    /// This will panic if a malformed URL is passed.
    ///
    /// # Errors
    ///
    /// Returns errors from the middleware, http backend, and network sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let client = surf::Client::new();
    /// let string = client.options("https://httpbin.org/options").recv_string().await?;
    /// # Ok(()) }
    /// ```
    pub fn options(&self, uri: impl AsRef<str>) -> Request {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::OPTIONS, uri, self.client.clone())
    }

    /// Perform an HTTP `TRACE` request using the `Client` connection.
    ///
    /// # Panics
    ///
    /// This will panic if a malformed URL is passed.
    ///
    /// # Errors
    ///
    /// Returns errors from the middleware, http backend, and network sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let client = surf::Client::new();
    /// let string = client.trace("https://httpbin.org/trace").recv_string().await?;
    /// # Ok(()) }
    /// ```
    pub fn trace(&self, uri: impl AsRef<str>) -> Request {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::TRACE, uri, self.client.clone())
    }

    /// Perform an HTTP `PATCH` request using the `Client` connection.
    ///
    /// # Panics
    ///
    /// This will panic if a malformed URL is passed.
    ///
    /// # Errors
    ///
    /// Returns errors from the middleware, http backend, and network sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let client = surf::Client::new();
    /// let string = client.patch("https://httpbin.org/patch").recv_string().await?;
    /// # Ok(()) }
    /// ```
    pub fn patch(&self, uri: impl AsRef<str>) -> Request {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::PATCH, uri, self.client.clone())
    }
}

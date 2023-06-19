use std::{str::FromStr, time::Duration};

use cookie::{Cookie, ParseError};
use cookie_store::CookieStore;
use hyper::{
    body::{aggregate, to_bytes, Buf},
    client::HttpConnector,
    header::{
        Entry, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, CONTENT_TYPE, COOKIE, PRAGMA, REFERER,
        SET_COOKIE, USER_AGENT,
    },
    http::{request, HeaderValue},
    Body, Client, HeaderMap, Method, Request, Response, StatusCode, Uri,
};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use log::error;
use tokio::time::timeout;
use url::Url;

const LOGIN_URL: &str = "https://account.anarchy-online.com/";
const ACCOUNT_URL: &str = "https://account.anarchy-online.com/account/";
const SUBSCRIPTION_URL: &str = "https://account.anarchy-online.com/subscription/";
const UNFREEZE_URL: &str = "https://account.anarchy-online.com/uncancel_sub";
const LOGOUT_URL: &str = "https://account.anarchy-online.com/log_out";

const PROXY_URL: &str = "http://proxy.nadybot.org:22222";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

const USER_AGENT_LIST_URL: &str =
    "https://raw.githubusercontent.com/Kikobeats/top-user-agents/master/index.json";

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36";
const NEEDLE: &str = r#"<a href="/subscription/"#;

pub enum SubscriptionId {
    Found(u64),
    NotFound,
    NotParsed,
}

#[derive(Clone)]
pub enum AccountManagerHttpClient {
    Proxied(hyper::Client<hyper_proxy::ProxyConnector<HttpConnector>>),
    Unproxied(hyper::Client<hyper_rustls::HttpsConnector<HttpConnector>>),
}

impl AccountManagerHttpClient {
    pub fn new(with_proxy: bool) -> Self {
        if with_proxy {
            let mut proxy = Proxy::new(Intercept::All, Uri::from_static(PROXY_URL));
            proxy.force_connect();

            let mut connector = HttpConnector::new();
            connector.set_connect_timeout(Some(Duration::from_secs(5)));

            let proxy_connector = ProxyConnector::from_proxy(connector, proxy).unwrap();

            let client = Client::builder().build(proxy_connector);

            Self::Proxied(client)
        } else {
            let connector = hyper_rustls::HttpsConnectorBuilder::new()
                .with_webpki_roots()
                .https_or_http()
                .enable_http1()
                .build();

            let client = Client::builder().build(connector);

            Self::Unproxied(client)
        }
    }

    async fn request(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        match self {
            Self::Proxied(p) => p.request(req).await,
            Self::Unproxied(c) => c.request(req).await,
        }
    }
}

pub struct AccountManager {
    main_username: Option<String>,
    username: String,
    main_password: Option<String>,
    password: String,
    email: String,
    client: AccountManagerHttpClient,
    cookies: CookieStore,
    headers: HeaderMap,
    referer: Option<String>,
}

#[derive(Debug)]
pub enum UnfreezeResult {
    Unfrozen,
    Failed,
}

impl UnfreezeResult {
    #[must_use]
    pub fn should_continue(&self) -> bool {
        matches!(self, Self::Unfrozen)
    }
}

fn uri_to_url(uri: &Uri) -> Url {
    Url::parse(&uri.to_string()).unwrap()
}

impl AccountManager {
    pub fn from_client(client: AccountManagerHttpClient) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
        headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5"));
        headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("document"));
        headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
        headers.insert("Sec-Fetch-Site", HeaderValue::from_static("none"));
        headers.insert("Sec-Fetch-User", HeaderValue::from_static("?1"));
        headers.insert("Sec-GPC", HeaderValue::from_static("1"));
        headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));

        Self {
            main_username: None,
            username: String::new(),
            main_password: None,
            password: String::new(),
            email: String::new(),
            client,
            cookies: CookieStore::new(),
            headers,
            referer: None,
        }
    }

    #[must_use]
    pub fn main_username(mut self, main_username: impl Into<String>) -> Self {
        self.main_username = Some(main_username.into());

        self
    }

    #[must_use]
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();

        self
    }

    #[must_use]
    pub fn main_password(mut self, main_password: impl Into<String>) -> Self {
        self.main_password = Some(main_password.into());

        self
    }

    #[must_use]
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();

        self
    }

    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();

        self
    }

    fn apply_cookies(&self, builder: request::Builder, url: &Url) -> request::Builder {
        let s = self
            .cookies
            .get_request_values(url)
            .map(|(name, value)| format!("{name}=\"{value}\""))
            .collect::<Vec<_>>()
            .join("; ");

        if s.is_empty() {
            builder
        } else {
            builder.header(COOKIE, s)
        }
    }

    fn store_cookies(&mut self, response: &mut Response<Body>, url: &Url) {
        if let Entry::Occupied(entry) = response.headers_mut().entry(SET_COOKIE) {
            let (_, values) = entry.remove_entry_mult();

            let cookies = values.into_iter().filter_map(|val| {
                std::str::from_utf8(val.as_bytes())
                    .map_err(ParseError::from)
                    .and_then(Cookie::parse)
                    .map(Cookie::into_owned)
                    .ok()
            });

            self.cookies.store_response_cookies(cookies, url);
        };
    }

    async fn update_user_agent(&mut self) -> Result<(), hyper::Error> {
        let uri = Uri::from_static(USER_AGENT_LIST_URL);

        let mut res = loop {
            let mut builder = Request::builder().method(Method::GET).uri(&uri);
            builder.headers_mut().unwrap().extend(self.headers.clone());
            let req = builder.body(Body::empty()).unwrap();

            match timeout(REQUEST_TIMEOUT, self.client.request(req)).await {
                Ok(Ok(res)) => break res,
                Ok(Err(e)) => {
                    log::error!("Failed to get user agent list to due {e}, retrying");
                }
                Err(_) => {
                    log::error!("Timeout when fetching user agent list, retrying");
                }
            }
        };

        let body = aggregate(res.body_mut()).await?;

        match serde_json::from_reader::<_, Vec<String>>(body.reader()) {
            Ok(user_agents) => {
                if let Some(user_agent) = user_agents.get(0) {
                    self.headers
                        .insert(USER_AGENT, HeaderValue::from_str(user_agent).unwrap());
                }
            }
            Err(e) => {
                error!("Failed to deserialize user agent list JSON: {e}");
            }
        }

        Ok(())
    }

    async fn open_login_page(&mut self) -> Result<(), hyper::Error> {
        let uri = Uri::from_static(LOGIN_URL);
        let url = uri_to_url(&uri);

        let mut res = loop {
            let mut builder = Request::builder().method(Method::GET).uri(&uri);
            builder.headers_mut().unwrap().extend(self.headers.clone());
            let req = builder.body(Body::empty()).unwrap();

            match timeout(REQUEST_TIMEOUT, self.client.request(req)).await {
                Ok(Ok(res)) => break res,
                Ok(Err(e)) => {
                    log::error!("Failed to request login page to due {e}, retrying");
                }
                Err(_) => {
                    log::error!("Timeout when requesting login page, retrying");
                }
            }
        };

        self.store_cookies(&mut res, &url);
        self.referer = Some(uri.to_string());

        let cookie = Cookie::new("cookieconsent_status", "deny");
        self.cookies.insert_raw(&cookie, &url).unwrap();

        Ok(())
    }

    async fn login(&mut self) -> Result<(), hyper::Error> {
        let uri = Uri::from_static(LOGIN_URL);
        let url = uri_to_url(&uri);

        let mut res = loop {
            let payload = serde_urlencoded::to_string([
                (
                    "nickname",
                    self.main_username.as_deref().unwrap_or(&self.username),
                ),
                (
                    "password",
                    self.main_password.as_deref().unwrap_or(&self.password),
                ),
            ])
            .unwrap();

            let mut builder = Request::builder()
                .method(Method::POST)
                .uri(&uri)
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded");

            if let Some(referer) = &self.referer {
                builder = builder.header(REFERER, referer);
            };

            builder.headers_mut().unwrap().extend(self.headers.clone());
            builder = self.apply_cookies(builder, &url);
            let req = builder.body(Body::from(payload)).unwrap();

            match timeout(REQUEST_TIMEOUT, self.client.request(req)).await {
                Ok(Ok(res)) => {
                    if res.status() == StatusCode::FOUND {
                        break res;
                    }
                    log::error!("Login to account did not return a HTTP 302. Retrying");
                }
                Ok(Err(e)) => {
                    log::error!("Failed to log in to due {e}, retrying");
                }
                Err(_) => {
                    log::error!("Timeout when logging in, retrying");
                }
            }
        };

        self.store_cookies(&mut res, &url);
        self.referer = Some(uri.to_string());

        Ok(())
    }

    async fn open_account_page(&mut self, parse_id: bool) -> Result<SubscriptionId, hyper::Error> {
        let uri = Uri::from_static(ACCOUNT_URL);
        let url = uri_to_url(&uri);

        let mut res = loop {
            let mut builder = Request::builder().method(Method::GET).uri(&uri);

            if let Some(referer) = &self.referer {
                builder = builder.header(REFERER, referer);
            };

            builder.headers_mut().unwrap().extend(self.headers.clone());
            builder = self.apply_cookies(builder, &url);
            let req = builder.body(Body::empty()).unwrap();

            match timeout(REQUEST_TIMEOUT, self.client.request(req)).await {
                Ok(Ok(res)) => break res,
                Ok(Err(e)) => {
                    log::error!("Failed to open account page to due {e}, retrying");
                }
                Err(_) => {
                    log::error!("Timeout when opening account page, retrying");
                }
            }
        };

        self.store_cookies(&mut res, &url);
        self.referer = Some(uri.to_string());

        if !parse_id {
            return Ok(SubscriptionId::NotParsed);
        }

        let body = unsafe { String::from_utf8_unchecked(to_bytes(res.body_mut()).await?.to_vec()) };

        let mut idx = 0;

        loop {
            if let Some(occurence_idx) = body[idx..].find(NEEDLE) {
                idx += occurence_idx + NEEDLE.len();

                if let Some(end_idx) = body[idx..].find('"') {
                    let subscription_id_str = &body[idx..idx + end_idx];
                    idx += end_idx + "\">".len();

                    if let Some(name_end_idx) = body[idx..].find('<') {
                        let user_name = &body[idx..idx + name_end_idx];

                        if user_name.eq_ignore_ascii_case(&self.username) {
                            return Ok(SubscriptionId::Found(subscription_id_str.parse().unwrap()));
                        }
                    }
                }
            } else {
                return Ok(SubscriptionId::NotFound);
            }
        }
    }

    async fn switch_subscription(&mut self, subscription_id: u64) -> Result<(), hyper::Error> {
        let uri = Uri::from_str(&format!("{SUBSCRIPTION_URL}{subscription_id}")).unwrap();
        let url = uri_to_url(&uri);

        let mut res = loop {
            let mut builder = Request::builder().method(Method::GET).uri(&uri);

            if let Some(referer) = &self.referer {
                builder = builder.header(REFERER, referer);
            };

            builder.headers_mut().unwrap().extend(self.headers.clone());
            builder = self.apply_cookies(builder, &url);
            let req = builder.body(Body::empty()).unwrap();

            match timeout(REQUEST_TIMEOUT, self.client.request(req)).await {
                Ok(Ok(res)) => {
                    if res.status() == StatusCode::FOUND {
                        break res;
                    }
                    log::error!("Failed to switch subscription, invalid subscription ID? Retrying");
                }
                Ok(Err(e)) => {
                    log::error!("Failed to switch subscription to due {e}, retrying");
                }
                Err(_) => {
                    log::error!("Timeout when switching subscription, retrying");
                }
            }
        };

        self.store_cookies(&mut res, &url);
        self.referer = Some(uri.to_string());

        Ok(())
    }

    async fn unfreeze_account(&mut self) -> Result<(), hyper::Error> {
        let uri = Uri::from_static(UNFREEZE_URL);
        let url = uri_to_url(&uri);

        let mut res = loop {
            let mut builder = Request::builder().method(Method::GET).uri(&uri);

            if let Some(referer) = &self.referer {
                builder = builder.header(REFERER, referer);
            };

            builder.headers_mut().unwrap().extend(self.headers.clone());
            builder = self.apply_cookies(builder, &url);
            let req = builder.body(Body::empty()).unwrap();

            match timeout(REQUEST_TIMEOUT, self.client.request(req)).await {
                Ok(Ok(res)) => {
                    if res.status() == StatusCode::FOUND {
                        log::info!("Successfully unfroze account.");
                        break res;
                    }
                    log::error!("Failed to unfreeze acount, retrying");
                }
                Ok(Err(e)) => {
                    log::error!("Failed to unfreeze account due to {e}, retrying");
                }
                Err(_) => {
                    log::error!("Timeout when unfreezing account page, retrying");
                }
            }
        };

        self.store_cookies(&mut res, &url);
        self.referer = Some(uri.to_string());

        Ok(())
    }

    async fn logout(&mut self) -> Result<(), hyper::Error> {
        let uri = Uri::from_static(LOGOUT_URL);
        let url = uri_to_url(&uri);

        let mut res = loop {
            let mut builder = Request::builder().method(Method::GET).uri(&uri);

            if let Some(referer) = &self.referer {
                builder = builder.header(REFERER, referer);
            };

            builder.headers_mut().unwrap().extend(self.headers.clone());
            builder = self.apply_cookies(builder, &url);
            let req = builder.body(Body::empty()).unwrap();

            match timeout(REQUEST_TIMEOUT, self.client.request(req)).await {
                Ok(Ok(res)) => {
                    if res.status() == StatusCode::FOUND {
                        break res;
                    }
                    log::error!("Failed to log out, retrying");
                }
                Ok(Err(e)) => {
                    log::error!("Failed to log out due to {e}, retrying");
                }
                Err(_) => {
                    log::error!("Timeout when logging out, retrying");
                }
            }
        };

        self.store_cookies(&mut res, &url);
        self.referer = Some(uri.to_string());

        Ok(())
    }

    pub async fn reactivate(&mut self) -> Result<UnfreezeResult, hyper::Error> {
        self.update_user_agent().await?;
        self.open_login_page().await?;
        self.login().await?;
        let subscription_id = self.open_account_page(true).await?;

        match subscription_id {
            SubscriptionId::Found(subscription_id) => {
                self.switch_subscription(subscription_id).await?;
                self.open_account_page(false).await?;
                self.unfreeze_account().await?;
                self.open_account_page(false).await?;
                self.logout().await?;

                Ok(UnfreezeResult::Unfrozen)
            }
            SubscriptionId::NotFound => Ok(UnfreezeResult::Failed),
            SubscriptionId::NotParsed => unreachable!(),
        }
    }
}

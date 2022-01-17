use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), CrawlError> {
    let urls = (
        "http://10.0.0.6/sd/init?scheme=http&host=10.0.0.6&path=%2f",
        |hrun_mac: &str, hrun_cip: &str, hrun_oip: &str| {
            format!(
                "https://redirect.teliawifi.telia.com/portal?mac={}&ip={}&nas_ip={}",
                hrun_mac, hrun_cip, hrun_oip
            )
        },
        |session_token: &str| {
            format!(
                "https://cp.teliawifi.telia.com/TW-Reg/api/telia/v1/email_registration/{}",
                session_token
            )
        },
    );

    let five_seconds = Duration::new(5, 0);
    let client = reqwest::Client::builder()
        .connect_timeout(five_seconds)
        .build()
        .unwrap();

    eprintln!("Get 10.0.0.6");
    let res = client.get(urls.0).send().await?;
    let body = res.text().await?;

    eprintln!("Get addresses");
    let parsed = parse_body(body).ok_or("Nothing to parse")?;
    let (hrun_mac, hrun_cip, hrun_oip) = parsed;
    let res = client
        .get(urls.1(
            hrun_mac.as_str(),
            hrun_cip.as_str(),
            hrun_oip.as_str(),
        ))
        .send()
        .await?;

    eprintln!("Get Token");
    let url = res.url();
    let (_, token) = url
        .query_pairs()
        .find(|(key, _)| key == "session_token")
        .ok_or("Missing Token!")?;

    client
        .post(urls.2(token.into_owned().as_str()))
        .body(r#"{"email":"m@m.m"}"#)
        .send()
        .await?;

    if res.status().is_success() {
        eprintln!("Success");
    } else {
        return Err("Something went wrong!".into());
    }

    eprintln!("-- Done --");
    Ok(())
}

pub fn parse_body(body: String) -> Option<(String, String, String)> {
    let (hrun_mac_index, _) = body.match_indices("hrunMAC").next()?;
    let (hrun_cip_index, _) = body.match_indices("hrunCIP").next()?;
    let (hrun_oip_index, _) = body.match_indices("hrunOIP").next()?;

    let hrun_mac = &body[hrun_mac_index..hrun_mac_index + 30]
        .split('"')
        .nth(1)
        .unwrap()
        .to_owned();
    let hrun_cip = &body[hrun_cip_index..hrun_cip_index + 30]
        .split('"')
        .nth(1)
        .unwrap()
        .to_owned();
    let hrun_oip = &body[hrun_oip_index..hrun_oip_index + 30]
        .split('"')
        .nth(1)
        .unwrap()
        .to_owned();

    Some((hrun_mac.into(), hrun_cip.into(), hrun_oip.into()))
}

#[derive(Debug)]
enum CrawlError {
    StrErr(&'static str),
    ReqErr(reqwest::Error),
}

impl From<&'static str> for CrawlError {
    fn from(err: &'static str) -> CrawlError {
        CrawlError::StrErr(err)
    }
}
impl From<reqwest::Error> for CrawlError {
    fn from(err: reqwest::Error) -> CrawlError {
        CrawlError::ReqErr(err)
    }
}

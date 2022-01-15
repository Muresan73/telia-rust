#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
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

    let res = reqwest::get(urls.0).await?;

    let body = res.text().await?;
    let parsed = parse_body(body);
    if let Some((hrun_mac, hrun_cip, hrun_oip)) = parsed {
        eprintln!("{:?}", (&hrun_mac, &hrun_cip, &hrun_oip));
        eprintln!(
            "{:?}",
            urls.1(hrun_mac.as_str(), hrun_cip.as_str(), hrun_oip.as_str(),)
        );

        let res = reqwest::get(urls.1(
            hrun_mac.as_str(),
            hrun_cip.as_str(),
            hrun_oip.as_str(),
        ))
        .await?;

        let url = res.url();

        let query = url.query_pairs().find(|(key, _)| key == "session_token");
        if let Some((_, token)) = query {
            let client = reqwest::Client::new();
            let res = client
                .post(urls.2(token.into_owned().as_str()))
                .body(r#"{"email":"m@m.m"}"#)
                .send()
                .await?;
            if res.status().is_success() {
                eprintln!("Success");
            } else {
                eprintln!("Something went wrong!");
            }
        } else {
            eprintln!("Missing Token!");
        }
    } else {
        eprintln!("Nothing to parse")
    }
    eprintln!("Done");

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

#[cfg(test)]
#[test]
fn is_parse_correct() {
    let body = String::from(
        r#"
    <html>
    <head>
        <base href="http://10.0.0.6/smaccd/hrse_vwm_prod_rr/">
        <meta http-equiv="content-type" content="text/html;charset=UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=0">
        <meta name="author" content="Aptilo Networks">
        <meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1">
        <meta http-equiv="cache-control" content="no-cache">
        <meta http-equiv="pragma" content="no-cache">
        <title>TELIA WIFI</title>
    </head>
    <body>
        <script type="text/javascript">
            var hrunLang = "en",
                hrefURL  = "http://10.0.0.6/smaccd/hrse_vwm_prod_rr/",
                hrunLCP  = "tsse001351",
                hrunANW  = "anw001",
                hrunMAC  = "58:f8:cd:87:61:96",
                hrunCIP  = "78.76.88.150",
                hrunOIP  = "78.75.108.26";
            window.location = 'https://redirect.teliawifi.telia.com/portal?mac=' + hrunMAC + '&ip=' + hrunCIP + '&nas_ip=' + hrunOIP;
        </script>
        </body>
    </html>
    <!--
    <?xml version="1.0"?>

    <smartClient>
    <page>
        <login>
        <login_url>http://10.0.0.6/sd/gis_login</login_url>
        </login>
    </page>
    </smartClient>
    // -->
    "#,
    );
    assert_eq!(
        parse_body(body).unwrap(),
        (
            String::from("58:f8:cd:87:61:96"),
            String::from("78.76.88.150"),
            String::from("78.75.108.26")
        )
    );
}

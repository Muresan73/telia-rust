
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
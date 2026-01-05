use fluxor::cans::content::do_html;

const HEAD: &str = r#"<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta name="description" content="{{description}}" />
    <meta name="keywords" content="{{keywords}}" />
    <link rel="manifest" href="/manifest.json" />
    <link rel="icon" href="/images/favicon.ico" type="image/x-icon" />
    <title>{{page_title}}</title>
    <link href="fonts.googleapis.com" rel="stylesheet">
    <link rel="stylesheet" href="/css/styles.css">
    <script defer src="/js/alpine.min.js"></script>
</head>"#;

pub fn head(title: &str, description: &str, keywords: &str) -> String {
    do_html!(
        HEAD,
        description=description,
        keywords=keywords,
        page_title=title
    )
}
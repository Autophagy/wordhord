pub static INDEX: &str = r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta http-equiv="x-ua-compatible" content="ie=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=yes">
    <meta name="theme-color" content="#000000">
    <link rel="shortcut icon" href="/favicon.ico">
    <link rel="stylesheet" href="/static/css/main.css">
    <link rel="preconnect" href="https://fonts.gstatic.com">
    <link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono&display=swap" rel="stylesheet">
    <title>wordhord</title>
</head>
    <body>
        <header>
            wordhord
        </header>
        <ul>
        {{ for post in posts }}
            <li><a href="gewritu/{post.slug}">[{post.published}] {post.title}</a></li>
        {{ endfor }}
        </ul>
        {{ call footer with config }}
    </body>
</html>
"##;

pub static POST: &str = r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta http-equiv="x-ua-compatible" content="ie=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=yes">
    <meta name="theme-color" content="#000000">
    <link rel="shortcut icon" href="/favicon.ico">
    <link rel="stylesheet" href="/static/css/main.css">
    <link rel="preconnect" href="https://fonts.gstatic.com">
    <link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono&display=swap" rel="stylesheet">
    <title>[{post.published}] :: {post.title}</title>
</head>
    <body>
        <header>
            <div id="post-path">
                <a href="/">wordhord</a>
            </div>
            <div id="post-details">
                <div class="row">
                    <span class="label">title</span><span class="sep">::</span><span class="detail">{post.title}</span>
                </div>
                <div class="row">
                    <span class="label">published</span><span class="sep">::</span><span class="detail">{post.published}</span>
                </div>
                <div class="row">
                    <span class="label">time</span><span class="sep">::</span><span class="detail">{post.read_time} mins</span>
                </div>
                <div class="row">
                    <span class="label">tags</span><span class="sep">::</span><span class="detail">[ {{ for tag in post.tags }} <a href="/tags/{tag}">{tag}</a> {{ endfor }} ]</span>
                </div>
            </div>
        </header>
        <div id="content">
            {post.content}
        </div>
        {{ call footer with config }}
    </body>
</html>
"##;

pub static TAG: &str = r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta http-equiv="x-ua-compatible" content="ie=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=yes">
    <meta name="theme-color" content="#000000">
    <link rel="shortcut icon" href="/favicon.ico">
    <link rel="stylesheet" href="/static/css/main.css">
    <link rel="preconnect" href="https://fonts.gstatic.com">
    <link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono&display=swap" rel="stylesheet">
    <title>{tag}</title>
</head>
    <body>
        <header>
            <div id="post-path">
                <a href="/">wordhord</a> :: tag :: {tag}
            </div>
        </header>
        <ul>
            {{ for post in posts }}
            <li><a href="/gewritu/{post.slug}">[{post.published}] :: {post.title}</a></li>
            {{ endfor}}
        </ul>
        {{ call footer with config }}
    </body>
</html>
"##;

pub static FOOTER: &str = r##"
<footer>
    <div id="derivation">
        <a href="{repo}">{drv}</a>
    </div>
</footer>
"##;

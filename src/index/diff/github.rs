use reqwest::{StatusCode, Url};

#[derive(Debug)]
pub(crate) enum FastPathRev {
    UpToDate,
    NeedsFetch,
    Indeterminate,
}

pub(crate) fn has_changes(
    fetch_url: &gix::Url,
    last_seen_reference: &gix::ObjectId,
    branch_name: &str,
) -> Result<FastPathRev, reqwest::Error> {
    let url = match Url::parse(&fetch_url.to_string()) {
        Ok(url) => url,
        Err(_) => return Ok(FastPathRev::Indeterminate),
    };

    if url.scheme() != "https" || !(url.host_str() == Some("github.com")) {
        return Ok(FastPathRev::Indeterminate);
    }

    // This expects GitHub urls in the form `github.com/user/repo` and nothing
    // else
    let mut pieces = url.path_segments().unwrap();

    let username = dbg!(pieces.next().unwrap());

    let repository = pieces.next().unwrap();
    let repository = dbg!(repository.strip_suffix(".git").unwrap_or(repository));

    if pieces.next().is_some() {
        panic!("too many segments on URL");
    }

    let url = dbg!(format!(
        "https://api.github.com/repos/{}/{}/commits/{}",
        username, repository, branch_name,
    ));

    let client = reqwest::blocking::Client::builder()
        .user_agent("crates-index-diff")
        .build()?;
    let response = client
        .get(&url)
        .header("Accept", "application/vnd.github.sha")
        .header("If-None-Match", format!("\"{}\"", last_seen_reference))
        .send()?
        .error_for_status()?;

    if response.status() == StatusCode::NOT_MODIFIED {
        Ok(FastPathRev::UpToDate)
    } else {
        Ok(FastPathRev::NeedsFetch)
    }
}

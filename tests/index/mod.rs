use crates_index_diff::Index;
use git_testtools::tempfile::TempDir;
use std::path::PathBuf;

mod changes_between_commits;

const NUM_CHANGES_SINCE_EVER: usize = 3516;

#[test]
fn peek_changes() -> crate::Result {
    let mut index = index_ro()?;
    index.branch_name = "main";
    assert!(
        index.last_seen_reference().is_err(),
        "marker ref doesn't exist"
    );
    let (changes, last_seen_revision) = index.peek_changes()?;
    assert_eq!(
        changes.len(),
        NUM_CHANGES_SINCE_EVER,
        "all changes since the beginning of history"
    );

    let origin_main = index
        .repository()
        .find_reference("refs/remotes/origin/main")?;
    assert_eq!(
        last_seen_revision,
        origin_main.id(),
        "last seen reference should the latest state from the clone"
    );
    assert!(
        index.last_seen_reference().is_err(),
        "the last-seen reference has not been created"
    );
    Ok(())
}

#[test]
fn clone_if_needed() {
    let tmp = TempDir::new().unwrap();
    Index::from_path_or_cloned_with_options(tmp.path(), clone_options())
        .expect("successful clone to be created");
    Index::from_path_or_cloned_with_options(tmp.path(), clone_options())
        .expect("second instance re-uses existing clone");
}

#[test]
fn quick_changes_since_last_fetch() -> crate::Result {
    let (index, _tmp) = index_rw()?;
    assert!(index.last_seen_reference().is_err(), "no marker exists");
    let num_changes_since_first_commit = index.fetch_changes()?.len();
    assert_eq!(
        num_changes_since_first_commit, NUM_CHANGES_SINCE_EVER,
        "all changes since ever"
    );
    let mut marker = index
        .last_seen_reference()
        .expect("must be created/update now");
    let remote_main = index
        .repository()
        .find_reference("refs/remotes/origin/main")?;
    assert_eq!(
        marker.target(),
        remote_main.target(),
        "we are updated to the most recent known version of the remote"
    );

    // reset to previous one
    marker
        .set_target_id(
            index
                .repository()
                .rev_parse(format!("{}~2", index.seen_ref_name).as_str())?
                .single()
                .unwrap(),
            "resetting to previous commit",
        )
        .expect("reset success");
    let num_seen_after_reset = index.fetch_changes()?.len();
    assert_eq!(
        index.last_seen_reference()?.target(),
        remote_main.target(),
        "seen branch was updated again"
    );
    assert_eq!(
        num_seen_after_reset, 1,
        "normalization has no changes, but the commit before has one"
    );

    assert_eq!(
        index.fetch_changes()?.len(),
        0,
        "nothing if there was no change"
    );
    Ok(())
}

fn index_ro() -> crate::Result<Index> {
    let dir = fixture_dir()?;
    Ok(Index::from_path_or_cloned(dir.join("clone"))?)
}

fn index_rw() -> crate::Result<(Index, TempDir)> {
    let tmp = TempDir::new().unwrap();
    let mut index = Index::from_path_or_cloned_with_options(tmp.path(), clone_options())?;
    index.branch_name = "main";
    Ok((index, tmp))
}

fn fixture_dir() -> crate::Result<PathBuf> {
    git_testtools::scripted_fixture_repo_read_only_with_args(
        "make-index-from-parts.sh",
        std::env::current_dir()
            .ok()
            .map(|p| p.to_str().unwrap().to_owned()),
    )
}

fn clone_options() -> crates_index_diff::index::CloneOptions<'static> {
    crates_index_diff::index::CloneOptions {
        repository_url: fixture_dir().unwrap().join("base").display().to_string(),
        fetch_options: None,
    }
}
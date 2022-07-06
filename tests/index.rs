use crates_index_diff::*;
use git2::Reference;
use serial_test::serial;
use std::{collections::HashMap, env, path::PathBuf};
use tempdir::TempDir;

const NUM_VERSIONS_AT_RECENT_COMMIT: usize = 39752;
// TODO: find new hashes for the ones below with similar states as they don't exist anymore. See ignored tests.
const REV_ONE_ADDED: &str = "615c9c41942a3ba13e088fbcb1470c61b169a187";
const REV_ONE_YANKED: &str = "8cf8fbad7876586ced34c4b778f6a80fadd2a59b";
const REV_ONE_UNYANKED: &str = "f8cb00181";
const REV_CRATE_DELETE: &str = "de5be3e8bb6cd7a3179857bdbdf28ca4fa23f84c";
const REV_MANY_CRATE_DELETE: &str = "d04e9e978cc1fe9c96cf3515e5c8cad7f0c0090a";

#[test]
#[ignore] // This test takes too long for my taste, this library is stable by now
fn clone_if_needed() {
    let tmp = TempDir::new("new-index").unwrap();
    Index::from_path_or_cloned(tmp.path()).expect("successful clone to be created");
    Index::from_path_or_cloned(tmp.path()).expect("second instance re-uses existing clone");
}

fn make_index() -> (Index, TempDir) {
    let tmp = TempDir::new("new-index").unwrap();
    let index = Index::from_path_or_cloned(
        env::var("CRATES_INDEX_DIFF_TEST_EXISTING_INDEX")
            .map(PathBuf::from)
            .unwrap_or_else(|_| tmp.path().to_owned()),
    )
    .expect("successful clone");
    (index, tmp)
}

fn origin_master_of(index: &Index) -> Reference<'_> {
    index
        .repository()
        .find_reference("refs/remotes/origin/master")
        .unwrap()
}

#[test]
#[serial]
#[ignore]
fn quick_changes_since_last_fetch() {
    let (mut index, _tmp) = make_index();
    index.seen_ref_name = "refs/our-test-ref_because-we-can_hidden-from-ui";
    index
        .last_seen_reference()
        .and_then(|mut r| r.delete())
        .ok();
    let num_changes_since_first_commit = index.fetch_changes().unwrap().len();
    assert!(
        num_changes_since_first_commit >= NUM_VERSIONS_AT_RECENT_COMMIT,
        "should have fetched enough commits"
    );
    let mut seen_marker_ref = index
        .last_seen_reference()
        .expect("must be created/update now");
    assert!(
        seen_marker_ref == origin_master_of(&index),
        "should update the last_seen_reference to latest remote origin master"
    );

    // reset to previous one
    seen_marker_ref
        .set_target(
            index
                .repository()
                .revparse_single(REV_ONE_UNYANKED)
                .unwrap()
                .id(),
            "resetting to previous commit",
        )
        .expect("reset success");
    let num_seen_after_reset = index.fetch_changes().unwrap().len();
    let origin_master = origin_master_of(&index);
    assert!(
        seen_marker_ref == origin_master,
        "{} ({}) != {} ({})",
        seen_marker_ref.name().unwrap(),
        seen_marker_ref.peel_to_commit().unwrap().id(),
        origin_master.name().unwrap(),
        origin_master.peel_to_commit().unwrap().id()
    );
    assert!(num_seen_after_reset < num_changes_since_first_commit);
    assert!(num_seen_after_reset > 1000);

    // nothing if there was no change
    assert_eq!(index.fetch_changes().unwrap().len(), 0);
}

#[test]
#[serial]
fn peek_changes_since_last_fetch() {
    let (mut index, _tmp) = make_index();
    index.seen_ref_name = "refs/our-test-ref_because-we-can_hidden-from-ui";
    index
        .last_seen_reference()
        .and_then(|mut r| r.delete())
        .ok();
    let (changes, last_seen_rev) = index.peek_changes().unwrap();
    assert!(changes.len() >= NUM_VERSIONS_AT_RECENT_COMMIT);
    assert_eq!(
        last_seen_rev,
        origin_master_of(&index).target().unwrap(),
        "last seen reference should be origin"
    );
    assert!(
        index.last_seen_reference().is_err(),
        "the last-seen reference has not been created (or updated, but we don't test that yet)"
    );
}

fn changes_of(index: &Index, commit: &str) -> Vec<Change> {
    index
        .changes(format!("{}~1^{{tree}}", commit), commit)
        .expect("id to be valid and diff OK")
}

#[test]
#[serial]
fn crate_delete() {
    let (index, _tmp) = make_index();

    let changes = changes_of(&index, REV_CRATE_DELETE);
    assert_eq!(changes, vec![Change::Deleted("rustdecimal".to_string())],);
}

#[test]
#[serial]
fn many_crate_delete() {
    let (index, _tmp) = make_index();

    let changes = changes_of(&index, REV_MANY_CRATE_DELETE);
    assert_eq!(
        changes,
        vec![
            Change::Deleted("ago".to_string()),
            Change::Deleted("bed".to_string()),
            Change::Deleted("boy".to_string()),
            Change::Deleted("buy".to_string()),
            Change::Deleted("cmu".to_string()),
            Change::Deleted("cup".to_string()),
            Change::Deleted("her".to_string()),
            Change::Deleted("him".to_string()),
            Change::Deleted("his".to_string()),
            Change::Deleted("its".to_string()),
            Change::Deleted("kid".to_string()),
            Change::Deleted("law".to_string()),
            Change::Deleted("our".to_string()),
            Change::Deleted("per".to_string()),
            Change::Deleted("pku".to_string()),
            Change::Deleted("sit".to_string()),
            Change::Deleted("ucb".to_string()),
            Change::Deleted("yet".to_string()),
            Change::Deleted("agree".to_string()),
            Change::Deleted("agreement".to_string()),
            Change::Deleted("ahead".to_string()),
            Change::Deleted("alone".to_string()),
            Change::Deleted("along".to_string()),
            Change::Deleted("already".to_string()),
            Change::Deleted("although".to_string()),
            Change::Deleted("always".to_string()),
            Change::Deleted("american".to_string()),
            Change::Deleted("among".to_string()),
            Change::Deleted("amount".to_string()),
            Change::Deleted("another".to_string()),
            Change::Deleted("answer".to_string()),
            Change::Deleted("anyone".to_string()),
            Change::Deleted("appear".to_string()),
            Change::Deleted("approach".to_string()),
            Change::Deleted("around".to_string()),
            Change::Deleted("arrive".to_string()),
            Change::Deleted("article".to_string()),
            Change::Deleted("attack".to_string()),
            Change::Deleted("attention".to_string()),
            Change::Deleted("attorney".to_string()),
            Change::Deleted("audience".to_string()),
            Change::Deleted("author".to_string()),
            Change::Deleted("available".to_string()),
            Change::Deleted("avoid".to_string()),
            Change::Deleted("away".to_string()),
            Change::Deleted("baby".to_string()),
            Change::Deleted("back".to_string()),
            Change::Deleted("beat".to_string()),
            Change::Deleted("beautiful".to_string()),
            Change::Deleted("because".to_string()),
            Change::Deleted("before".to_string()),
            Change::Deleted("begin".to_string()),
            Change::Deleted("behavior".to_string()),
            Change::Deleted("behind".to_string()),
            Change::Deleted("berkeley".to_string()),
            Change::Deleted("better".to_string()),
            Change::Deleted("billion".to_string()),
            Change::Deleted("bring".to_string()),
            Change::Deleted("brother".to_string()),
            Change::Deleted("building".to_string()),
            Change::Deleted("business".to_string()),
            Change::Deleted("caltech".to_string()),
            Change::Deleted("cambridge".to_string()),
            Change::Deleted("campaign".to_string()),
            Change::Deleted("cancer".to_string()),
            Change::Deleted("candidate".to_string()),
            Change::Deleted("capital".to_string()),
            Change::Deleted("care".to_string()),
            Change::Deleted("career".to_string()),
            Change::Deleted("carry".to_string()),
            Change::Deleted("central".to_string()),
            Change::Deleted("century".to_string()),
            Change::Deleted("certain".to_string()),
            Change::Deleted("chair".to_string()),
            Change::Deleted("challenge".to_string()),
            Change::Deleted("character".to_string()),
            Change::Deleted("child".to_string()),
            Change::Deleted("clearly".to_string()),
            Change::Deleted("coach".to_string()),
            Change::Deleted("college".to_string()),
            Change::Deleted("commercial".to_string()),
            Change::Deleted("concern".to_string()),
            Change::Deleted("conference".to_string()),
            Change::Deleted("congress".to_string()),
            Change::Deleted("consider".to_string()),
            Change::Deleted("contain".to_string()),
            Change::Deleted("couple".to_string()),
            Change::Deleted("court".to_string()),
            Change::Deleted("crime".to_string()),
            Change::Deleted("cultural".to_string()),
            Change::Deleted("culture".to_string()),
            Change::Deleted("customer".to_string()),
            Change::Deleted("daughter".to_string()),
            Change::Deleted("debate".to_string()),
            Change::Deleted("defense".to_string()),
            Change::Deleted("degree".to_string()),
            Change::Deleted("democrat".to_string()),
            Change::Deleted("democratic".to_string()),
            Change::Deleted("describe".to_string()),
            Change::Deleted("design".to_string()),
            Change::Deleted("despite".to_string()),
            Change::Deleted("detail".to_string()),
            Change::Deleted("determine".to_string()),
            Change::Deleted("develop".to_string()),
            Change::Deleted("development".to_string()),
            Change::Deleted("different".to_string()),
            Change::Deleted("difficult".to_string()),
            Change::Deleted("director".to_string()),
            Change::Deleted("discuss".to_string()),
            Change::Deleted("discussion".to_string()),
            Change::Deleted("disease".to_string()),
            Change::Deleted("door".to_string()),
            Change::Deleted("during".to_string()),
            Change::Deleted("east".to_string()),
            Change::Deleted("economic".to_string()),
            Change::Deleted("education".to_string()),
            Change::Deleted("effort".to_string()),
            Change::Deleted("eight".to_string()),
            Change::Deleted("employee".to_string()),
            Change::Deleted("enough".to_string()),
            Change::Deleted("entire".to_string()),
            Change::Deleted("especially".to_string()),
            Change::Deleted("establish".to_string()),
            Change::Deleted("evening".to_string()),
            Change::Deleted("everybody".to_string()),
            Change::Deleted("everyone".to_string()),
            Change::Deleted("everything".to_string()),
            Change::Deleted("evidence".to_string()),
            Change::Deleted("exactly".to_string()),
            Change::Deleted("experience".to_string()),
            Change::Deleted("family".to_string()),
            Change::Deleted("federal".to_string()),
            Change::Deleted("feeling".to_string()),
            Change::Deleted("field".to_string()),
            Change::Deleted("fight".to_string()),
            Change::Deleted("finish".to_string()),
            Change::Deleted("firm".to_string()),
            Change::Deleted("floor".to_string()),
            Change::Deleted("food".to_string()),
            Change::Deleted("foot".to_string()),
            Change::Deleted("force".to_string()),
            Change::Deleted("foreign".to_string()),
            Change::Deleted("four".to_string()),
            Change::Deleted("fudan".to_string()),
            Change::Deleted("full".to_string()),
            Change::Deleted("fund".to_string()),
            Change::Deleted("general".to_string()),
            Change::Deleted("generation".to_string()),
            Change::Deleted("girl".to_string()),
            Change::Deleted("good".to_string()),
            Change::Deleted("hair".to_string()),
            Change::Deleted("hand".to_string()),
            Change::Deleted("hang".to_string()),
            Change::Deleted("happen".to_string()),
            Change::Deleted("harvard".to_string()),
            Change::Deleted("hear".to_string()),
            Change::Deleted("heavy".to_string()),
            Change::Deleted("herself".to_string()),
            Change::Deleted("high".to_string()),
            Change::Deleted("himself".to_string()),
            Change::Deleted("hope".to_string()),
            Change::Deleted("hospital".to_string()),
            Change::Deleted("huge".to_string()),
            Change::Deleted("hundred".to_string()),
            Change::Deleted("husband".to_string()),
            Change::Deleted("important".to_string()),
            Change::Deleted("improve".to_string()),
            Change::Deleted("include".to_string()),
            Change::Deleted("including".to_string()),
            Change::Deleted("increase".to_string()),
            Change::Deleted("indeed".to_string()),
            Change::Deleted("indicate".to_string()),
            Change::Deleted("individual".to_string()),
            Change::Deleted("industry".to_string()),
            Change::Deleted("information".to_string()),
            Change::Deleted("inside".to_string()),
            Change::Deleted("instead".to_string()),
            Change::Deleted("institution".to_string()),
            Change::Deleted("interest".to_string()),
            Change::Deleted("interesting".to_string()),
            Change::Deleted("international".to_string()),
            Change::Deleted("investment".to_string()),
            Change::Deleted("involve".to_string()),
            Change::Deleted("kind".to_string()),
            Change::Deleted("know".to_string()),
            Change::Deleted("knowledge".to_string()),
            Change::Deleted("large".to_string()),
            Change::Deleted("late".to_string()),
            Change::Deleted("later".to_string()),
            Change::Deleted("laugh".to_string()),
            Change::Deleted("lawyer".to_string()),
            Change::Deleted("leader".to_string()),
            Change::Deleted("leave".to_string()),
            Change::Deleted("left".to_string()),
            Change::Deleted("look".to_string()),
            Change::Deleted("lose".to_string()),
            Change::Deleted("loss".to_string()),
            Change::Deleted("magazine".to_string()),
            Change::Deleted("maintain".to_string()),
            Change::Deleted("major".to_string()),
            Change::Deleted("majority".to_string()),
            Change::Deleted("manage".to_string()),
            Change::Deleted("management".to_string()),
            Change::Deleted("many".to_string()),
            Change::Deleted("marriage".to_string()),
            Change::Deleted("material".to_string()),
            Change::Deleted("medical".to_string()),
            Change::Deleted("meeting".to_string()),
            Change::Deleted("member".to_string()),
            Change::Deleted("mention".to_string()),
            Change::Deleted("million".to_string()),
            Change::Deleted("miss".to_string()),
            Change::Deleted("mission".to_string()),
            Change::Deleted("morning".to_string()),
            Change::Deleted("most".to_string()),
            Change::Deleted("mother".to_string()),
            Change::Deleted("mouth".to_string()),
            Change::Deleted("movement".to_string()),
            Change::Deleted("myself".to_string()),
            Change::Deleted("national".to_string()),
            Change::Deleted("nearly".to_string()),
            Change::Deleted("necessary".to_string()),
            Change::Deleted("need".to_string()),
            Change::Deleted("newspaper".to_string()),
            Change::Deleted("north".to_string()),
            Change::Deleted("occur".to_string()),
            Change::Deleted("offer".to_string()),
            Change::Deleted("officer".to_string()),
            Change::Deleted("official".to_string()),
            Change::Deleted("often".to_string()),
            Change::Deleted("onto".to_string()),
            Change::Deleted("operation".to_string()),
            Change::Deleted("opportunity".to_string()),
            Change::Deleted("other".to_string()),
            Change::Deleted("others".to_string()),
            Change::Deleted("outside".to_string()),
            Change::Deleted("oxford".to_string()),
            Change::Deleted("pain".to_string()),
            Change::Deleted("painting".to_string()),
            Change::Deleted("parent".to_string()),
            Change::Deleted("participant".to_string()),
            Change::Deleted("particular".to_string()),
            Change::Deleted("particularly".to_string()),
            Change::Deleted("partner".to_string()),
            Change::Deleted("patient".to_string()),
            Change::Deleted("perform".to_string()),
            Change::Deleted("performance".to_string()),
            Change::Deleted("physical".to_string()),
            Change::Deleted("picture".to_string()),
            Change::Deleted("piece".to_string()),
            Change::Deleted("plan".to_string()),
            Change::Deleted("police".to_string()),
            Change::Deleted("policy".to_string()),
            Change::Deleted("political".to_string()),
            Change::Deleted("politics".to_string()),
            Change::Deleted("poor".to_string()),
            Change::Deleted("popular".to_string()),
            Change::Deleted("population".to_string()),
            Change::Deleted("positive".to_string()),
            Change::Deleted("prepare".to_string()),
            Change::Deleted("president".to_string()),
            Change::Deleted("prevent".to_string()),
            Change::Deleted("price".to_string()),
            Change::Deleted("produce".to_string()),
            Change::Deleted("production".to_string()),
            Change::Deleted("professional".to_string()),
            Change::Deleted("professor".to_string()),
            Change::Deleted("provide".to_string()),
            Change::Deleted("purpose".to_string()),
            Change::Deleted("quality".to_string()),
            Change::Deleted("quickly".to_string()),
            Change::Deleted("quite".to_string()),
            Change::Deleted("race".to_string()),
            Change::Deleted("rather".to_string()),
            Change::Deleted("reality".to_string()),
            Change::Deleted("really".to_string()),
            Change::Deleted("receive".to_string()),
            Change::Deleted("recent".to_string()),
            Change::Deleted("recently".to_string()),
            Change::Deleted("recognize".to_string()),
            Change::Deleted("relate".to_string()),
            Change::Deleted("relationship".to_string()),
            Change::Deleted("religious".to_string()),
            Change::Deleted("represent".to_string()),
            Change::Deleted("republican".to_string()),
            Change::Deleted("require".to_string()),
            Change::Deleted("respond".to_string()),
            Change::Deleted("responsibility".to_string()),
            Change::Deleted("reveal".to_string()),
            Change::Deleted("right".to_string()),
            Change::Deleted("risk".to_string()),
            Change::Deleted("road".to_string()),
            Change::Deleted("role".to_string()),
            Change::Deleted("room".to_string()),
            Change::Deleted("school".to_string()),
            Change::Deleted("seat".to_string()),
            Change::Deleted("seek".to_string()),
            Change::Deleted("seem".to_string()),
            Change::Deleted("sell".to_string()),
            Change::Deleted("senior".to_string()),
            Change::Deleted("serious".to_string()),
            Change::Deleted("several".to_string()),
            Change::Deleted("sexual".to_string()),
            Change::Deleted("shoot".to_string()),
            Change::Deleted("should".to_string()),
            Change::Deleted("shoulder".to_string()),
            Change::Deleted("side".to_string()),
            Change::Deleted("significant".to_string()),
            Change::Deleted("simply".to_string()),
            Change::Deleted("sister".to_string()),
            Change::Deleted("situation".to_string()),
            Change::Deleted("sjtu".to_string()),
            Change::Deleted("skill".to_string()),
            Change::Deleted("social".to_string()),
            Change::Deleted("somebody".to_string()),
            Change::Deleted("someone".to_string()),
            Change::Deleted("sometimes".to_string()),
            Change::Deleted("soon".to_string()),
            Change::Deleted("south".to_string()),
            Change::Deleted("southern".to_string()),
            Change::Deleted("specific".to_string()),
            Change::Deleted("speech".to_string()),
            Change::Deleted("spend".to_string()),
            Change::Deleted("staff".to_string()),
            Change::Deleted("stand".to_string()),
            Change::Deleted("stanford".to_string()),
            Change::Deleted("statement".to_string()),
            Change::Deleted("stay".to_string()),
            Change::Deleted("strategy".to_string()),
            Change::Deleted("street".to_string()),
            Change::Deleted("student".to_string()),
            Change::Deleted("study".to_string()),
            Change::Deleted("subject".to_string()),
            Change::Deleted("successful".to_string()),
            Change::Deleted("such".to_string()),
            Change::Deleted("suddenly".to_string()),
            Change::Deleted("suffer".to_string()),
            Change::Deleted("sure".to_string()),
            Change::Deleted("teach".to_string()),
            Change::Deleted("teacher".to_string()),
            Change::Deleted("team".to_string()),
            Change::Deleted("television".to_string()),
            Change::Deleted("tend".to_string()),
            Change::Deleted("than".to_string()),
            Change::Deleted("their".to_string()),
            Change::Deleted("them".to_string()),
            Change::Deleted("themselves".to_string()),
            Change::Deleted("theory".to_string()),
            Change::Deleted("there".to_string()),
            Change::Deleted("third".to_string()),
            Change::Deleted("those".to_string()),
            Change::Deleted("though".to_string()),
            Change::Deleted("thought".to_string()),
            Change::Deleted("thousand".to_string()),
            Change::Deleted("threat".to_string()),
            Change::Deleted("throughout".to_string()),
            Change::Deleted("tongji".to_string()),
            Change::Deleted("tonight".to_string()),
            Change::Deleted("total".to_string()),
            Change::Deleted("toward".to_string()),
            Change::Deleted("town".to_string()),
            Change::Deleted("traditional".to_string()),
            Change::Deleted("training".to_string()),
            Change::Deleted("travel".to_string()),
            Change::Deleted("treat".to_string()),
            Change::Deleted("treatment".to_string()),
            Change::Deleted("trial".to_string()),
            Change::Deleted("trouble".to_string()),
            Change::Deleted("tsinghua".to_string()),
            Change::Deleted("understand".to_string()),
            Change::Deleted("ustc".to_string()),
            Change::Deleted("usually".to_string()),
            Change::Deleted("various".to_string()),
            Change::Deleted("very".to_string()),
            Change::Deleted("victim".to_string()),
            Change::Deleted("violence".to_string()),
            Change::Deleted("wait".to_string()),
            Change::Deleted("weapon".to_string()),
            Change::Deleted("wear".to_string()),
            Change::Deleted("week".to_string()),
            Change::Deleted("weight".to_string()),
            Change::Deleted("well".to_string()),
            Change::Deleted("western".to_string()),
            Change::Deleted("whether".to_string()),
            Change::Deleted("whole".to_string()),
            Change::Deleted("whom".to_string()),
            Change::Deleted("whose".to_string()),
            Change::Deleted("will".to_string()),
            Change::Deleted("without".to_string()),
            Change::Deleted("woman".to_string()),
            Change::Deleted("worry".to_string()),
            Change::Deleted("would".to_string()),
            Change::Deleted("writer".to_string()),
            Change::Deleted("wrong".to_string()),
            Change::Deleted("year".to_string()),
            Change::Deleted("young".to_string()),
            Change::Deleted("your".to_string()),
            Change::Deleted("yourself".to_string())
        ]
    );
}

#[test]
#[ignore]
#[serial]
fn quick_traverse_unyanked_crates() {
    //    [CrateVersion { dependencies: [Dependency { name: "freetype-rs", required_version: "^0.11", features: [], optional: false, default_features: true, target: None, kind: Some("normal"), package: None }, Dependency { name: "gfx", required_version: "^0.12.2", features: [], optional: false, default_features: true, target: None, kind: Some("normal"), package: None }, Dependency { name: "glutin", required_version: "^0.6", features: [], optional: false, default_features: true, target: None, kind: Some("dev"), package: None }, Dependency { name: "gfx_window_glutin", required_version: "^0.12", features: [], optional: false, default_features: true, target: None, kind: Some("dev"), package: None }] }]
    let (index, _tmp) = make_index();

    let crates = changes_of(&index, REV_ONE_UNYANKED);
    assert_eq!(
        crates,
        vec![Change::Added(CrateVersion {
            name: "gfx_text".to_owned(),
            yanked: false,
            version: "0.13.2".to_owned(),
            dependencies: vec![
                Dependency {
                    name: "freetype-rs".into(),
                    required_version: "^0.11".into(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    target: None,
                    kind: Some("normal".into()),
                    package: None
                },
                Dependency {
                    name: "gfx".into(),
                    required_version: "^0.12.2".into(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    target: None,
                    kind: Some("normal".into()),
                    package: None
                },
                Dependency {
                    name: "glutin".into(),
                    required_version: "^0.6".into(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    target: None,
                    kind: Some("dev".into()),
                    package: None
                },
                Dependency {
                    name: "gfx_window_glutin".into(),
                    required_version: "^0.12".into(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    target: None,
                    kind: Some("dev".into()),
                    package: None
                }
            ],
            features: {
                let mut h = HashMap::new();
                h.insert("default".to_string(), vec!["include-font".to_string()]);
                h.insert("include-font".into(), vec![]);
                h
            },
            checksum: "d0b1240e3627e646f69685ddd3e7d83dd3ff3d586afe83bf3679082028183f2d".into(),
        })]
    );
}

#[test]
#[ignore]
#[serial]
fn quick_traverse_yanked_crates() {
    let (index, _tmp) = make_index();

    let crates = changes_of(&index, REV_ONE_YANKED);
    assert_eq!(
        crates,
        vec![Change::Yanked(CrateVersion {
            name: "sha3".to_owned(),
            yanked: true,
            version: "0.0.0".to_owned(),
            dependencies: Vec::new(),
            features: HashMap::new(),
            checksum: "dbba9d72d3d04e2167fb9c76ce22aed118eb003727bbe59774b9bf3603fa1f43".into(),
        })]
    );
}

#[test]
#[ignore]
#[serial]
fn quick_traverse_added_crates() {
    let (index, _tmp) = make_index();
    assert!(index.changes("foo", REV_ONE_ADDED).is_err());
    assert!(index.changes(REV_ONE_ADDED, "bar").is_err());

    let crates = changes_of(&index, REV_ONE_ADDED);
    assert_eq!(
        crates,
        vec![Change::Added(CrateVersion {
            name: "rpwg".to_owned(),
            yanked: false,
            version: "0.1.0".to_owned(),
            dependencies: vec![
                Dependency {
                    name: "rand".into(),
                    required_version: "^0.3".into(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    target: None,
                    kind: Some("normal".into()),
                    package: None
                },
                Dependency {
                    name: "clap".into(),
                    required_version: "^2.19".into(),
                    features: vec![],
                    optional: false,
                    default_features: true,
                    target: None,
                    kind: Some("normal".into()),
                    package: None
                }
            ],
            features: HashMap::new(),
            checksum: "14437a3702699dba0c49ddc401a0529898e83f8b769348549985a0f4d818d3ca".into(),
        })]
    );
}

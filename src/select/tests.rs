use rstest::rstest;

use super::mock::{MockDice, MockLibrary, now, question, solved, topic, tried};
use super::*;
use crate::questions::Difficulty::{Easy, Hard, Medium};

fn labels() -> Vec<String> {
    ["google", "meta", "dfs", "graphs", "arrays"]
        .map(String::from)
        .to_vec()
}

fn parse(line: &str) -> Result<Request, ParseError> {
    let args: Vec<&str> = line.split_whitespace().collect();
    Request::parse(&args, &labels(), Kind::Need)
}

fn ids(s: &Selection) -> Vec<u32> {
    s.ids.clone()
}

// ── parsing ──────────────────────────────────────────────────────────

#[rstest]
#[case("need google", "google need")]
#[case("need google dfs 3", "3 dfs google need")]
#[case("need google dfs 3", "google 3 need dfs")]
#[case("need google dfs 3", "need dfs google -n 3")]
#[case("google", "need google")]
#[case("google easy hard", "hard google easy")]
#[case("random graphs 2", "graphs 2 random")]
#[case("random graphs 2", "-n 2 random GRAPHS")]
#[case("need need google", "need google")]
#[case("need", "need 1")]
fn word_order_does_not_matter(#[case] a: &str, #[case] b: &str) {
    assert_eq!(parse(a), parse(b));
    assert!(parse(a).is_ok());
}

#[test]
fn filter_words_alone_mean_need() {
    let Ok(Request::Pick {
        kind,
        filter,
        count,
    }) = parse("google dfs 3")
    else {
        panic!()
    };
    assert_eq!(kind, Kind::Need);
    assert_eq!(filter.words(), "dfs google");
    assert_eq!(count, 3);
}

#[rstest]
#[case("3", Request::Ids(vec![3]))]
#[case("3 7 3", Request::Ids(vec![3, 7]))]
#[case("two sum", Request::Search { text: "two sum".into(), count: 1 })]
#[case("two sum -n 2", Request::Search { text: "two sum".into(), count: 2 })]
#[case("google two", Request::Search { text: "google two".into(), count: 1 })]
#[case("3sum", Request::Search { text: "3sum".into(), count: 1 })]
fn ids_and_searches(#[case] line: &str, #[case] expected: Request) {
    assert_eq!(parse(line), Ok(expected));
}

#[rstest]
#[case("", ParseError::Empty)]
#[case("-1", ParseError::BadId("-1".into()))]
#[case("1.5", ParseError::BadId("1.5".into()))]
#[case("need -n 0", ParseError::BadN)]
#[case("need google 0", ParseError::BadCount("0".into()))]
#[case("need google -2", ParseError::BadCount("-2".into()))]
#[case("need 2 3", ParseError::TwoCounts)]
#[case("google 3 -n 2", ParseError::TwoCounts)]
#[case("need random", ParseError::TwoKinds)]
#[case("need foo", ParseError::NotAFilter { word: "foo".into(), kind: "need" })]
#[case("google random two", ParseError::NotAFilter { word: "two".into(), kind: "random" })]
fn rejects(#[case] line: &str, #[case] expected: ParseError) {
    assert_eq!(parse(line), Err(expected));
}

// ── need ─────────────────────────────────────────────────────────────

fn need(library: &MockLibrary, line: &str) -> Selection {
    select(
        &parse(line).unwrap(),
        library,
        &MockDice::default(),
        &Preferences::default(),
        now(),
    )
    .unwrap()
}

#[test]
fn untried_questions_come_mediums_first() {
    let library = MockLibrary::with(vec![
        question(1, Easy, &["arrays"]),
        question(2, Hard, &["arrays"]),
        question(3, Medium, &["arrays"]),
        question(4, Easy, &["arrays"]),
        question(5, Medium, &["arrays"]),
    ]);
    assert_eq!(ids(&need(&library, "need 5")), [3, 5, 1, 4, 2]);
}

#[test]
fn solved_questions_go_last_weakest_first() {
    let library = MockLibrary::with(vec![
        solved(question(1, Medium, &["arrays"]), 1.0, 2),
        solved(question(2, Medium, &["arrays"]), 0.9, 2),
        question(3, Hard, &["arrays"]),
        question(4, Easy, &["arrays"]),
    ]);
    assert_eq!(ids(&need(&library, "need 4")), [4, 3, 2, 1]);
}

#[test]
fn just_tried_questions_wait_a_day() {
    let library = MockLibrary::with(vec![
        tried(question(1, Medium, &["arrays"]), 1),
        tried(question(2, Medium, &["arrays"]), 30),
        question(3, Hard, &["arrays"]),
    ]);
    // #2 was tried yesterday, so it's back in line; #1 an hour ago waits.
    assert_eq!(ids(&need(&library, "need 3")), [2, 3, 1]);
}

#[test]
fn gaps_then_due_reviews_then_new_topics() {
    let mut library = MockLibrary::with(vec![
        solved(question(1, Medium, &["arrays"]), 0.7, 24 * 10), // due
        tried(question(2, Easy, &["dfs"]), 48),
        question(3, Easy, &["dfs"]),
        question(4, Medium, &["dfs"]),
        question(5, Hard, &["graphs"]),
        question(6, Medium, &["graphs"]),
    ]);
    library.topics = vec![
        topic("dfs", 0.0, 1),
        topic("arrays", 0.7, 1),
        topic("graphs", 0.0, 0),
    ];
    let s = need(&library, "need 4");
    assert_eq!(ids(&s), [4, 1, 6, 2]);
    let reasons: Vec<&str> = s.reasons.iter().map(|r| r.reason.as_str()).collect();
    assert_eq!(
        reasons,
        [
            "dfs is a gap (0% mastery)",
            "due for review",
            "graphs not practiced yet",
            "not solved yet"
        ]
    );
}

#[test]
fn filters_narrow_need() {
    let library = MockLibrary::with(vec![
        question(1, Medium, &["arrays", "google"]),
        solved(question(2, Medium, &["dfs", "google"]), 1.0, 5),
        question(3, Hard, &["dfs", "google"]),
        question(4, Medium, &["dfs", "meta"]),
        question(5, Easy, &["dfs", "google"]),
    ]);
    let s = need(&library, "google dfs 5");
    assert_eq!(ids(&s), [5, 3, 2]);
    assert_eq!(s.mode, "need");
    assert_eq!(s.query.as_deref(), Some("dfs google"));
    assert_eq!(s.wanted, 5);
    assert_eq!(ids(&need(&library, "google dfs easy medium 5")), [5, 2]);
}

#[test]
fn the_same_question_isnt_offered_twice_running() {
    // The original bug: `/solve google` gave Two Sum every time.
    let mut library = MockLibrary::with(vec![
        question(1, Easy, &["arrays", "google"]),
        question(2, Medium, &["arrays", "google"]),
        question(3, Medium, &["dfs", "google"]),
    ]);
    assert_eq!(ids(&need(&library, "google")), [2]);
    library.questions[1] = tried(library.questions[1].clone(), 0);
    assert_eq!(ids(&need(&library, "google")), [3]);
    library.questions[2] = solved(library.questions[2].clone(), 1.0, 0);
    assert_eq!(ids(&need(&library, "google")), [1]);
}

// ── random ───────────────────────────────────────────────────────────

#[test]
fn random_draws_from_the_filtered_pool() {
    let library = MockLibrary::with(vec![
        question(1, Medium, &["graphs"]),
        question(2, Medium, &["arrays"]),
        question(3, Hard, &["graphs"]),
        question(4, Easy, &["graphs"]),
    ]);
    let dice = MockDice::rolling(&[2, 0]);
    let s = select(
        &parse("graphs random 2").unwrap(),
        &library,
        &dice,
        &Preferences::default(),
        now(),
    )
    .unwrap();
    // Pool [1, 3, 4]: roll 2 swaps 4 to the front, roll 0 keeps 3 second.
    assert_eq!(ids(&s), [4, 3]);
    assert_eq!(*dice.asked.borrow(), [3, 2]);
    assert_eq!(s.mode, "random");
    assert!(s.reasons.is_empty());
}

#[test]
fn random_never_repeats_and_stops_at_the_pool() {
    let library = MockLibrary::with(vec![
        question(1, Medium, &["graphs"]),
        question(2, Medium, &["graphs"]),
    ]);
    let dice = MockDice::rolling(&[7, 7, 7]);
    let s = select(
        &parse("random 5").unwrap(),
        &library,
        &dice,
        &Preferences::default(),
        now(),
    )
    .unwrap();
    let mut got = ids(&s);
    got.sort();
    assert_eq!(got, [1, 2]);
    assert_eq!(s.wanted, 5);
}

// ── plumbing ─────────────────────────────────────────────────────────

#[test]
fn search_asks_the_library_and_skips_history() {
    let library = MockLibrary {
        matches: vec![9, 8, 7],
        ..MockLibrary::default()
    };
    let s = select(
        &parse("two sum -n 2").unwrap(),
        &library,
        &MockDice::default(),
        &Preferences::default(),
        now(),
    )
    .unwrap();
    assert_eq!(ids(&s), [9, 8]);
    assert_eq!(*library.searched.borrow(), ["two sum"]);
    assert_eq!(library.history_calls.get(), 0);
}

#[test]
fn search_with_no_match_is_an_error() {
    let library = MockLibrary::default();
    let e = select(
        &parse("nothing").unwrap(),
        &library,
        &MockDice::default(),
        &Preferences::default(),
        now(),
    )
    .unwrap_err();
    assert_eq!(e.to_string(), "no question matches `nothing`");
}

#[test]
fn a_filter_nothing_matches_is_an_error() {
    let library = MockLibrary::with(vec![
        question(1, Easy, &["google"]),
        question(2, Hard, &["meta"]),
    ]);
    let e = select(
        &parse("google hard").unwrap(),
        &library,
        &MockDice::default(),
        &Preferences::default(),
        now(),
    )
    .unwrap_err();
    assert_eq!(e.to_string(), "no question matches `google hard`");
}

#[test]
fn history_failures_surface() {
    let library = MockLibrary {
        broken: true,
        ..MockLibrary::with(vec![question(1, Easy, &["google"])])
    };
    let e = select(
        &parse("need").unwrap(),
        &library,
        &MockDice::default(),
        &Preferences::default(),
        now(),
    )
    .unwrap_err();
    assert!(e.to_string().contains("disk on fire"));
    assert_eq!(library.history_calls.get(), 1);
}

#[test]
fn ids_pass_through_untouched() {
    let library = MockLibrary::default();
    let s = select(
        &parse("5 2").unwrap(),
        &library,
        &MockDice::default(),
        &Preferences::default(),
        now(),
    )
    .unwrap();
    assert_eq!(ids(&s), [5, 2]);
    assert_eq!(library.history_calls.get(), 0);
    assert!(library.searched.borrow().is_empty());
}

// ── preferences ──────────────────────────────────────────────────────

#[test]
fn the_configured_strategy_is_the_default() {
    let args = ["google", "2"];
    let Ok(Request::Pick { kind, .. }) = Request::parse(&args, &labels(), Kind::Random) else {
        panic!()
    };
    assert_eq!(kind, Kind::Random);
    // Naming one still wins.
    let args = ["need", "google"];
    let Ok(Request::Pick { kind, .. }) = Request::parse(&args, &labels(), Kind::Random) else {
        panic!()
    };
    assert_eq!(kind, Kind::Need);
}

#[test]
fn the_configured_difficulty_order_is_used() {
    let library = MockLibrary::with(vec![
        question(1, Medium, &["arrays"]),
        question(2, Hard, &["arrays"]),
        question(3, Easy, &["arrays"]),
    ]);
    let prefs = Preferences {
        difficulty_order: vec![Hard, Easy],
        ..Preferences::default()
    };
    let s = select(
        &parse("need 3").unwrap(),
        &library,
        &MockDice::default(),
        &prefs,
        now(),
    )
    .unwrap();
    // Medium isn't listed, so it comes last.
    assert_eq!(ids(&s), [2, 3, 1]);
}

#[rstest]
#[case("", Ok(Preferences::default()))]
#[case(
    "strategy = \"random\"",
    Ok(Preferences { strategy: Kind::Random, ..Preferences::default() })
)]
#[case(
    "difficulty_order = [\"easy\", \"medium\", \"hard\"]",
    Ok(Preferences { difficulty_order: vec![Easy, Medium, Hard], ..Preferences::default() })
)]
#[case("strategy = \"hardest\"", Err("unknown variant `hardest`"))]
#[case("difficulty_order = [\"tricky\"]", Err("unknown variant `tricky`"))]
fn reads_from_config(#[case] toml_text: &str, #[case] expected: Result<Preferences, &str>) {
    let got: Result<Preferences, _> = toml::from_str(toml_text);
    match expected {
        Ok(p) => assert_eq!(got.unwrap(), p),
        Err(msg) => assert!(got.unwrap_err().to_string().contains(msg)),
    }
}

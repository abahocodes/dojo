use rstest::rstest;

use super::mock::{MockDice, MockLibrary, asked, now, question, solved, topic, tried};
use super::*;
use crate::questions::Difficulty::{Easy, Hard, Medium};

fn vocab() -> Vocabulary {
    let set = |words: &[&str]| words.iter().map(|w| w.to_string()).collect();
    Vocabulary {
        tags: set(&["dfs", "graphs", "arrays", "two-pointers"]),
        companies: set(&["google", "meta", "amazon", "medium"]),
        title_words: set(&["two", "sum", "median", "of", "arrays"]),
    }
}

fn parse(line: &str) -> Result<Request, ParseError> {
    let args: Vec<&str> = line.split_whitespace().collect();
    Request::parse(&args, &vocab(), Kind::Need)
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
#[case("medium", "med")]
#[case("medium", "--difficulty medium")]
#[case("medium", "-d medium")]
#[case("medium", "-d=medium")]
#[case("google dfs", "--tag dfs --company google")]
#[case("google dfs", "-c google -t dfs")]
#[case("dfs graphs", "--tag dfs,graphs")]
#[case("dfs graphs", "-t dfs -t graphs")]
#[case("easy hard", "-d easy,hard")]
#[case("random google", "--random -c google")]
#[case("need google 3", "--need google --count 3")]
#[case("need google 3", "-c google 3")]
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
    select(&parse(line).unwrap(), library, &MockDice::default(), now()).unwrap()
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
fn most_asked_questions_come_first() {
    let library = MockLibrary::with(vec![
        question(1, Medium, &["arrays"]), // no company known: 0
        question(2, Medium, &["arrays", "google"]),
        asked(question(3, Hard, &["arrays"]), "amazon", 5),
        asked(question(4, Easy, &["arrays"]), "meta", 3),
    ]);
    // 5, then the 3s by difficulty (medium before easy), then unknown.
    assert_eq!(ids(&need(&library, "need 4")), [3, 2, 4, 1]);
}

#[test]
fn a_company_filter_judges_frequency_by_that_company() {
    let library = MockLibrary::with(vec![
        asked(
            asked(question(1, Medium, &["arrays"]), "google", 2),
            "amazon",
            5,
        ),
        asked(question(2, Medium, &["arrays"]), "google", 4),
    ]);
    // Overall #1 is asked more (Amazon 5), but Google asks #2 more.
    assert_eq!(ids(&need(&library, "need 2")), [1, 2]);
    assert_eq!(ids(&need(&library, "google 2")), [2, 1]);
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
fn each_gap_topic_gets_its_own_question() {
    let mut library = MockLibrary::with(vec![
        asked(question(1, Medium, &["dfs", "graphs"]), "google", 5),
        question(2, Medium, &["graphs"]),
        question(3, Medium, &["dfs"]),
        tried(question(4, Easy, &["dfs", "graphs"]), 48),
    ]);
    library.topics = vec![topic("dfs", 0.0, 1), topic("graphs", 0.0, 1)];
    let s = need(&library, "need 2");
    // #1 is the best for both; graphs gets the next best rather than none.
    assert_eq!(ids(&s), [1, 2]);
    assert_eq!(s.reasons[1].reason, "graphs is a gap (0% mastery)");
}

#[test]
fn each_new_topic_gets_its_own_question() {
    let mut library = MockLibrary::with(vec![
        asked(question(1, Medium, &["dfs", "graphs"]), "google", 5),
        question(2, Medium, &["graphs"]),
        question(3, Medium, &["dfs"]),
    ]);
    library.topics = vec![topic("dfs", 0.0, 0), topic("graphs", 0.0, 0)];
    let s = need(&library, "need 2");
    // Both topics' best is #1; dfs takes it and graphs gets the next best.
    assert_eq!(ids(&s), [1, 2]);
    assert_eq!(s.reasons[1].reason, "graphs not practiced yet");
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

#[test]
fn the_open_question_isnt_picked_again() {
    // Mid-question, `/solve amazon 3` again: the open one is still
    // unfinished, so it looks untried, but it's being left behind.
    let mut library = MockLibrary::with(vec![
        asked(question(1, Medium, &["arrays"]), "amazon", 5),
        asked(question(2, Medium, &["arrays"]), "amazon", 3),
        asked(question(3, Easy, &["arrays"]), "amazon", 3),
    ]);
    library.open = Some(1);
    assert_eq!(ids(&need(&library, "amazon 3")), [2, 3]);
    let s = select(
        &parse("random amazon 3").unwrap(),
        &library,
        &MockDice::default(),
        now(),
    )
    .unwrap();
    assert!(!s.ids.contains(&1));
    // Asked for by id, it restarts.
    assert_eq!(ids(&need(&library, "1")), [1]);
}

#[test]
fn the_open_question_stays_when_its_all_that_matches() {
    let mut library = MockLibrary::with(vec![
        question(1, Medium, &["dfs"]),
        question(2, Easy, &["arrays"]),
    ]);
    library.open = Some(1);
    assert_eq!(ids(&need(&library, "dfs")), [1]);
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
    let s = select(&parse("graphs random 2").unwrap(), &library, &dice, now()).unwrap();
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
    let s = select(&parse("random 5").unwrap(), &library, &dice, now()).unwrap();
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
    let Ok(Request::Pick { kind, .. }) = Request::parse(&args, &vocab(), Kind::Random) else {
        panic!()
    };
    assert_eq!(kind, Kind::Random);
    // Naming one still wins.
    let args = ["need", "google"];
    let Ok(Request::Pick { kind, .. }) = Request::parse(&args, &vocab(), Kind::Random) else {
        panic!()
    };
    assert_eq!(kind, Kind::Need);
}

#[rstest]
#[case("", Ok(Preferences::default()))]
#[case(
    "strategy = \"random\"",
    Ok(Preferences { strategy: Kind::Random })
)]
#[case("strategy = \"hardest\"", Err("unknown variant `hardest`"))]
fn reads_from_config(#[case] toml_text: &str, #[case] expected: Result<Preferences, &str>) {
    let got: Result<Preferences, _> = toml::from_str(toml_text);
    match expected {
        Ok(p) => assert_eq!(got.unwrap(), p),
        Err(msg) => assert!(got.unwrap_err().to_string().contains(msg)),
    }
}

// ── options and cut-off words ────────────────────────────────────────

/// A company called medium is reached with --company; bare `medium` is
/// the difficulty.
#[test]
fn options_say_which_meaning() {
    let Ok(Request::Pick { filter, .. }) = parse("--company medium") else {
        panic!()
    };
    assert!(filter.matches(&asked(question(1, Hard, &[]), "medium", 3)));
    assert!(!filter.matches(&question(2, Medium, &[])));

    let Ok(Request::Pick { filter, .. }) = parse("medium") else {
        panic!()
    };
    assert!(filter.matches(&question(2, Medium, &[])));
    assert!(!filter.matches(&asked(question(1, Hard, &[]), "medium", 3)));
}

/// --tag dfs doesn't match a company that happens to be named dfs.
#[test]
fn tag_option_only_matches_tags() {
    let Ok(Request::Pick { filter, .. }) = parse("--tag dfs") else {
        panic!()
    };
    assert!(filter.matches(&question(1, Easy, &["dfs"])));
    assert!(!filter.matches(&asked(question(2, Easy, &[]), "dfs", 3)));
}

#[rstest]
#[case("--search two sum", "two sum")]
#[case("-s google", "google")]
#[case("--search medi", "medi")]
#[case("--search=median", "median")]
#[case("two --search sum", "two sum")]
fn search_option_searches_titles(#[case] line: &str, #[case] text: &str) {
    assert_eq!(
        parse(line),
        Ok(Request::Search {
            text: text.into(),
            count: 1
        })
    );
}

/// `/solve medi` opened the hard Median questions: a word cut off partway
/// now asks what was meant instead of searching titles.
#[rstest]
#[case("medi", "medium (difficulty), medium (company)")]
#[case("mediu", "medium (difficulty), medium (company)")]
#[case("google medi", "medium (difficulty), medium (company)")]
#[case("nee", "need (strategy)")]
#[case("goo", "google (company)")]
#[case("gra 2", "graphs (tag)")]
fn cut_off_words_ask(#[case] line: &str, #[case] options: &str) {
    match parse(line) {
        Err(ParseError::CutOff { options: o, .. }) => assert_eq!(o, options),
        other => panic!("{other:?}"),
    }
}

/// Whole title words still search, even when they start a tag
/// (`two` / two-pointers), and so do words that start nothing.
#[rstest]
#[case("two")]
#[case("median")]
#[case("trap rain")]
#[case("two goo")]
#[case("median medi")]
fn whole_words_search(#[case] line: &str) {
    assert!(
        matches!(parse(line), Ok(Request::Search { .. })),
        "{:?}",
        parse(line)
    );
}

#[rstest]
#[case("--company googel", "`googel` isn't a company  ·  Tab lists them")]
#[case("-c goo", "`goo` isn't a company  ·  did you mean google?")]
#[case("-d med", "`med` isn't a difficulty  ·  did you mean medium?")]
#[case("-t", "-t needs a value  ·  Tab lists them")]
#[case("--company", "--company needs a value  ·  Tab lists them")]
#[case("--search", "--search needs a value  ·  Tab lists them")]
#[case("--hard", "`--hard` isn't an option  ·  /help solve")]
#[case("--need=3", "`--need=3` isn't an option  ·  /help solve")]
#[case(
    "--search two -c google",
    "--search looks at titles alone; drop need, random and filters"
)]
#[case(
    "-c google two",
    "`two` isn't a tag, company or difficulty  ·  /solve need google dfs 2  ·  /help solve"
)]
#[case("-n", "-n needs a positive number")]
#[case("--count x", "-n needs a positive number")]
fn option_mistakes(#[case] line: &str, #[case] message: &str) {
    assert_eq!(parse(line).unwrap_err().to_string(), message);
}

/// With an option, a lone number is a count, not an id.
#[test]
fn numbers_after_options_are_counts() {
    let Ok(Request::Pick { count, .. }) = parse("-c google 3") else {
        panic!()
    };
    assert_eq!(count, 3);
    assert_eq!(
        parse("--random 3").map(|r| matches!(r, Request::Pick { count: 3, .. })),
        Ok(true)
    );
}

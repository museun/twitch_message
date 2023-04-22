#![no_main]
use libfuzzer_sys::fuzz_target;

use itertools::Itertools;
fuzz_target!(|data: &str| { fun(data) });

// notes on ord: https://doc.rust-lang.org/std/cmp/trait.Ord.html
// Implementations must be consistent with the PartialOrd implementation, and ensure max, min, and clamp are consistent with cmp:
//
// partial_cmp(a, b) == Some(cmp(a, b)).
// max(a, b) == max_by(a, b, cmp) (ensured by the default implementation).
// min(a, b) == min_by(a, b, cmp) (ensured by the default implementation).
// For a.clamp(min, max), see the method docs (ensured by the default implementation).
//
// It’s easy to accidentally make cmp and partial_cmp disagree by deriving some of the traits and manually implementing others.
// Corollaries
//
// From the above and the requirements of PartialOrd, it follows that < defines a strict total order. This means that for all a, b and c:
//
// exactly one of a < b, a == b or a > b is true; and
// < is transitive: a < b and b < c implies a < c. The same must hold for both == and >.

pub fn fun(data: &str) {
    if data.is_empty()
        || data.chars().any(|c: char| {
            !c.is_ascii_alphanumeric() && c != '_' && c != '-' && c != '/' && c != ','
        })
    {
        return;
    }
    let Ok(badges) = std::panic::catch_unwind(|| twitch_message::parse_badges(data).collect::<Vec<_>>()) else {
        return;
    };
    for badge in &badges {
        if badge.set_id.as_str().is_empty() || badge.id.as_str().is_empty() {
            return;
        }
    }
    if badges.len() == 2 {
        for (a, b) in [(0, 1), (1, 0)] {
            //dbg!(badges[a].partial_cmp(&badges[b]), Some(badges[a].cmp(&badges[b])));
            assert!(badges[a].partial_cmp(&badges[b]) == Some(badges[a].cmp(&badges[b])))
        }
    }
    if badges.len() != 3 {
        return;
    }

    for vec in [0, 1, 2].iter().permutations(3) {
        let a = &badges[*vec[0]];
        let b = &badges[*vec[1]];
        let c = &badges[*vec[2]];
        let res = [a > b, a == b, a < b];
        assert!(
            res.iter().filter(|x| **x).count() == 1,
            "{a:?} {b:?} {c:?} {:?}",
            res
        );
        let res = [c > b, c == b, c < b];
        assert!(
            res.iter().filter(|x| **x).count() == 1,
            "{a:?} {b:?} {c:?} {:?}",
            res
        );

        if a < b && b < c {
            assert!(a < c, "{a:?} < {b:?} && {b:?} < {c:?}", a = a);
        } else if a == b && b == c {
            assert!(a == c, "{a:?} == {b:?} && {b:?} == {c:?}", a = a);
        } else if a > b && b > c {
            assert!(a > c, "{a:?} > {b:?} && {b:?} > {c:?}", a = a);
        }
    }
}

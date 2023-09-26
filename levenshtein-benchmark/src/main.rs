//
// Levenshtein distance benchmarks
//
use levenshtein::levenshtein as levenshtein_rs;
use levenshtein_automata::{LevenshteinAutomatonBuilder};
use edit_distance::edit_distance;
use levenshtein_diff as levenshtein_diff;
use strsim::{levenshtein as strsim_levenshtein};

fn main() {

    //let string1 = "kitten";
    //let string2 = "sitting";
    let string1 = "日本語は難しいです";
    let string2 = "英語は難しいです";
    let test_times = 1000;

    println!("*levenshtein lib test* {}/{}", string1, string2);

    //levenshtein-rs
    {
        let _exec_time = Elapsed::start("levenshtein_rs");
        for _n in 1..=test_times {
            let _res = levenshtein_rs(string1, string2);
        }
        println!("distance by levenshtein-rs lib: {}", levenshtein_rs(string1, string2));
    }

    //edit_distance
    {
        let _exec_time = Elapsed::start("edit_distance");
        for _n in 1..=test_times {
            let _res = edit_distance(string1, string2);
        }
        println!("distance by edit_distance::edit_distance: {}", edit_distance(string1, string2));
    }

    //levenshtein_diff
    {
        let _exec_time = Elapsed::start("levenshtein_diff");
        for _n in 1..=test_times {
            let (_distance, _) = levenshtein_diff::distance(string1.as_bytes(), string2.as_bytes());
        }
        let (distance, _) = levenshtein_diff::distance(string1.as_bytes(), string2.as_bytes());
        println!("distance by levenshtein-diff: {:?}", distance);
    }

    //strsim
    {
        let _exec_time = Elapsed::start("strsim_levenshtein");
        for _n in 1..=test_times {
            let _res = strsim_levenshtein(string1, string2);
        }
        println!("distance by strsim_levenshtein: {:?}", strsim_levenshtein(string1, string2));
    }

    //levenshtein_automata
    {
        let _exec_time = Elapsed::start("levenshtein_automata");
        for _n in 1..=test_times {
            let lev_automaton_builder = LevenshteinAutomatonBuilder::new(2, true);
            let dfa = lev_automaton_builder.build_dfa(string1);
            let mut state = dfa.initial_state();
            for &b in string2.as_bytes() {
                state = dfa.transition(state, b);
            }
        }
        let lev_automaton_builder = LevenshteinAutomatonBuilder::new(2, true);
        let dfa = lev_automaton_builder.build_dfa(string1);
        let mut state = dfa.initial_state();
        for &b in string2.as_bytes() {
            state = dfa.transition(state, b);
        }
        println!("distance by levenshtein_automata: {:?}", dfa.distance(state));
    }

}

// https://stackoverflow.com/a/65493006
struct Elapsed(&'static str, std::time::SystemTime);

impl Drop for Elapsed {
    fn drop(&mut self) {
        println!(
            "operation {} finished for {} ms",
            self.0,
            self.1.elapsed().unwrap_or_default().as_millis()
        );
    }
}

impl Elapsed {
    pub fn start(op: &'static str) -> Elapsed {
        let now = std::time::SystemTime::now();

        Elapsed(op, now)
    }
}
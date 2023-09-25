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

    println!("*levenshtein lib test* {}/{}", string1, string2);

    //levenshtein-rs
    println!("distance by levenshtein-rs lib: {}", levenshtein_rs(string1, string2));

    //levenshtein_automata
    let lev_automaton_builder = LevenshteinAutomatonBuilder::new(2, true);
    let dfa = lev_automaton_builder.build_dfa(string1);
    let mut state = dfa.initial_state();
        for &b in string2.as_bytes() {
        state = dfa.transition(state, b);
    }
    println!("distance by levenshtein_automata: {:?}", dfa.distance(state));

    //edit_distance
    println!("distance by edit_distance::edit_distance: {}", edit_distance(string1, string2));

    //levenshtein_diff
    let (distance, _) = levenshtein_diff::distance(string1.as_bytes(), string2.as_bytes());
    println!("distance by levenshtein-diff: {:?}", distance);

    //strsim
    println!("distance by strsim_levenshtein: {:?}", strsim_levenshtein(string1, string2));

}
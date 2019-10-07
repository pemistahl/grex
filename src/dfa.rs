/*
 * Copyright © 2019 Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::ast::{concatenate, repeat_zero_or_more_times, union, Expression};
use linked_hash_set::LinkedHashSet;
use linked_list::LinkedList;
use ndarray::{Array1, Array2};
use petgraph::graph::NodeIndex;
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::Dfs;
use petgraph::Direction;
use std::collections::{BTreeSet, HashSet};
use unicode_segmentation::UnicodeSegmentation;

type State = NodeIndex<u32>;

pub struct DFA {
    alphabet: BTreeSet<String>,
    graph: StableGraph<HashSet<usize>, String>,
    initial_state: State,
    final_state_indices: HashSet<usize>,
}

impl DFA {
    pub fn new() -> Self {
        let mut graph = StableGraph::new();
        let initial_state = graph.add_node(HashSet::new());
        Self {
            alphabet: BTreeSet::new(),
            graph,
            initial_state,
            final_state_indices: HashSet::new(),
        }
    }

    pub fn from(strs: Vec<String>) -> Self {
        let mut dfa = Self::new();
        for elem in strs {
            dfa.insert(elem);
        }
        dfa.minimize();
        dfa
    }

    fn insert(&mut self, s: String) {
        let mut current_state = self.initial_state;
        for grapheme in UnicodeSegmentation::graphemes(s.as_str(), true) {
            self.alphabet.insert(grapheme.to_string());
            current_state = self.get_next_state(current_state, &grapheme);
        }
        self.final_state_indices.insert(current_state.index());
    }

    fn get_next_state(&mut self, current_state: State, label: &str) -> State {
        match self.find_next_state(current_state, label) {
            Some(next_state) => next_state,
            None => self.add_new_state(current_state, label),
        }
    }

    fn find_next_state(&self, current_state: State, label: &str) -> Option<State> {
        for next_state in self.graph.neighbors(current_state) {
            let edge_idx = self.graph.find_edge(current_state, next_state).unwrap();
            let edge_label = self.graph.edge_weight(edge_idx).unwrap();
            if edge_label == label {
                return Some(next_state);
            }
        }
        None
    }

    fn add_new_state(&mut self, current_state: State, label: &str) -> State {
        let next_state = self.graph.add_node(HashSet::new());
        self.graph
            .add_edge(current_state, next_state, label.to_string());
        next_state
    }

    pub fn minimize(&mut self) {
        let mut p = self.get_initial_partition();
        let mut w = LinkedHashSet::new();
        for elem in p.iter() {
            w.insert(elem.clone());
        }

        let mut p_cursor = p.cursor();

        while !w.is_empty() {
            let a = w.pop_front().unwrap();

            for edge_label in self.alphabet.iter() {
                let x = self.get_parent_states(&a, edge_label);
                let mut replacements = vec![];

                while let Some(y) = p_cursor.peek_next() {
                    let i = x
                        .intersection(y)
                        .map(|elem| *elem)
                        .collect::<LinkedHashSet<State>>();

                    if i.is_empty() {
                        p_cursor.next();
                        continue;
                    }

                    let d = y
                        .difference(&x)
                        .map(|elem| *elem)
                        .collect::<LinkedHashSet<State>>();

                    if d.is_empty() {
                        p_cursor.next();
                        continue;
                    }

                    replacements.push((y.clone(), i.clone(), d.clone()));

                    p_cursor.remove();
                    p_cursor.insert(i);
                    p_cursor.next();
                    p_cursor.insert(d);
                    p_cursor.prev();
                }

                p_cursor.reset();

                for (y, i, d) in replacements {
                    if w.contains(&y) {
                        w.remove(&y);
                        w.insert(i);
                        w.insert(d);
                    } else if i.len() <= d.len() {
                        w.insert(i);
                    } else {
                        w.insert(d);
                    }
                }
            }
        }

        self.recreate_graph(p);
    }

    fn get_initial_partition(&self) -> LinkedList<LinkedHashSet<State>> {
        let (final_states, non_final_states): (LinkedHashSet<State>, LinkedHashSet<State>) = self
            .graph
            .node_indices()
            .partition(|&state| !self.final_state_indices.contains(&state.index()));

        linked_list![final_states, non_final_states]
    }

    fn get_parent_states(&self, a: &LinkedHashSet<State>, label: &str) -> LinkedHashSet<State> {
        let mut x = LinkedHashSet::new();
        for state in self.graph.node_indices() {
            let next_states = self.graph.neighbors(state);
            for next_state in next_states {
                if a.contains(&next_state) {
                    let edge = self.graph.find_edge(state, next_state).unwrap();
                    let edge_label = self.graph.edge_weight(edge).unwrap();
                    if edge_label == label {
                        x.insert(state);
                        break;
                    }
                }
            }
        }
        x
    }

    fn recreate_graph(&mut self, p: LinkedList<LinkedHashSet<State>>) {
        let mut graph = StableGraph::<HashSet<usize>, String>::new();
        let mut final_state_indices = HashSet::new();

        for equivalence_class in p.iter() {
            let state_label = equivalence_class
                .iter()
                .map(|&state| state.index())
                .collect::<HashSet<usize>>();

            graph.add_node(state_label);
        }

        for equivalence_class in p.iter() {
            let old_source_state = *equivalence_class.iter().next().unwrap();
            let new_source_state = self.get_target_state(&graph, &old_source_state);

            for old_target_state in self.graph.neighbors(old_source_state) {
                let edge = self
                    .graph
                    .find_edge(old_source_state, old_target_state)
                    .unwrap();

                let edge_label = self.graph.edge_weight(edge).unwrap();
                let new_target_state = self.get_target_state(&graph, &old_target_state);

                graph.add_edge(new_source_state, new_target_state, edge_label.clone());

                if self.final_state_indices.contains(&old_target_state.index()) {
                    final_state_indices.insert(new_target_state.index());
                }
            }
        }
        self.initial_state = self.get_initial_state(&graph);
        self.final_state_indices = final_state_indices;
        self.graph = graph;
    }

    fn get_target_state(
        &self,
        graph: &StableGraph<HashSet<usize>, String>,
        source_state: &State,
    ) -> State {
        graph
            .node_indices()
            .find(|&state| {
                graph
                    .node_weight(state)
                    .unwrap()
                    .contains(&source_state.index())
            })
            .unwrap()
    }

    fn get_initial_state(&self, graph: &StableGraph<HashSet<usize>, String>) -> State {
        graph
            .node_indices()
            .find(|&state| graph.edges_directed(state, Direction::Incoming).count() == 0)
            .unwrap()
    }

    fn is_final_state(&self, state: &State) -> bool {
        self.final_state_indices.contains(&state.index())
    }

    pub fn to_regex(&self) -> String {
        let state_count = self.graph.node_count();

        let mut a = Array2::<Option<Expression>>::default((state_count, state_count));
        let mut b = Array1::<Option<Expression>>::default(state_count);

        let mut depth_first_search = Dfs::new(&self.graph, self.initial_state);
        let mut states = vec![];

        while let Some(state) = depth_first_search.next(&self.graph) {
            states.push(state);
        }

        for (i, state) in states.iter().enumerate() {
            if self.is_final_state(state) {
                b[i] = Some(Expression::new_literal(""));
            }

            for edge in self.graph.edges_directed(*state, Direction::Outgoing) {
                let edge_label = edge.weight();
                let literal = Expression::new_literal(edge_label);
                let j = states
                    .iter()
                    .position(|&it| it.index() == edge.target().index())
                    .unwrap();

                a[(i, j)] = if a[(i, j)].is_some() {
                    union(&a[(i, j)], &Some(literal))
                } else {
                    Some(literal)
                }
            }
        }

        for n in (0..state_count).rev() {
            if a[(n, n)].is_some() {
                b[n] = concatenate(&repeat_zero_or_more_times(&a[(n, n)]), &b[n]);
                for j in 0..n {
                    a[(n, j)] = concatenate(&repeat_zero_or_more_times(&a[(n, n)]), &a[(n, j)]);
                }
            }

            for i in 0..n {
                if a[(i, n)].is_some() {
                    b[i] = union(&b[i], &concatenate(&a[(i, n)], &b[n]));
                    for j in 0..n {
                        a[(i, j)] = union(&a[(i, j)], &concatenate(&a[(i, n)], &a[(n, j)]));
                    }
                }
            }
        }

        b[0].as_ref().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::collections::HashMap;

    #[test]
    fn ensure_correctness_of_regular_expressions() {
        for (input, expected_output) in params() {
            assert_eq!(DFA::from(input).to_regex(), expected_output);
        }
    }

    #[test]
    fn ensure_regular_expressions_match_input() {
        for (input, expected_output) in params() {
            let re = Regex::new(expected_output).unwrap();
            for input_str in input {
                assert_match(&re, input_str);
            }
        }
    }

    fn assert_match(re: &Regex, text: &str) {
        assert!(re.is_match(text), "\"{}\" does not match regex", text);
    }

    fn params() -> HashMap<Vec<&'static str>, &'static str> {
        hashmap![
            vec!["a", "b", "c"] => "a|b|c",
            vec!["a", "b", "bc"] => "bc?|a",
            vec!["a", "b", "bcd"] => "b(cd)?|a",
            vec!["a", "ab", "abc"] => "a(bc?)?",
            vec!["ac", "bc"] => "(a|b)c",
            vec!["ab", "ac"] => "a(b|c)",
            vec!["abx", "cdx"] => "(ab|cd)x",
            vec!["abd", "acd"] => "a(b|c)d",
            vec!["abc", "abcd"] => "abcd?",
            vec!["abc", "abcde"] => "abc(de)?",
            vec!["2.0-3.5", "2.5-6.0"] => "2\\.(0\\-3\\.5|5\\-6\\.0)"
        ]
    }
}

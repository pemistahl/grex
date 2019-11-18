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

use std::collections::{BTreeSet, HashMap, HashSet};

use crate::str::GraphemeCluster;
use itertools::Itertools;
use linked_list::LinkedList;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::{Edges, StableGraph};
use petgraph::visit::Dfs;
use petgraph::{Directed, Direction};
use unicode_segmentation::UnicodeSegmentation;

type State = NodeIndex<u32>;
type StateLabel = String;
type EdgeLabel = GraphemeCluster;

pub(crate) struct DFA {
    alphabet: BTreeSet<String>,
    graph: StableGraph<StateLabel, EdgeLabel>,
    initial_state: State,
    final_state_indices: HashSet<usize>,
}

impl DFA {
    pub(crate) fn from(graphemes_vecs: Vec<Vec<GraphemeCluster>>) -> Self {
        let mut dfa = Self::new();
        for graphemes in graphemes_vecs {
            dfa.insert(graphemes);
        }
        dfa.minimize();
        dfa
    }

    pub(crate) fn state_count(&self) -> usize {
        self.graph.node_count()
    }

    pub(crate) fn states_in_depth_first_order(&self) -> Vec<State> {
        let mut depth_first_search = Dfs::new(&self.graph, self.initial_state);
        let mut states = vec![];
        while let Some(state) = depth_first_search.next(&self.graph) {
            states.push(state);
        }
        states
    }

    pub(crate) fn outgoing_edges(&self, state: State) -> Edges<GraphemeCluster, Directed> {
        self.graph.edges_directed(state, Direction::Outgoing)
    }

    pub(crate) fn is_final_state(&self, state: State) -> bool {
        self.final_state_indices.contains(&state.index())
    }

    fn new() -> Self {
        let mut graph = StableGraph::new();
        let initial_state = graph.add_node("".to_string());
        Self {
            alphabet: BTreeSet::new(),
            graph,
            initial_state,
            final_state_indices: HashSet::new(),
        }
    }

    fn insert(&mut self, graphemes: Vec<GraphemeCluster>) {
        let mut current_state = self.initial_state;

        for grapheme in graphemes.iter() {
            self.alphabet.insert(grapheme.value.clone());
            current_state = self.get_next_state(current_state, &grapheme);
        }
        self.final_state_indices.insert(current_state.index());
    }

    fn get_next_state(&mut self, current_state: State, edge_label: &GraphemeCluster) -> State {
        match self.find_next_state(current_state, edge_label) {
            Some(next_state) => next_state,
            None => self.add_new_state(current_state, edge_label),
        }
    }

    fn find_next_state(&self, current_state: State, edge_label: &GraphemeCluster) -> Option<State> {
        for next_state in self.graph.neighbors(current_state) {
            let edge_idx = self.graph.find_edge(current_state, next_state).unwrap();
            let current_label = self.graph.edge_weight(edge_idx).unwrap();
            if current_label == edge_label {
                return Some(next_state);
            }
        }
        None
    }

    fn add_new_state(&mut self, current_state: State, edge_label: &GraphemeCluster) -> State {
        let next_state = self.graph.add_node("".to_string());
        self.graph.add_edge(
            current_state,
            next_state,
            GraphemeCluster::from(&edge_label.value[..]),
        );
        next_state
    }

    #[allow(clippy::many_single_char_names)]
    fn minimize(&mut self) {
        let mut p = self.get_initial_partition();
        let mut w = p.iter().cloned().collect_vec();
        let mut p_cursor = p.cursor();

        while !w.is_empty() {
            let a = w.drain(0..1).next().unwrap();

            for edge_label in self.alphabet.iter() {
                let x = self.get_parent_states(&a, edge_label);
                let mut replacements = vec![];

                while let Some(y) = p_cursor.peek_next() {
                    let i = x.intersection(y).copied().collect::<HashSet<State>>();

                    if i.is_empty() {
                        p_cursor.next();
                        continue;
                    }

                    let d = y.difference(&x).copied().collect::<HashSet<State>>();

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
                        let idx = w.iter().position(|it| it == &y).unwrap();
                        w.remove(idx);
                        w.push(i);
                        w.push(d);
                    } else if i.len() <= d.len() {
                        w.push(i);
                    } else {
                        w.push(d);
                    }
                }
            }
        }

        self.recreate_graph(p.iter().filter(|&it| !it.is_empty()).collect_vec());
    }

    fn get_initial_partition(&self) -> LinkedList<HashSet<State>> {
        let (final_states, non_final_states): (HashSet<State>, HashSet<State>) = self
            .graph
            .node_indices()
            .partition(|&state| !self.final_state_indices.contains(&state.index()));

        linked_list![final_states, non_final_states]
    }

    fn get_parent_states(&self, a: &HashSet<State>, label: &str) -> HashSet<State> {
        let mut x = HashSet::new();

        for &state in a {
            let direct_parent_states = self.graph.neighbors_directed(state, Direction::Incoming);
            for parent_state in direct_parent_states {
                let edge = self.graph.find_edge(parent_state, state).unwrap();
                let grapheme_cluster = self.graph.edge_weight(edge).unwrap();
                if grapheme_cluster.value == label {
                    x.insert(parent_state);
                    break;
                }
            }
        }
        x
    }

    fn recreate_graph(&mut self, p: Vec<&HashSet<State>>) {
        let mut graph = StableGraph::<StateLabel, EdgeLabel>::new();
        let mut final_state_indices = HashSet::new();
        let mut state_mappings = HashMap::new();
        let mut new_initial_state: Option<NodeIndex> = None;

        for equivalence_class in p.iter() {
            let new_state = graph.add_node("".to_string());

            for old_state in equivalence_class.iter() {
                if self.initial_state == *old_state {
                    new_initial_state = Some(new_state);
                }
                state_mappings.insert(*old_state, new_state);
            }
        }

        for equivalence_class in p.iter() {
            let old_source_state = *equivalence_class.iter().next().unwrap();
            let new_source_state = state_mappings.get(&old_source_state).unwrap();

            for old_target_state in self.graph.neighbors(old_source_state) {
                let edge = self
                    .graph
                    .find_edge(old_source_state, old_target_state)
                    .unwrap();

                let grapheme_cluster = self.graph.edge_weight(edge).unwrap().clone();
                let new_target_state = state_mappings.get(&old_target_state).unwrap();

                graph.add_edge(
                    *new_source_state,
                    *new_target_state,
                    grapheme_cluster.clone(),
                );

                if self.final_state_indices.contains(&old_target_state.index()) {
                    final_state_indices.insert(new_target_state.index());
                }
            }
        }
        self.initial_state = new_initial_state.unwrap();
        self.final_state_indices = final_state_indices;
        self.graph = graph;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_count() {
        let mut dfa = DFA::new();
        assert_eq!(dfa.state_count(), 1);

        dfa.insert(graphemes("abcd"));
        assert_eq!(dfa.state_count(), 5);
    }

    #[test]
    fn test_is_final_state() {
        let dfa = DFA::from(vec![graphemes("abcd")]);

        let intermediate_state = State::new(3);
        assert_eq!(dfa.is_final_state(intermediate_state), false);

        let final_state = State::new(4);
        assert_eq!(dfa.is_final_state(final_state), true);
    }

    #[test]
    fn test_outgoing_edges() {
        let dfa = DFA::from(vec![graphemes("abcd"), graphemes("abxd")]);
        let state = State::new(2);
        let mut edges = dfa.outgoing_edges(state);

        let first_edge = edges.next();
        assert!(first_edge.is_some());
        assert_eq!(first_edge.unwrap().weight(), &grapheme("c"));

        let second_edge = edges.next();
        assert!(second_edge.is_some());
        assert_eq!(second_edge.unwrap().weight(), &grapheme("x"));

        let third_edge = edges.next();
        assert!(third_edge.is_none());
    }

    #[test]
    fn test_states_in_depth_first_order() {
        let dfa = DFA::from(vec![graphemes("abcd"), graphemes("axyz")]);
        let states = dfa.states_in_depth_first_order();
        assert_eq!(states.len(), 7);

        let first_state = states.get(0).unwrap();
        let mut edges = dfa.outgoing_edges(*first_state);
        assert_eq!(edges.next().unwrap().weight(), &grapheme("a"));
        assert!(edges.next().is_none());

        let second_state = states.get(1).unwrap();
        edges = dfa.outgoing_edges(*second_state);
        assert_eq!(edges.next().unwrap().weight(), &grapheme("b"));
        assert_eq!(edges.next().unwrap().weight(), &grapheme("x"));
        assert!(edges.next().is_none());

        let third_state = states.get(2).unwrap();
        edges = dfa.outgoing_edges(*third_state);
        assert_eq!(edges.next().unwrap().weight(), &grapheme("y"));
        assert!(edges.next().is_none());

        let fourth_state = states.get(3).unwrap();
        edges = dfa.outgoing_edges(*fourth_state);
        assert_eq!(edges.next().unwrap().weight(), &grapheme("z"));
        assert!(edges.next().is_none());

        let fifth_state = states.get(4).unwrap();
        edges = dfa.outgoing_edges(*fifth_state);
        assert!(edges.next().is_none());

        let sixth_state = states.get(5).unwrap();
        edges = dfa.outgoing_edges(*sixth_state);
        assert_eq!(edges.next().unwrap().weight(), &grapheme("c"));
        assert!(edges.next().is_none());

        let seventh_state = states.get(6).unwrap();
        edges = dfa.outgoing_edges(*seventh_state);
        assert_eq!(edges.next().unwrap().weight(), &grapheme("d"));
        assert!(edges.next().is_none());
    }

    #[test]
    fn test_minimization_algorithm() {
        let mut dfa = DFA::new();
        assert_eq!(dfa.graph.node_count(), 1);
        assert_eq!(dfa.graph.edge_count(), 0);

        dfa.insert(graphemes("abcd"));
        assert_eq!(dfa.graph.node_count(), 5);
        assert_eq!(dfa.graph.edge_count(), 4);

        dfa.insert(graphemes("abxd"));
        assert_eq!(dfa.graph.node_count(), 7);
        assert_eq!(dfa.graph.edge_count(), 6);

        dfa.minimize();
        assert_eq!(dfa.graph.node_count(), 5);
        assert_eq!(dfa.graph.edge_count(), 5);
    }

    #[test]
    fn test_dfa_constructor() {
        let dfa = DFA::from(vec![graphemes("abcd"), graphemes("abxd")]);
        assert_eq!(dfa.graph.node_count(), 5);
        assert_eq!(dfa.graph.edge_count(), 5);
    }

    fn grapheme(s: &str) -> GraphemeCluster {
        graphemes(s).first().unwrap().clone()
    }

    fn graphemes(s: &str) -> Vec<GraphemeCluster> {
        UnicodeSegmentation::graphemes(s, true)
            .map(|it| GraphemeCluster::from(it))
            .collect_vec()
    }
}

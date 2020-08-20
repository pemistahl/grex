/*
 * Copyright © 2019-2020 Peter M. Stahl pemistahl@gmail.com
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

use crate::char::{Grapheme, GraphemeCluster, GraphemeOverlapState};
use crate::regexp::RegExpConfig;
use itertools::Itertools;
use petgraph::dot::Dot;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::{Edges, StableGraph};
use petgraph::visit::{Dfs, EdgeRef, NodeIndexable};
use petgraph::{Directed, Direction};
use std::cmp::max;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter, Result};

type State = NodeIndex<u32>;
type EdgeLabel = Grapheme;

#[derive(PartialEq, Clone, Debug, Copy)]
enum StateStatus {
    Intermediate,
    Accept,
}

#[derive(Clone)]
struct StateLabel {
    description: String,
    status: StateStatus,
}

#[derive(Clone)]
pub struct DFA {
    alphabet: BTreeSet<Grapheme>,
    graph: StableGraph<StateLabel, EdgeLabel>,
    initial_state: State,
    config: RegExpConfig,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum InputState<T> {
    Both(T, T),
    Left(T),
}

struct DFASubtractionBuilder<'a> {
    left: &'a DFA,
    right: &'a DFA,
    result: DFA,
    new_nodes: HashMap<InputState<usize>, State>,
    queue: VecDeque<InputState<usize>>,
    enqueued: HashSet<InputState<usize>>,
}

impl DFA {
    pub(crate) fn from(grapheme_clusters: Vec<GraphemeCluster>, config: &RegExpConfig) -> Self {
        let mut dfa = Self::new(config);
        for cluster in grapheme_clusters {
            dfa.insert(cluster);
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

    pub(crate) fn outgoing_edges(&self, state: State) -> Edges<Grapheme, Directed> {
        self.graph.edges_directed(state, Direction::Outgoing)
    }

    pub(crate) fn is_final_state(&self, state: State) -> bool {
        self.graph[state].status == StateStatus::Accept
    }

    pub(crate) fn subtract(&self, other: &Self) -> Option<Self> {
        DFASubtractionBuilder::subtract(self, other)
    }

    #[allow(dead_code)]
    fn println(&self, comment: &str) {
        println!("{}: {}", comment, Dot::new(&self.graph));
    }

    fn new(config: &RegExpConfig) -> Self {
        let mut graph = StableGraph::new();
        let initial_state = graph.add_node(StateLabel {
            description: "initial".to_string(),
            status: StateStatus::Intermediate,
        });
        Self {
            alphabet: BTreeSet::new(),
            graph,
            initial_state,
            config: config.clone(),
        }
    }

    fn insert(&mut self, cluster: GraphemeCluster) {
        let mut current_state = self.initial_state;

        for grapheme in cluster.graphemes() {
            self.alphabet.insert(grapheme.clone());
            current_state = self.get_next_state(current_state, grapheme);
        }
        self.graph[current_state].status = StateStatus::Accept;
    }

    fn get_next_state(&mut self, current_state: State, edge_label: &Grapheme) -> State {
        match self
            .graph
            .edges_directed(current_state, Direction::Outgoing)
            .find(|edge| *edge.weight() == *edge_label)
        {
            Some(edge) => edge.target(),
            None => self.add_new_state(current_state, edge_label),
        }
    }

    fn add_new_state(&mut self, current_state: State, edge_label: &Grapheme) -> State {
        let next_state = self.graph.add_node(StateLabel {
            description: "".to_string(),
            status: StateStatus::Intermediate,
        });
        self.graph
            .add_edge(current_state, next_state, edge_label.clone());
        next_state
    }

    fn minimize(&mut self) {
        self.remove_dead_ends();
        self.deduplicate_equivalent_states();
        self.deduplicate_redundant_edges();
    }

    pub fn remove_dead_ends(&mut self) {
        let mut queue: VecDeque<State> = self.graph.node_indices().collect();
        let initial_size = self.graph.node_count();

        while let Some(state) = queue.pop_front() {
            if !self.graph.contains_node(state) || state.index() == self.initial_state.index() {
                continue;
            }

            let has_outgoing = self.graph.edges_directed(state, Direction::Outgoing).next() != None;
            let has_incoming = self.graph.edges_directed(state, Direction::Incoming).next() != None;
            let is_final = self.graph[state].status == StateStatus::Accept;

            if (has_outgoing || is_final) && has_incoming {
                continue;
            }

            queue.append(&mut self.graph.neighbors(state).collect());
            self.graph.remove_node(state);
        }

        if self.graph.node_count() != initial_size {
            self.alphabet = self
                .graph
                .edge_indices()
                .map(|e| self.graph[e].clone())
                .collect();
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn deduplicate_equivalent_states(&mut self) {
        let mut p = self.get_initial_partition();
        let mut w = p.iter().cloned().collect_vec();

        while !w.is_empty() {
            let a = w.drain(0..1).next().unwrap();

            for edge_label in self.alphabet.iter() {
                let x = self.get_parent_states(&a, edge_label);
                let mut replacements = vec![];
                let mut is_replacement_needed = true;
                let mut start_idx = 0;

                while is_replacement_needed {
                    for (idx, y) in p.iter().enumerate().skip(start_idx) {
                        if x.intersection(y).count() == 0 || y.difference(&x).count() == 0 {
                            is_replacement_needed = false;
                            continue;
                        }

                        let i = x.intersection(y).copied().collect::<HashSet<State>>();
                        let d = y.difference(&x).copied().collect::<HashSet<State>>();

                        is_replacement_needed = true;
                        start_idx = idx;

                        replacements.push((y.clone(), i, d));

                        break;
                    }

                    if is_replacement_needed {
                        let (_, i, d) = replacements.last().unwrap();

                        p.remove(start_idx);
                        p.insert(start_idx, i.clone());
                        p.insert(start_idx + 1, d.clone());
                    }
                }

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

    fn deduplicate_redundant_edges(&mut self) {
        for state in self.graph.node_indices().collect_vec() {
            let mut next_nodes = HashMap::new();
            for edge in self.graph.edges_directed(state, Direction::Outgoing) {
                next_nodes
                    .entry(edge.target().index())
                    .or_insert_with(HashMap::new)
                    .entry(edge.weight().value())
                    .or_insert_with(Vec::new)
                    .push(edge.id());
            }

            for (next_index, mut grapheme_map) in next_nodes {
                for edges in grapheme_map.values_mut() {
                    let graphemes = edges
                        .iter()
                        .map(|edge| self.graph[*edge].clone())
                        .collect_vec();
                    let new_graphemes = self.merge_graphemes(graphemes);
                    if new_graphemes.len() == edges.len() {
                        continue;
                    }
                    for edge in edges {
                        self.graph.remove_edge(*edge);
                    }
                    for grapheme in new_graphemes.iter().rev() {
                        self.graph.add_edge(
                            state,
                            self.graph.from_index(next_index),
                            grapheme.clone(),
                        );
                    }
                }
            }
        }
    }

    fn merge_graphemes(&self, mut graphemes: Vec<Grapheme>) -> Vec<Grapheme> {
        graphemes.sort_by_key(|grapheme| (grapheme.minimum(), grapheme.maximum()));

        let mut result = Vec::new();
        for grapheme in graphemes {
            match result.last_mut() {
                None => result.push(grapheme),
                Some(last) => {
                    if last.maximum() + 1 >= grapheme.minimum() {
                        *last = Grapheme::new(
                            last.chars().clone(),
                            last.minimum(),
                            max(last.maximum(), grapheme.maximum()),
                            &self.config,
                        );
                    } else {
                        result.push(grapheme);
                    }
                }
            }
        }
        result
    }

    fn get_initial_partition(&self) -> Vec<HashSet<State>> {
        let (final_states, non_final_states): (HashSet<State>, HashSet<State>) = self
            .graph
            .node_indices()
            .partition(|&state| self.graph[state].status != StateStatus::Intermediate);

        vec![non_final_states, final_states]
    }

    fn get_parent_states(&self, a: &HashSet<State>, label: &Grapheme) -> HashSet<State> {
        let mut x = HashSet::new();

        for &state in a {
            let direct_parent_states = self.graph.neighbors_directed(state, Direction::Incoming);
            for parent_state in direct_parent_states {
                let edge = self.graph.find_edge(parent_state, state).unwrap();
                let grapheme = self.graph.edge_weight(edge).unwrap();
                if grapheme.value() == label.value()
                    && (grapheme.maximum() == label.maximum()
                        || grapheme.minimum() == label.minimum())
                {
                    x.insert(parent_state);
                    break;
                }
            }
        }
        x
    }

    fn recreate_graph(&mut self, equivalence_classes: Vec<&HashSet<State>>) {
        let mut graph = StableGraph::<StateLabel, EdgeLabel>::new();
        let mut state_mappings = HashMap::new();
        let mut new_initial_state: Option<NodeIndex> = None;

        for equivalence_class in equivalence_classes.iter() {
            let new_state = graph.add_node(StateLabel {
                description: format!(
                    "{:?}",
                    equivalence_class
                        .iter()
                        .map(|state| state.index())
                        .collect_vec()
                ),
                status: self.graph[*equivalence_class.iter().next().unwrap()].status,
            });

            for old_state in equivalence_class.iter() {
                if self.initial_state == *old_state {
                    new_initial_state = Some(new_state);
                    graph[new_state].description.push_str(" initial");
                }
                state_mappings.insert(*old_state, new_state);
            }
        }

        for equivalence_class in equivalence_classes.iter() {
            let new_source_state = state_mappings
                .get(equivalence_class.iter().next().unwrap())
                .unwrap();

            let mut added_edges: HashSet<(Grapheme, usize)> = HashSet::new();

            for edge in equivalence_class
                .iter()
                .flat_map(|from_state| self.graph.edges_directed(*from_state, Direction::Outgoing))
            {
                let grapheme = edge.weight();
                let new_target_state = state_mappings.get(&edge.target()).unwrap();

                if added_edges.insert((grapheme.clone(), new_target_state.index())) {
                    graph.add_edge(*new_source_state, *new_target_state, grapheme.clone());
                }
            }
        }
        self.initial_state = new_initial_state.unwrap();
        self.graph = graph;
    }
}

impl Display for StateLabel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ({:?})", self.description, self.status)
    }
}

impl<T> InputState<T> {
    fn map_any<R, F, G>(&self, f: F, g: G) -> InputState<R>
    where
        F: FnOnce(&T) -> R,
        G: FnOnce(&T) -> R,
    {
        match self {
            Self::Left(x) => InputState::Left(f(x)),
            Self::Both(x, y) => InputState::Both(f(x), g(y)),
        }
    }
}

impl<'a> DFASubtractionBuilder<'a> {
    fn subtract(left: &'a DFA, right: &'a DFA) -> Option<DFA> {
        if left.config != right.config {
            None
        } else {
            let mut builder = Self {
                left,
                right,
                result: DFA::new(&left.config),
                new_nodes: HashMap::new(),
                queue: VecDeque::new(),
                enqueued: HashSet::new(),
            };
            builder.build();
            Some(builder.result)
        }
    }

    fn output_state_status(&self, idx: InputState<usize>) -> StateStatus {
        match idx.map_any(
            |state_left| self.left.graph[self.left.graph.from_index(*state_left)].status,
            |state_right| self.right.graph[self.right.graph.from_index(*state_right)].status,
        ) {
            InputState::Both(_, StateStatus::Accept) => StateStatus::Intermediate,
            InputState::Both(StateStatus::Accept, _) => StateStatus::Accept,
            InputState::Left(StateStatus::Accept) => StateStatus::Accept,
            _ => StateStatus::Intermediate,
        }
    }

    fn input_to_output_state_space(&mut self, idx: &InputState<usize>) -> State {
        if *idx
            == InputState::Both(
                self.left.initial_state.index(),
                self.right.initial_state.index(),
            )
        {
            self.result.initial_state
        } else {
            match self.new_nodes.get(&idx) {
                Some(state) => *state,
                None => {
                    let new_state = self.result.graph.add_node(StateLabel {
                        description: format!("{:?}", idx),
                        status: self.output_state_status(idx.clone()),
                    });
                    self.new_nodes.insert(idx.clone(), new_state);
                    new_state
                }
            }
        }
    }

    fn add_edge(&mut self, new_from: State, grapheme: Grapheme, input: &InputState<usize>) {
        let new_to = self.input_to_output_state_space(input);
        self.result.graph.add_edge(new_from, new_to, grapheme);
        if !self.enqueued.contains(&input) {
            self.queue.push_back(input.clone());
            self.enqueued.insert(input.clone());
        }
    }

    fn overlap_all_grapheme(
        &mut self,
        left: Grapheme,
        rights: Vec<(Grapheme, usize)>,
    ) -> (Vec<Grapheme>, Vec<(Grapheme, usize)>) {
        let mut overlaps = Vec::new();
        let mut unoverlapped = vec![left];

        for (right_grapheme, right_idx) in rights {
            let mut new_unoverlapped = Vec::new();
            for left_grapheme in unoverlapped {
                for (new_grapheme, overlap_state) in
                    left_grapheme.overlap_with(&right_grapheme).unwrap()
                {
                    match overlap_state {
                        GraphemeOverlapState::Right => continue,
                        GraphemeOverlapState::Overlap => overlaps.push((new_grapheme, right_idx)),
                        GraphemeOverlapState::Left => new_unoverlapped.push(new_grapheme),
                    }
                }
            }
            unoverlapped = new_unoverlapped;
        }
        (unoverlapped, overlaps)
    }

    fn build(&mut self) {
        let input_initial = InputState::Both(
            self.left.initial_state.index(),
            self.right.initial_state.index(),
        );
        self.queue.push_back(input_initial.clone());
        self.enqueued.insert(input_initial);

        while let Some(idx) = self.queue.pop_front() {
            let new_from = self.input_to_output_state_space(&idx);

            match idx.map_any(
                |left_idx| {
                    self.left
                        .outgoing_edges(self.left.graph.from_index(*left_idx))
                },
                |right_idx| {
                    self.right
                        .outgoing_edges(self.right.graph.from_index(*right_idx))
                },
            ) {
                InputState::Left(left_edges) => {
                    for edge in left_edges {
                        self.add_edge(
                            new_from,
                            edge.weight().clone(),
                            &InputState::Left(edge.target().index()),
                        );
                    }
                }
                InputState::Both(left_edges, right_edges) => {
                    let right_edges_vec = right_edges.collect_vec();

                    for edge_left in left_edges {
                        let grapheme_left = edge_left.weight();
                        let edge_left_to = edge_left.target().index();

                        let matching_edges = right_edges_vec
                            .iter()
                            .filter(|edge| edge.weight().value() == grapheme_left.value())
                            .collect_vec();

                        let (unoverlapped, overlapped) = self.overlap_all_grapheme(
                            grapheme_left.clone(),
                            matching_edges
                                .iter()
                                .map(|edge| (edge.weight().clone(), edge.target().index()))
                                .collect_vec(),
                        );

                        for new_grapheme in unoverlapped {
                            self.add_edge(
                                new_from,
                                new_grapheme.clone(),
                                &InputState::Left(edge_left_to),
                            );
                        }

                        for (new_grapheme, edge_right_to) in overlapped {
                            self.add_edge(
                                new_from,
                                new_grapheme.clone(),
                                &InputState::Both(edge_left_to, edge_right_to),
                            );
                        }
                    }
                }
            }
        }

        self.result.alphabet = self
            .result
            .graph
            .edge_indices()
            .map(|e| self.result.graph[e].clone())
            .collect();

        self.result.minimize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_count() {
        let config = RegExpConfig::new();
        let mut dfa = DFA::new(&config);
        assert_eq!(dfa.state_count(), 1);

        dfa.insert(GraphemeCluster::from("abcd", &RegExpConfig::new()));
        assert_eq!(dfa.state_count(), 5);
    }

    #[test]
    fn test_is_final_state() {
        let config = RegExpConfig::new();
        let dfa = DFA::from(
            vec![GraphemeCluster::from("abcd", &RegExpConfig::new())],
            &config,
        );

        let intermediate_state = State::new(3);
        assert_eq!(dfa.is_final_state(intermediate_state), false);

        let final_state = State::new(4);
        assert_eq!(dfa.is_final_state(final_state), true);
    }

    #[test]
    fn test_subtraction() {
        let config = RegExpConfig::new();
        let dfa1 = DFA::from(
            vec![
                GraphemeCluster::from("abcd", &RegExpConfig::new()),
                GraphemeCluster::from("abcf", &RegExpConfig::new()),
            ],
            &config,
        );
        let dfa2 = DFA::from(
            vec![GraphemeCluster::from("abcd", &RegExpConfig::new())],
            &config,
        );

        let dfa_option = dfa1.subtract(&dfa2);
        assert!(dfa_option.is_some());

        let dfa = dfa_option.unwrap();
        assert_eq!(dfa.state_count(), 5);
    }

    #[test]
    fn test_outgoing_edges() {
        let config = RegExpConfig::new();
        let dfa = DFA::from(
            vec![
                GraphemeCluster::from("abcd", &RegExpConfig::new()),
                GraphemeCluster::from("abxd", &RegExpConfig::new()),
            ],
            &config,
        );
        let state = State::new(2);
        let mut edges = dfa.outgoing_edges(state);

        let first_edge = edges.next();
        assert!(first_edge.is_some());
        assert_eq!(
            first_edge.unwrap().weight(),
            &Grapheme::from("c", &RegExpConfig::new())
        );

        let second_edge = edges.next();
        assert!(second_edge.is_some());
        assert_eq!(
            second_edge.unwrap().weight(),
            &Grapheme::from("x", &RegExpConfig::new())
        );

        let third_edge = edges.next();
        assert!(third_edge.is_none());
    }

    #[test]
    fn test_states_in_depth_first_order() {
        let config = RegExpConfig::new();
        let dfa = DFA::from(
            vec![
                GraphemeCluster::from("abcd", &RegExpConfig::new()),
                GraphemeCluster::from("axyz", &RegExpConfig::new()),
            ],
            &config,
        );
        let states = dfa.states_in_depth_first_order();
        assert_eq!(states.len(), 7);

        let first_state = states.get(0).unwrap();
        let mut edges = dfa.outgoing_edges(*first_state);
        assert_eq!(
            edges.next().unwrap().weight(),
            &Grapheme::from("a", &RegExpConfig::new())
        );
        assert!(edges.next().is_none());

        let second_state = states.get(1).unwrap();
        edges = dfa.outgoing_edges(*second_state);
        assert_eq!(
            edges.next().unwrap().weight(),
            &Grapheme::from("b", &RegExpConfig::new())
        );
        assert_eq!(
            edges.next().unwrap().weight(),
            &Grapheme::from("x", &RegExpConfig::new())
        );
        assert!(edges.next().is_none());

        let third_state = states.get(2).unwrap();
        edges = dfa.outgoing_edges(*third_state);
        assert_eq!(
            edges.next().unwrap().weight(),
            &Grapheme::from("y", &RegExpConfig::new())
        );
        assert!(edges.next().is_none());

        let fourth_state = states.get(3).unwrap();
        edges = dfa.outgoing_edges(*fourth_state);
        assert_eq!(
            edges.next().unwrap().weight(),
            &Grapheme::from("z", &RegExpConfig::new())
        );
        assert!(edges.next().is_none());

        let fifth_state = states.get(4).unwrap();
        edges = dfa.outgoing_edges(*fifth_state);
        assert!(edges.next().is_none());

        let sixth_state = states.get(5).unwrap();
        edges = dfa.outgoing_edges(*sixth_state);
        assert_eq!(
            edges.next().unwrap().weight(),
            &Grapheme::from("c", &RegExpConfig::new())
        );
        assert!(edges.next().is_none());

        let seventh_state = states.get(6).unwrap();
        edges = dfa.outgoing_edges(*seventh_state);
        assert_eq!(
            edges.next().unwrap().weight(),
            &Grapheme::from("d", &RegExpConfig::new())
        );
        assert!(edges.next().is_none());
    }

    #[test]
    fn test_minimization_algorithm() {
        let config = RegExpConfig::new();
        let mut dfa = DFA::new(&config);
        assert_eq!(dfa.graph.node_count(), 1);
        assert_eq!(dfa.graph.edge_count(), 0);

        dfa.insert(GraphemeCluster::from("abcd", &RegExpConfig::new()));
        assert_eq!(dfa.graph.node_count(), 5);
        assert_eq!(dfa.graph.edge_count(), 4);

        dfa.insert(GraphemeCluster::from("abxd", &RegExpConfig::new()));
        assert_eq!(dfa.graph.node_count(), 7);
        assert_eq!(dfa.graph.edge_count(), 6);

        dfa.minimize();
        assert_eq!(dfa.graph.node_count(), 5);
        assert_eq!(dfa.graph.edge_count(), 5);
    }

    #[test]
    fn test_dfa_constructor() {
        let config = RegExpConfig::new();
        let dfa = DFA::from(
            vec![
                GraphemeCluster::from("abcd", &RegExpConfig::new()),
                GraphemeCluster::from("abxd", &RegExpConfig::new()),
            ],
            &config,
        );
        assert_eq!(dfa.graph.node_count(), 5);
        assert_eq!(dfa.graph.edge_count(), 5);
    }
}

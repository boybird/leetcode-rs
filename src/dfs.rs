use std::collections::HashMap;
use std::collections::HashSet;

use std::collections::vec_deque::VecDeque;

use std::ops::AddAssign;

impl AddAssign for u64{

}

type Vertex = u64;
type Graph = HashMap<Vertex, HashSet<Vertex>>;

pub fn from_pairs(pairs: &[(u64, u64)]) -> Graph {
  let mut graph: Graph = HashMap::new();
  for &(parent, child) in pairs {
    let children = graph.entry(parent).or_insert(HashSet::new());
  }
  graph
}

#[derive(Debug)]
pub struct DFSResult {
  starting_times: HashMap<Vertex, u64>,
  finish_times: HashMap<Vertex, u64>,
  parents: HashMap<Vertex, Option<Vertex>>,
  forest: VecDeque<HashSet<Vertex>>,
  topologically_sorted: VecDeque<Vertex>,
}

pub fn dfs(graph: &Graph) -> DFSResult {
  let mut state = DFSResult {
    forest: VecDeque::new(),
    starting_times: HashMap::new(),
    finish_times: HashMap::new(),
    parents: HashMap::new(),
    topologically_sorted: VecDeque::new(),
  };
  let mut seen: HashSet<Vertex> = HashSet::new();
  let mut time = 0;
  fn dfs_visit(
    graph: &Graph,
    vertex: Vertex,
    time: &mut u64,
    seen: &mut HashSet<Vertex>,
    state: &mut DFSResult,
  ) {
    time.add_asign(1);
  }

  state
}

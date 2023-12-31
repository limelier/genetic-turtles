use crate::genetic::definitions::{
    Generation, Individual, Parents, INDIVIDUALS_PER_METHOD_AND_DEPTH, MAX_GEN_DEPTH,
    MIN_GEN_DEPTH, POPULATION_SIZE, P_GROW_LEAF,
};
use crate::trees::definitions::Node;
use crate::trees::random::random_useful_leaf;
use crate::vm::definitions::{BinaryOperation, UnaryOperation};
use rand::Rng;

#[derive(Copy, Clone, PartialEq)]
pub enum Method {
    Full,
    Grow,
}

impl Generation {
    /// Generate a generation from scratch, using ramped half-and-half
    /// Creates individuals with a max depth from MIN_DEPTH to MAX_DEPTH,
    /// generating INDIVIDUALS_PER_METHOD_AND_DEPTH of them with each method per depth.
    pub fn random() -> Self {
        let mut population = Vec::with_capacity(POPULATION_SIZE);
        for depth in MIN_GEN_DEPTH..=MAX_GEN_DEPTH {
            for _ in 0..INDIVIDUALS_PER_METHOD_AND_DEPTH {
                population.push(Individual {
                    tree: generate(Method::Grow, depth),
                    result: None,
                });
                population.push(Individual {
                    tree: generate(Method::Full, depth),
                    result: None,
                });
            }
        }

        Generation {
            population,
            best_index: None,
            worst_index: None,
        }
    }

    /// Create a generation by keeping some individuals and crossing over others to create new ones
    pub fn from_old(
        generation: &Generation,
        kept_indices: &Vec<usize>,
        parent_pairs: &Vec<Parents>,
    ) -> Generation {
        let mut population = Vec::with_capacity(POPULATION_SIZE);
        for idx in kept_indices {
            population.push(Individual {
                tree: generation.population[*idx].tree.clone(),
                result: None,
            });
        }

        population.append(&mut generation.crossover(parent_pairs));

        Generation {
            population,
            best_index: None,
            worst_index: None,
        }
    }
}

pub fn generate(method: Method, max_depth: usize) -> Node {
    recurse(method, max_depth, 0)
}

fn recurse(method: Method, max_depth: usize, current_depth: usize) -> Node {
    let mut rng = rand::thread_rng();

    if current_depth == max_depth || (method == Method::Grow && rng.gen_bool(P_GROW_LEAF)) {
        random_useful_leaf()
    } else {
        random_function(method, max_depth, current_depth)
    }
}

fn random_function(method: Method, max_depth: usize, current_depth: usize) -> Node {
    let mut rng = rand::thread_rng();

    match rng.gen_range(0..=7) {
        0 => Node::Unary(
            rng.gen::<UnaryOperation>(),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        1 => Node::Binary(
            rng.gen::<BinaryOperation>(),
            Box::from(recurse(method, max_depth, current_depth + 1)),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        2 => Node::Compare(
            Box::from(recurse(method, max_depth, current_depth + 1)),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        3 => Node::Store(
            rng.gen::<u8>(),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        4 => Node::If(
            Box::from(recurse(method, max_depth, current_depth + 1)),
            Box::from(recurse(method, max_depth, current_depth + 1)),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        5 => Node::While(
            Box::from(recurse(method, max_depth, current_depth + 1)),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        6 => Node::Then(
            Box::from(recurse(method, max_depth, current_depth + 1)),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        7 => Node::Repeat(
            Box::from(recurse(method, max_depth, current_depth + 1)),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        _ => unreachable!(),
    }
}

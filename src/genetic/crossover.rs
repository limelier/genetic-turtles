use crate::genetic::definitions::{Generation, Individual, Parents, POPULATION_SIZE};
use crate::trees::definitions::Node;

impl Generation {
    pub fn crossover(&self, parent_pairs: &Vec<Parents>) -> Vec<Individual> {
        let mut new_individuals = Vec::with_capacity(POPULATION_SIZE);

        for &Parents { stock, scion } in parent_pairs {
            let stock = &self.population[stock].tree;
            let scion = &self.population[scion].tree;

            let tree = crossover(stock, scion);
            new_individuals.push(Individual { tree, result: None })
        }

        new_individuals
    }
}

fn crossover(stock: &Node, scion: &Node) -> Node {
    let mut stock = stock.clone();
    let (stock_point, stock_point_depth) = stock.get_weighted_node_mut();

    // randomly descend as many levels as the stock point is deep, *then* choose a random node
    let scion_point = scion
        .randomly_descend(stock_point_depth)
        .get_weighted_node();

    let mut new_branch = scion_point.clone();
    std::mem::swap(stock_point, &mut new_branch);

    stock
}

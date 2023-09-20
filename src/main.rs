use genetic_programming::meta::train::train_many_silent;
use genetic_programming::simulator::definitions::BlockSpace;

fn main() {
    let mut target = BlockSpace::default();
    // simple example: line
    for i in 0..8 {
        target[i][1][0] = 1;
    }

    let target = target; // remove mutability
    let individuals = train_many_silent(&target, 30);

    let mut idx_sum = 0.0;
    let mut depth_sum = 0;
    for individual in &individuals {
        let dice_index = individual.result.unwrap().dice_index;
        let depth = individual.tree.get_max_depth();
        idx_sum += dice_index;
        depth_sum += depth;
        println!("{} {}", dice_index, depth);
        dbg!(&individual.tree);
    }
    let num = individuals.len() as f64;
    println!("{} {}", idx_sum / num, (depth_sum as f64) / num);
}

use std::collections::HashMap;
use std::io::Write;
use rand::Rng;
use std::fs::File;
use chrono;

pub struct FullyConnectedGraph {
    pub nb_nodes: usize,
    pub cost: Vec<Vec<u32>>,
}

impl FullyConnectedGraph {
    /// Generate a fully connected graph with nb_nodes nodes.
    /// Each node is placed on a grid of size nb_nodes x nb_nodes.
    /// The cost of each edge is computed as the euclidean distance
    /// between the two nodes on the grid plane.
    pub fn generate_random_graph(
        nb_nodes: usize
    ) -> FullyConnectedGraph {
        let mut cost = vec![vec![0; nb_nodes]; nb_nodes];

        let mut coordinates_to_node_index: HashMap<(u32, u32), u32> = HashMap::new();

        // generate random coordinates for each node
        for i in 0..nb_nodes {
            let mut coordinates_are_selected = false;
            while !coordinates_are_selected {
                let random_x: u32 = rand::thread_rng().gen_range(0..=nb_nodes).try_into().unwrap();
                let random_y: u32 = rand::thread_rng().gen_range(0..=nb_nodes).try_into().unwrap();

                if coordinates_to_node_index.contains_key(&(random_x, random_y)) {
                    continue;
                } else {
                    coordinates_to_node_index.insert((random_x, random_y), i as u32);
                    coordinates_are_selected = true;
                }
            }
        }

        for i in 0..nb_nodes {
            for j in 0..nb_nodes {
                let x = i as f32;
                let y = j as f32;
                cost[i][j] = ((x - y) * (x - y) + (x + y) * (x + y)).sqrt() as u32;
            }
        }
        FullyConnectedGraph {
            nb_nodes: nb_nodes,
            cost: cost,
        }
    }

    pub fn save_to_file(&self, filepath: &str) {
        let mut file = File::create(filepath).unwrap();
        file.write_all(format!("{}\n", self.nb_nodes).as_bytes()).unwrap();
        for i in 0..self.nb_nodes {
            for j in 0..self.nb_nodes {
                file.write_all(format!("{} ", self.cost[i][j]).as_bytes()).unwrap();
            }
            file.write_all("\n".as_bytes()).unwrap();
        }
    }
    
}

fn main() {
    let dir_path = "generated";
    let timestamp = chrono::Utc::now().format("%Y-%m-%d-%H-%M-%S").to_string();

    let graph: FullyConnectedGraph = FullyConnectedGraph::generate_random_graph(10);
    let output_filepath = format!("{}/fully_connected_graph_{}.txt", dir_path, timestamp);
    graph.save_to_file(&output_filepath);

    // generate an image of the graph
    // display the graph as a MST, with only the edges that are in the MST
    let output_image_filepath = format!("{}/fully_connected_graph_{}.png", dir_path, timestamp);
    // TODO: complete
}

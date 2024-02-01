use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use petgraph::Undirected;
use rand::Rng;
use chrono;
use petgraph::graph::{Graph, UnGraph, NodeIndex};
use petgraph::algo::min_spanning_tree;
use petgraph::visit::EdgeRef;
use image::RgbImage;
use imageproc::drawing;
use svg::node::element::{Circle, Line, Text};
use svg::Document;

pub struct FullyConnectedGraph {
    pub nb_nodes: usize,
    pub cost: Vec<Vec<u32>>, // index is node index
    pub coordinates: Vec<(u32, u32)>, // index is node index
}

impl FullyConnectedGraph {
    pub fn generate_random_graph(nb_nodes: usize) -> FullyConnectedGraph {
        let mut cost = vec![vec![0; nb_nodes]; nb_nodes];
        let mut coordinates = vec![(0, 0); nb_nodes];
        let mut coordinates_to_node_index: HashMap<(u32, u32), usize> = HashMap::new();

        for i in 0..nb_nodes {
            let mut node_coordinates_are_selected = false;
            while !node_coordinates_are_selected {
                let random_x: u32 = rand::thread_rng().gen_range(0..nb_nodes as u32);
                let random_y: u32 = rand::thread_rng().gen_range(0..nb_nodes as u32);

                if coordinates_to_node_index.contains_key(&(random_x, random_y)) {
                    continue;
                } else {
                    coordinates_to_node_index.insert((random_x, random_y), i);
                    coordinates[i] = (random_x, random_y);
                    node_coordinates_are_selected = true;
                }
            }
        }

        for (i, &coord1) in coordinates.iter().enumerate() {
            for (j, &coord2) in coordinates.iter().enumerate() {
                // Note that when i == j, cost[i][j] is 0 (fully connected graph)
                if i != j {
                    let dx = coord1.0 as f32 - coord2.0 as f32;
                    let dy = coord1.1 as f32 - coord2.1 as f32;
                    cost[i][j] = (dx.powi(2) + dy.powi(2)).sqrt().round() as u32;
                }
            }
        }

        FullyConnectedGraph {
            nb_nodes,
            cost,
            coordinates,
        }
    }

    pub fn save_to_file(&self, filepath: &str) {
        let mut file = File::create(filepath).unwrap();
        
        // save number of nodes
        file.write_all(format!("{}\n", self.nb_nodes).as_bytes()).unwrap();
        
        // save cost matrix
        for row in &self.cost {
            for &val in row {
                file.write_all(format!("{} ", val).as_bytes()).unwrap();
            }
            file.write_all(b"\n").unwrap();
        }

        // save mst cost
        let mst_cost = self.compute_mst_cost();
        file.write_all(format!("{}\n", mst_cost).as_bytes()).unwrap();
    }

    fn get_mst(&self) -> Graph<(), u32, Undirected>  {
        let graph = UnGraph::<(), u32>::from_edges(self.coordinates.iter().enumerate().flat_map(|(i, _)| {
            self.coordinates.iter().enumerate().filter_map(move |(j, _)| {
                if i < j {
                    Some((NodeIndex::new(i), NodeIndex::new(j), self.cost[i][j]))
                } else {
                    None
                }
            })
        }));

        let mst_iter = min_spanning_tree(&graph);
        let mst: UnGraph<(), u32> = {
            let mst_edges: Vec<(NodeIndex<_>, NodeIndex<_>, u32)> = mst_iter.filter_map(|element| {
                if let petgraph::data::Element::Edge { source, target, weight } = element {
                    Some((NodeIndex::new(source), NodeIndex::new(target), weight))
                } else {
                    None
                }
            }).collect();
            UnGraph::from_edges(mst_edges)
        };
        return mst;
    }

    fn compute_mst_cost(&self) -> u32 {
        let mst = self.get_mst();
        let mut mst_cost = 0;
        for edge in mst.edge_references() {
            mst_cost += *edge.weight();
        }
        return mst_cost;
    }

    pub fn generate_mst_png_image(&self, filepath: &str) {
        let mst = self.get_mst();

        let scaling_factor: i32 = 10;
        let mut imgbuf = RgbImage::new((scaling_factor as usize * self.nb_nodes + 20) as u32, (10 * self.nb_nodes + 20) as u32);
        
        let node_color = image::Rgb([255, 0, 0]);
        let edge_color = image::Rgb([255, 255, 255]);

        for edge in mst.edge_references() {
            let (source, target) = (edge.source().index(), edge.target().index());
            let (source_x, source_y): (i32, i32) = {
                let (x, y) = self.coordinates[source];
                (x as i32, y as i32)
            };
            let (target_x, target_y) = {
                let (x, y) = self.coordinates[target];
                (x as i32, y as i32)
            };

            let source_circle_coordinates = (source_x*scaling_factor + 10, source_y*scaling_factor + 10);
            let target_circle_coordinates = (target_x*scaling_factor + 10, target_y*scaling_factor + 10);

            // Draw edges:
            let source_circle_coordinates_float = (source_circle_coordinates.0 as f32, source_circle_coordinates.1 as f32);
            let target_circle_coordinates_float = (target_circle_coordinates.0 as f32, target_circle_coordinates.1 as f32);
            drawing::draw_line_segment_mut(&mut imgbuf, source_circle_coordinates_float, target_circle_coordinates_float, edge_color);
            drawing::draw_line_segment_mut(&mut imgbuf, source_circle_coordinates_float, target_circle_coordinates_float, edge_color);

            // Draw circles:
            drawing::draw_filled_circle_mut(&mut imgbuf, source_circle_coordinates, 3, node_color);
            drawing::draw_filled_circle_mut(&mut imgbuf, target_circle_coordinates, 3, node_color);
        }

        imgbuf.save(filepath).unwrap();
    }

    pub fn generate_mst_svg_image(&self, filepath: &str) {
        let mst = self.get_mst();

        let scaling_factor: i32 = 20;
        let node_radius: i32 = 8;

        let mut document = Document::new()
            .set("width", self.nb_nodes * scaling_factor as usize + 40)
            .set("height", self.nb_nodes * scaling_factor as usize + 40);

        for edge in mst.edge_references() {
            let (source, target) = (edge.source().index(), edge.target().index());
            let (source_x, source_y) = self.coordinates[source];
            let (target_x, target_y) = self.coordinates[target];

            let source_pos = (source_x as i32 * scaling_factor + 20, source_y as i32 * scaling_factor + 20);
            let target_pos = (target_x as i32 * scaling_factor + 20, target_y as i32 * scaling_factor + 20);

            // Draw line for the edge
            let line = Line::new()
                .set("x1", source_pos.0)
                .set("y1", source_pos.1)
                .set("x2", target_pos.0)
                .set("y2", target_pos.1)
                .set("stroke", "black");

            document = document.add(line);

            // Draw edge weight next to the center of the edge
            let text = Text::new()
                .set("x", ((source_pos.0 + target_pos.0) / 2) + 5)
                .set("y", ((source_pos.1 + target_pos.1) / 2) + 5)
                .set("font-size", 10)
                .set("font-family", "Arial")
                .set("fill", "grey")
                .add(svg::node::Text::new(format!("{}", edge.weight())));
            document = document.add(text);
        }

        // Draw nodes and indices after to ensure they are on top
        for (index, &(x, y)) in self.coordinates.iter().enumerate() {
            let pos = (x as i32 * scaling_factor + 20, y as i32 * scaling_factor + 20);

            // Draw node
            let circle = Circle::new()
                .set("cx", pos.0)
                .set("cy", pos.1)
                .set("r", node_radius)
                .set("fill", "red");
            document = document.add(circle);

            // Display node index
            let text = Text::new()
                .set("x", pos.0 - (node_radius / 2)) // Slightly offset the text from the node
                .set("y", pos.1 + (node_radius / 2))
                .set("font-size", 10)
                .set("font-family", "Arial")
                .set("fill", "orange")
                .add(svg::node::Text::new(format!("{}", index)));
            document = document.add(text);
        }

        svg::save(filepath, &document).unwrap();
    }
}

fn main() {
    let dir_path = "generated";
    let timestamp = chrono::Utc::now().format("%Y-%m-%d-%H-%M-%S").to_string();

    let graph = FullyConnectedGraph::generate_random_graph(10);
    let output_filepath = format!("{}/fully_connected_graph_{}.txt", dir_path, timestamp);
    graph.save_to_file(&output_filepath);

    let output_image_filepath = format!("{}/fully_connected_graph_{}.png", dir_path, timestamp);
    graph.generate_mst_png_image(&output_image_filepath);

    let output_filepath = format!("{}/fully_connected_graph_{}.svg", dir_path, timestamp);
    graph.generate_mst_svg_image(&output_filepath);
}

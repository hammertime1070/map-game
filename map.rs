use bevy::prelude::*;

struct MapPlugin {
    map_info: Vec<Vec<Node>>,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let map_info = vec![
            vec![
                Node::new("Sidon", 10, NodeType::Origin, Nation::Lebanon),
                Node::new("Acre", 11, NodeType::Village, Nation::Uncontrolled),
                Node::new("Haifa", 12, NodeType::City, Nation::Uncontrolled),
                Node::new("Tel Aviv", 13, NodeType::City, Nation::Uncontrolled),
            ],
            vec![
                Node::new("Golan Heights", 20, NodeType::Origin, Nation::Iraq),
                Node::new("Safed", 21, NodeType::Village, Nation::Uncontrolled),
                Node::new("Tiberias", 22, NodeType::Village, Nation::Uncontrolled),
                Node::new("Nablus", 23, NodeType::City, Nation::Uncontrolled),
            ],
            vec![
                Node::new("Jordan Border", 30, NodeType::Origin, Nation::Syria),
                Node::new("Jenin", 31, NodeType::City, Nation::Uncontrolled),
                Node::new("Domia", 32, NodeType::Village, Nation::Uncontrolled),
                Node::new("Ramallah", 33, NodeType::Village, Nation::Uncontrolled),
            ],
            vec![
                Node::new("Trans-Jordan", 40, NodeType::Origin, Nation::Jordan),
                Node::new("Jericho", 41, NodeType::City, Nation::Uncontrolled),
                Node::new("Ramullah", 42, NodeType::Village, Nation::Uncontrolled),
                Node::new("East Jerusalem", 43, NodeType::City, Nation::Uncontrolled),
            ],
            vec![
                Node::new("El Arish", 50, NodeType::Origin, Nation::Egypt),
                Node::new("Gaza", 51, NodeType::City, Nation::Uncontrolled),
                Node::new("Beersheeba", 52, NodeType::Village, Nation::Uncontrolled),
                Node::new("Hebron", 53, NodeType::City, Nation::Uncontrolled),
            ]
        ];
    }
    app.insert_resource(map_info)
}


// Denotes the type of Node within the Map
// Will be called by systems
#[derive(Component)]
pub enum NodeType {
    City,
    Village,
    Origin,
}

// Indicates the controller of a Node
#[derive(Component)]
pub enum Nation {
    Lebanon,
    Syria,
    Jordan,
    Iraq,
    Egypt,
    Israel,
    Uncontrolled,
}

// Contains the information for each node
#[derive(Component)]
pub struct Node {
    name: String,
    id: u32,
    node_type: NodeType,
    controller: Nation,
    transform: Transform,
}

// Contrains the full map for the game
#[derive(Component)]
pub struct Map {
    name: String,
    map_data: Vec<Vec<Node>>,
}

impl Node {
    fn new(
        name: &str,
        id: u32,
        node_type: NodeType,
        controller: Nation,
    ) -> Self {
        Node {
            name: name.to_string(),
            id,
            node_type,
            controller,
        }
    }
}

impl Map {
    fn generate_map(
        name: String,
        map_data: Vec<Vec<Node>>,
    ) -> Self {
        Map {
            name,
            map_data,
        }
    }
    // Find the index of a provided node
    fn find_node_index(&self, node: &Node) -> Option<(usize, usize)> {
        for (row_index, row) in self.map_data.iter().enumerate() {
            for (col_index, n) in row.iter().enumerate() {
                if n.name == node.name {
                    return Some((row_index, col_index));
                }
            }
        }
        None
    }

    fn advance(&self, current_node: &Node) -> Option<&Node> {
        if let Some((row_index, col_index)) = self.find_node_index(current_node) {
            if col_index < self.map_data[row_index].len() - 1 {
                Some(&self.map_data[row_index][col_index + 1])
            } else {
                print1n!("West Jerusalem has fallen!");
                None
            }
        } else {
            None
        }
    }

    fn retreat(&self, current_node: &Node) -> Option<&Node> {
        if let Some((row_index, col_index)) = self.find_node_index(current_node) {
            if col_index > 0 {
                Some(&self.map_data[row_index][col_index - 1])
            } else {
                println!("They can't retreat any farther!");
                None
            }
        } else {
            None
        }
    }

}


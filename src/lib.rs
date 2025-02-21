use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub type GraphMLNoData = GraphML<()>;

trait AsStrId: Clone {
    fn id(&self) -> &str;
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(from = "Option<bool>")]
pub enum EdgeDirection {
    Directed,
    Undirected,
    Unspecified,
}

impl EdgeDirection {
    pub fn is_unspecified(&self) -> bool {
        matches!(self, EdgeDirection::Unspecified)
    }
}

impl From<Option<bool>> for EdgeDirection {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(true) => EdgeDirection::Directed,
            Some(false) => EdgeDirection::Undirected,
            None => EdgeDirection::Unspecified,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum KeyFor {
    Edge,
    Node,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AttrType {
    Boolean,
    Int,
    Long,
    Float,
    Double,
    String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "key")]
pub struct Key {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@for")]
    r#for: KeyFor,
    #[serde(rename = "@attr.name")]
    name: String,
    #[serde(rename = "@attr.type")]
    r#type: AttrType,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
struct DataString {
    #[serde(rename = "@key")]
    key: String,
    #[serde(rename = "$value")]
    content: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "graphml")]
pub struct GraphML<NodeData>
where
    NodeData: Clone,
{
    graph: Graph<NodeData>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename = "graph")]
pub struct Graph<NodeData = ()>
where
    NodeData: Clone,
{
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@edgedefault")]
    edge_default: String,
    #[serde(rename = "node")]
    nodes: NodesMap<NodeData>,
    #[serde(rename = "edge")]
    edges: EdgesMap,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename = "node")]
pub struct Node<NodeData = ()> {
    #[serde(rename = "@id")]
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<NodeData>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename = "edge")]
pub struct Edge {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@source")]
    source: String,
    #[serde(rename = "@target")]
    target: String,
    #[serde(rename = "directed")]
    #[serde(skip_serializing_if = "EdgeDirection::is_unspecified")]
    direction: EdgeDirection,
}

impl<NodeData: for<'a> Deserialize<'a> + Clone> GraphML<NodeData> {
    pub fn load<P: AsRef<Path>>(fname: P) -> Result<Self, String> {
        let fname = fname.as_ref();
        let content = std::fs::read_to_string(fname).map_err(|e| e.to_string())?;
        Self::from_str(&content)
    }
    pub fn from_str(content: &str) -> Result<Self, String> {
        quick_xml::de::from_str(content)
            .map_err(|e| e.to_string())
            .map(|g: Self| g.ensure_consistency())?
    }

    fn ensure_consistency(self) -> Result<Self, String> {
        // TODO check that the graph is completly consistent
        // Return a String explaining the error if any
        Ok(self)
    }
}

impl AsStrId for Edge {
    fn id(&self) -> &str {
        &self.id
    }
}

impl<NodeData: Clone> AsStrId for Node<NodeData> {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Deserialize, Serialize)]
pub struct StuffList<Stuff: AsStrId>(Vec<Stuff>);

#[derive(Deserialize, Serialize, Clone)]
#[serde(from = "StuffList<Stuff>")]
#[serde(into = "StuffList<Stuff>")]
pub struct StuffMap<Stuff: AsStrId>(IndexMap<String, Stuff>);

impl<Stuff: AsStrId> From<StuffList<Stuff>> for StuffMap<Stuff> {
    fn from(value: StuffList<Stuff>) -> StuffMap<Stuff> {
        StuffMap::<Stuff>(
            value
                .0
                .into_iter()
                .map(|n| (n.id().to_owned(), n))
                .collect(),
        )
    }
}

impl<Stuff: AsStrId> From<StuffMap<Stuff>> for StuffList<Stuff> {
    fn from(value: StuffMap<Stuff>) -> Self {
        StuffList(value.0.into_values().collect())
    }
}

pub type NodesMap<NodeData> = StuffMap<Node<NodeData>>;
pub type EdgesMap = StuffMap<Edge>;

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use crate::{DataString, Edge, Graph, GraphML, Key, Node};

    #[test]
    fn test_graphml_deserialisation() {
        let graphs = [r#"
            <?xml version="1.0" encoding="UTF-8"?>

<graphml xmlns="http://graphml.graphdrawing.org/xmlns"  
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd">
  <graph id="G" edgedefault="undirected">
    <node id="n0"/>
    <node id="n1"/>
    <edge id="e1" source="n0" target="n1"/>
  </graph>
</graphml>
            "#];

        for repr in graphs.into_iter() {
            let g: GraphML<()> = quick_xml::de::from_str(repr).unwrap();
        }
    }

    #[test]
    fn test_graph_deserialisation() {
        let graphs = [r#"
  <graph id="G" edgedefault="undirected">
    <node id="n0"/>
    <node id="n1"/>
    <edge id="e1" source="n0" target="n1"/>
  </graph>
            "#];

        for repr in graphs.into_iter() {
            dbg!(repr);
            let g: Graph = quick_xml::de::from_str(repr).unwrap();
            let repr_d = quick_xml::se::to_string(&g).unwrap();
        }
    }

    #[test]
    fn test_graph_with_data_deserialisation1() {
        #[derive(Serialize, Deserialize, Clone)]
        struct NodeData {
            x: f64,
            y: f64,
        }
        let graphs = [r#"
  <graph id="G" edgedefault="undirected">
    <node id="n0"><data><x>1</x><y>1</y></data></node>
    <node id="n1"><data><x>1</x><y>1</y></data></node>
    <edge id="e1" source="n0" target="n1"/>
  </graph>
            "#];

        for repr in graphs.into_iter() {
            dbg!(repr);
            let g: Graph<NodeData> = quick_xml::de::from_str(repr).unwrap();
            let repr_d = quick_xml::se::to_string(&g).unwrap();
            dbg!(repr_d);
        }
    }

    #[test]
    fn test_graph_with_data_deserialisation2() {
        let graphs = [r#"
  <graph id="G" edgedefault="undirected">
    <node id="n0"><data key="a">kfjdkfjd</data></node>
    <node id="n1"><data key="a">jfkdsjfdskj</data></node>
    <edge id="e1" source="n0" target="n1"/>
  </graph>
            "#];

        for repr in graphs.into_iter() {
            dbg!(repr);
            let g: Graph<DataString> = quick_xml::de::from_str(repr).unwrap();
            let repr_d = quick_xml::se::to_string(&g).unwrap();
            dbg!(repr_d);
        }
    }

    #[test]
    fn test_nodes_deserialization() {
        let nodes = ["<node id=\"n3\"/>"];

        for repr in nodes.into_iter() {
            let n: Node = quick_xml::de::from_str(repr).unwrap();
            let repr_d = quick_xml::se::to_string(&n).unwrap();
            assert_eq!(repr, &repr_d);
        }
    }

    #[test]
    fn test_nodes_with_data_deserialization() {
        #[derive(Serialize, Deserialize)]
        struct NodeData {
            #[serde(rename = "@key")]
            key: String,
            x: f64,
            y: f64,
        }
        let nodes = [r#"<node id="n0"><data key="kind"><x>1</x><y>1</y></data></node>"#];

        for repr in nodes.into_iter() {
            let n: Node<NodeData> = quick_xml::de::from_str(repr).unwrap();
            let repr_d = quick_xml::se::to_string(&n).unwrap();
            assert_eq!(repr, &repr_d);
        }
    }

    #[test]
    fn test_edges_deserialization() {
        let edges = ["<edge id=\"e0\" source=\"n1\" target=\"n12\"/>"];

        for repr in edges.into_iter() {
            let e: Edge = quick_xml::de::from_str(repr).unwrap();
            let repr_d = quick_xml::se::to_string(&e).unwrap();
            assert_eq!(repr, &repr_d);
        }
    }

    #[test]
    fn test_key_deserialization() {
        let keys = [
            r#"<key id="d1" for="edge" attr.name="weight" attr.type="double"/>"#,
            r#"<key id="d0" for="node" attr.name="color" attr.type="string">
    <default>yellow</default>
  </key>"#,
        ];

        for repr in keys.into_iter() {
            let e: Key = quick_xml::de::from_str(repr).unwrap();
            let repr_d = quick_xml::se::to_string(&e).unwrap();
            //assert_eq!(repr,& repr_d);
        }
    }

    #[test]
    fn test_data_deserialization() {
        let keys = [r#"<data key="d1">1.0</data>"#];

        for repr in keys.into_iter() {
            let e: DataString = quick_xml::de::from_str(repr).unwrap();
            let repr_d = quick_xml::se::to_string(&e).unwrap();
            //assert_eq!(repr,& repr_d);
        }
    }
}

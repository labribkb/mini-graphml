use std::path::Path;

use mini_graphml::GraphMLNoData;

#[test]
fn test_grapho() {
    let fname = Path::new("data").join("grafo113.28.graphml");
    dbg!(std::env::current_dir(), fname.exists());
    let g1 = GraphMLNoData::load(fname).expect("Unable to load file");

    let content1 = quick_xml::se::to_string(&g1).expect("Unable to serialize");
    let g2 = GraphMLNoData::from_str(&content1).expect("Unable to read serialized version");

    let content2 = quick_xml::se::to_string(&g2).expect("Unable to serialize");
    assert_eq!(content1, content2);
}

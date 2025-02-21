use mini_graphml::GraphMLNoData;
use std::path::Path;


#[test]
fn rome() {
    let rome_path = Path::new("data/rome");
    let rome_url = "http://www.graphdrawing.org/data/rome-graphml.tgz";

    handle_dataset(rome_url, rome_path);
}

#[test]
fn at_t() {
    handle_dataset(
        "http://www.graphdrawing.org/data/north-graphml.tgz",
        Path::new("data/north"),
    );
}

#[test]
fn random() {
    handle_dataset(
        "http://www.graphdrawing.org/data/random-dag-graphml.tgz",
        Path::new("data/random-dag"),
    );
}

fn handle_dataset(url: &str, path: &Path) {
    if !path.exists() {
        let content = ureq::get(url).call().unwrap().into_body().into_reader();

        let content = flate2::read::GzDecoder::new(content);

        let mut arch = tar::Archive::new(content);
        arch.unpack(path.parent().unwrap())
            .expect("Unable to unarchive rome data");
    }

    for fname in glob::glob(path.join("rome").join("*.graphml").to_str().unwrap()).unwrap() {
        let fname = fname.unwrap();
        let fname = &fname;
        let str_fname = fname.to_str().unwrap();
        eprintln!("Handle {str_fname}");

        let g1 =
            GraphMLNoData::load(fname).unwrap_or_else(|_| panic!("Unable to load {str_fname}"));

        let content1 = quick_xml::se::to_string(&g1).expect("Unable to serialize");
        let g2 = GraphMLNoData::from_str(&content1).expect("Unable to read serialized version");

        let content2 = quick_xml::se::to_string(&g2).expect("Unable to serialize");
        assert_eq!(content1, content2);

        #[cfg(feature = "petgraph")]
        {
            let p = g1.into_petgraph();
        }
    }
}

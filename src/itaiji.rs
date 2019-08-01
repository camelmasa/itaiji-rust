use std::collections::HashMap;
use yaml_rust::YamlLoader;

pub struct Converter {
    itaiji_pairs: HashMap<String, Vec<String>>,
}

impl Converter {
    pub fn new() -> Self {
        let mut itaiji_pairs = HashMap::new();

        const ITAIJI_STRING: &str = include_str!("../itaiji_list.yml");
        let docs = YamlLoader::load_from_str(ITAIJI_STRING).unwrap();

        for (s, i) in docs[0].as_hash().unwrap().iter() {
            let seijitai = s.as_str().unwrap().to_string();
            let mut itaijis: Vec<String> = vec![];

            for itaiji in i.clone().into_iter() {
                itaijis.push(itaiji.as_str().unwrap().to_string());
            }

            itaiji_pairs.insert(seijitai, itaijis);
        }

        Converter {
            itaiji_pairs: itaiji_pairs,
        }
    }

    pub fn seijitai(&self, text: &str) -> String {
        let mut itaiji_text = text.to_string();

        for (seijitai, itaijis) in self.itaiji_pairs.iter() {
            for itaiji in itaijis.iter() {
                itaiji_text = itaiji_text.replace(itaiji, seijitai);
            }
        }

        itaiji_text
    }

    pub fn itaiji(&self, text: &str) -> String {
        let mut seijitai_text = text.to_string();

        for (seijitai, itaijis) in self.itaiji_pairs.iter() {
            seijitai_text = seijitai_text.replace(seijitai, itaijis.first().unwrap());
        }

        seijitai_text
    }
}

#[cfg(test)]
mod tests {
    use super::Converter;

    #[test]
    fn seijitai() {
        let converter = Converter::new();
        let text = "齊藤";

        assert_eq!(converter.seijitai(text), "斉藤");
    }

    #[test]
    fn itaiji() {
        let converter = Converter::new();
        let text = "斉藤";

        assert_eq!(converter.itaiji(text), "齊藤");
    }
}

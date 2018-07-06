extern crate linked_hash_map;
use self::linked_hash_map::*;
extern crate colored;
use self::colored::*;

use file_finder::CTFile;

#[derive(Debug)]
pub struct CTMan{
    title: String,
    content: String,
}

impl CTMan{
    pub fn all(ct_file: &CTFile) -> Option<LinkedHashMap<String, CTMan>>{
        if let Ok(readme_content) = ct_file.get_readme_content() {
            return Some(CTMan::read_from_readme(readme_content))
        }
        None
    }

    fn read_from_readme(readme_content: String) -> LinkedHashMap<String, CTMan>{
        let man_chunks: Vec<&str> = readme_content.split("\n# ").collect();
        let mut out: LinkedHashMap<String,CTMan> = LinkedHashMap::new();
        man_chunks.into_iter()
            .map( |chunks| chunks.splitn(2, "\n").collect::<Vec<&str>>())
            .filter(|splits| splits.len() > 1)
            .map(|splits| (splits[0].to_string().to_lowercase(), CTMan{ title: splits[0].to_string(), content: splits[1].to_string()}))
            .for_each(|tuple| { let _ = out.insert(tuple.0, tuple.1); } );
        out
    }

    pub fn help(ct_file: &CTFile){
        if let Some(readme_content )= CTMan::all(ct_file){
            println!("{}", "Found the following manual topics in README.md".blue());
            readme_content.iter().map(|v| v.1)
                .for_each(|v| println!("{}", v.title));
        }
    }

    pub fn print(&self){
        println!("{}", self.title.blue());
        println!("{}", self.content);
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_from_readme(){
        let sample_readme = r"
# Project information

lMassa diam ac sodales a himenaeos pretium sit sociosqu euismod nisi ad hac duis neque hendrerit, nam donec ornare dis consequat venenatis dui ullamcorper vivamus sapien mus ante conubia mi. Consequat ac rutrum id aliquam vel elit turpis, arcu enim taciti scelerisque cursus eu, sodales primis mattis euismod dolor quisque. Nisi risus lacinia platea nullam tincidunt sem faucibus ligula pharetra et porta venenatis elementum class non, congue velit maecenas turpis auctor donec cursus per pellentesque aenean lobortis rhoncus diam malesuada. Tristique imperdiet elit habitasse faucibus commodo nam cursus dictumst, eu fermentum blandit rhoncus purus praesent lorem nec libero, luctus accumsan per diam felis aptent nascetur. Laoreet volutpat aliquam lectus id nisl natoque quisque ultricies tortor erat suscipit, vehicula neque cras hac at gravida fusce mi imperdiet commodo euismod, platea magnis dis massa lacus elementum nisi rhoncus mollis luctus. Urna vulputate fringilla mus diam nibh ultricies cras, platea velit taciti ad lacus quisque, dolor lobortis sollicitudin rhoncus ante vel.

Porta dictum mi fusce dis pellentesque convallis consectetur venenatis habitant parturient, augue dui quis in vulputate luctus primis ipsum imperdiet. Iaculis rhoncus suscipit vehicula ad tristique, rutrum porta ac urna habitant blandit, quam magna consectetur ligula. Est sapien placerat justo metus ligula ipsum aptent elit ultricies dictumst nisi, in senectus praesent fusce urna cubilia aenean class sem varius, consequat aliquam habitasse dolor nibh suscipit eu interdum quis mus. Sociosqu eget arcu litora curabitur scelerisque rhoncus urna habitant, mi turpis taciti tempor condimentum praesent quam ridiculus primis, fermentum ornare dis adipiscing dapibus elementum varius.

Lead : @code-troopers.com

# Links
    - trello    : https://trello.com/b/S7lhNFvV/
    - dev       : http://0.0.0.0:9000
    - prp       : https://pp.my.site

# Stack
    Front :
        - AngularJS
        -
    Back :
        - Restx
        - mongodb


# Dev build and run

## build

To build bend and pray

## run

To run, do the classic dancing thing

";
        let mans = CTMan::read_from_readme(sample_readme.to_string());
        println!("{:?}", mans);
        assert_eq!(mans.keys().len(), 4);
        assert!(mans.contains_key("links"));
    }
}
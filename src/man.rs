#![feature(nll)]
extern crate linked_hash_map;
use self::linked_hash_map::*;
extern crate colored;
use self::colored::*;
extern crate pulldown_cmark;
use self::pulldown_cmark::*;
use std::*;
use std::clone::Clone;

use file_finder::CTFile;
use std::thread::current;

#[derive(Debug, Clone)]
pub struct CTMan{
    title: String,
    content: String,
    //subpart: Vec<CTMan>,
    parent: Option<&'static CTMan>,
    level: i32
}

impl CTMan{
    pub fn all(ct_file: &CTFile) -> Option<LinkedHashMap<String, CTMan>>{
     /*   if let Ok(readme_content) = ct_file.get_readme_content() {
            return Some(CTMan::read_from_readme(readme_content))
        }*/
        None
    }


    pub fn find(ct_file: &CTFile, key: &str) -> Option<CTMan>{
        let readme_content = ct_file.get_readme_content().unwrap();
        CTMan::find_from_readme(readme_content, key)
    }

    fn find_from_readme(readme_content: String, key: &str) -> Option<CTMan>{
        let parser = Parser::new(&readme_content);
        let mut level = 0;
        let mut should_search = false;
        let mut should_read = false;
        let mut ct_man: Option<CTMan> = None;
        for event in parser{
            match event{
                Event::Start(tag) => {
                    match tag {
                        Tag::Header(lvl) => {
                            if ct_man.is_some(){
                                if level > lvl {
                                    return ct_man;
                                }
                            }
                            level = lvl;
                            should_search = true;
                        },
                        _ => ()
                    }
                },
                Event::End(tag) => {
                    should_search = false;
                }
                Event::Text(text) => {
                    if should_search && text.to_lowercase() == key.to_lowercase(){
                        should_read = true;
                        ct_man = Some(CTMan { title: text.to_string(), content: "".to_string(), level, parent: None});
                    }else if let Some(ref mut current_man)= ct_man{
                        current_man.content += &text.to_string();
                        current_man.content += "\n";
                    }

                }
                _ => ()
            }
        }
        ct_man
    }

  /*  fn read_from_readme(readme_content: String) -> LinkedHashMap<String, CTMan>{
        let mut out: LinkedHashMap<String,CTMan> = LinkedHashMap::new();
        let parser = Parser::new(&readme_content);
        let mut level = 0;
        let mut man_parts: Vec<CTMan> = Vec::new();
        let mut is_header = false;
        let mut current_man: Option<&'static CTMan> = None;

        for event in parser {
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::Header(lvl) => {
                            if lvl == 1 {
                                let ct_man = CTMan { title: "".to_string(), content: "".to_string(), parent: None, level: lvl };
                                man_parts.push(ct_man);
                                current_man = man_parts.last();

//                                    current_man = man_parts.last();
                            } else if level > lvl {
                                //current_man = man_parts.last();
                            }
                            level = lvl;
                            is_header = true;
                        },
                        _ => ()
                    }
                },
                Event::End(tag) => {
                    match tag {
                        Tag::Header(lvl) => {
                            level = lvl;

                            is_header = false;
                        },
                        _ => ()
                    }
                },
                Event::Text(text) => {

                      if let Some(ref mut current) = man_parts.last_mut() {
                    /*current_man.map( |current| {*/
                    if is_header {
                        if level > current.level {
                            let man = CTMan { title: text.to_string(), content: "".to_string(), parent: current_man, level };
                            current_man = Some(&man);

                            //current.subpart.push(man.to_owned());
                        } else if level == current.level {
                            current.title = text.to_string();
                        }
                    } else {
                        current.content += &text.to_string();
                    }
                }
                /*);*/
                  //  ()
                    // }
                },
                _ => ()
            }
        }

     /*   let allParts:Vec<&CTMan> = man_parts.iter().flat_map(|m| {
            let mut vec = Vec::new();
            vec.push(m);
            vec.extend(m.subpart.iter());
            return vec
        }).collect();
*/
        println!("| {:?}", man_parts);
      /*  allParts.into_iter().for_each( |a| {
            let clone = a.clone();
            let _ = out.insert(clone.title.to_lowercase().to_string(), clone);
        });*/
        out
    }
*/
  pub fn help(ct_file: &CTFile){
   /*     if let Some(readme_content )= CTMan::all(ct_file){
            println!("{}", "Found the following manual topics in README.md".blue());
            readme_content.iter().map(|v| v.1)
                .for_each(|v| println!("{}", v.title));
        }*/
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

lMassa

Porta

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

Dev workflow detailed here.

Warning this can be tricky.

## build

To build bend and pray.

Buy things

## run

To run, do the classic dancing thing

### dance

Dance dudes

";

        let man = CTMan::find_from_readme(sample_readme.to_string(), "dev build and run");
        assert!(man.is_some());
        println!("{:#?}", man.unwrap());
       /* let mans = CTMan::read_from_readme(sample_readme.to_string());
        println!("{:?}", mans.get("dev build and run"));
       // println!("------ \n {:?}", mans);
        assert_eq!(mans.keys().len(), 6);
        assert!(mans.contains_key("links"));
        assert!(mans.contains_key("build"));*/
    }
}

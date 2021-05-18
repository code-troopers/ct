extern crate linked_hash_map;

use self::linked_hash_map::*;

extern crate colored;

use self::colored::*;

extern crate pulldown_cmark;

use self::pulldown_cmark::*;
use std::*;
use std::clone::Clone;

use crate::file_finder::CTFile;


#[derive(PartialEq, Debug, Clone)]
pub struct CTMan {
    title: String,
    content: String,
    subpart: Vec<CTMan>,
    parent: Option<&'static CTMan>,
    level: u32,
}

impl CTMan {
    fn add_to_inner_sub_part(&mut self, next_element: CTMan) -> bool {
        if let Some(element) = self.subpart.last_mut() {
            return element.add_sub_part(next_element);
        }

        false
    }

    fn add_sub_part(&mut self, next_element: CTMan) -> bool {
        if next_element.level == self.level + 1 {
            self.subpart.push(next_element.to_owned());
            return true;
        } else {
            self.add_to_inner_sub_part(next_element);
        }
        false
    }

    pub fn all(_ct_file: &CTFile) -> Option<LinkedHashMap<String, CTMan>> {
        /*   if let Ok(readme_content) = ct_file.get_readme_content() {
               return Some(CTMan::read_from_readme(readme_content))
           }*/
        None
    }


    pub fn find(ct_file: &CTFile, key: &str) -> Option<CTMan> {
        let readme_content = ct_file.get_readme_content().unwrap();
        CTMan::find_from_readme(readme_content, key)
    }

    fn find_from_readme(readme_content: String, key: &str) -> Option<CTMan> {
        let parser = Parser::new(&readme_content);
        let mut level = 0;
        let mut should_search = false;
        let mut should_read_origin_level = 0;
        let mut ct_man: Vec<CTMan> = Vec::new();
        for event in parser {
            // println!("{:#?}", event);
            match event {
                Event::Start(Tag::Heading(lvl)) => {
                    if !ct_man.is_empty() && level > lvl {
                        return ct_man.first().map(|man| man.to_owned());
                    }
                    level = lvl;
                    should_search = true;
                    if level <= should_read_origin_level {
                        should_read_origin_level = 0
                    }
                }
                Event::End(Tag::Heading(_lvl)) => should_search = false,
                Event::Text(text) => {
                    if should_search && text.to_lowercase() == key.to_lowercase() {
                        should_read_origin_level = level;
                        ct_man.push(CTMan { title: text.to_string(), content: "".to_string(), level, parent: None, subpart: Vec::new() });
                    } else if should_read_origin_level > 0 {
                        if should_search {
                            ct_man.push(CTMan { title: text.to_string(), content: "".to_string(), level, parent: None, subpart: Vec::new() });
                        } else {
                            ct_man.last_mut().map(|man| {
                                man.content += &text.to_string();
                                man.content += "\n";
                            });
                        }
                    }
                }
                _ => ()
            }
        }
        //let mut out = vec![];
        {
            let _to_remove: Vec<&CTMan> = vec![];
            let mut iter = ct_man.iter_mut();
            while let Some(element) = iter.next() {
                while let Some(next_element) = iter.next() {
                    let next_man = next_element.to_owned();
                    element.add_sub_part(next_man);
                }
            }
        }
        println!(">>> {:#?}", &ct_man.clone());

        ct_man.first().map(|m| m.to_owned())
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
    pub fn help(_ct_file: &CTFile) {
        /*     if let Some(readme_content )= CTMan::all(ct_file){
                 println!("{}", "Found the following manual topics in README.md".blue());
                 readme_content.iter().map(|v| v.1)
                     .for_each(|v| println!("{}", v.title));
             }*/
    }


    pub fn print(&self) {
        println!("{}", self.title.blue());
        println!("{}", self.content);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const sample_readme: &str = r"
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

#### rock

Rock paper cissor lizard spock




#### sing

Don't hesitate to make your voice matter

##### chatter

With chat there is noise

";


    #[test]
    fn test_read_from_readme() {
        let man = CTMan::find_from_readme(sample_readme.to_string(), "dev build and run");
        assert!(man.is_some());
        println!("++++ {:#?}", man.unwrap());
        /* let mans = CTMan::read_from_readme(sample_readme.to_string());
         println!("{:?}", mans.get("dev build and run"));
        // println!("------ \n {:?}", mans);
         assert_eq!(mans.keys().len(), 6);
         assert!(mans.contains_key("links"));
         assert!(mans.contains_key("build"));*/
    }

    #[test]
    fn test_read_from_readme_build() {
        let man = CTMan::find_from_readme(sample_readme.to_string(), "links");
        assert!(man.is_some());
        println!("++++ {:#?}", man.unwrap());
        /* let mans = CTMan::read_from_readme(sample_readme.to_string());
         println!("{:?}", mans.get("dev build and run"));
        // println!("------ \n {:?}", mans);
         assert_eq!(mans.keys().len(), 6);
         assert!(mans.contains_key("links"));
         assert!(mans.contains_key("build"));*/
    }
}

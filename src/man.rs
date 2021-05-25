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
            self.subpart.push(next_element);
            return true;
        } else {
            self.add_to_inner_sub_part(next_element);
        }
        false
    }

    fn find_sub_part(&mut self, with_level: u32) -> Option<&mut CTMan> {
        if self.subpart.is_empty() {
            if self.level == with_level - 1 {
                return Some(self);
            }
            return None;
        }
        if let Some(man) = self.subpart.last() {
            if man.level == with_level {
                return Some(self);
            }
        }
        //need to split as we need to borrow mutably here only
        if let Some(man) = self.subpart.last_mut() {
            if man.level < with_level {
                return man.find_sub_part(with_level);
            }
        }
        None
    }

    pub fn all(ct_file: &CTFile) -> Option<LinkedHashMap<String, CTMan>> {
        if let Ok(readme_content) = ct_file.get_readme_content() {
            let mut out = LinkedHashMap::new();
            CTMan::find_from_readme(readme_content, None).into_iter()
                .for_each(|m| {
                    let title = m.title.clone().to_lowercase();
                    out.insert(title,  m);
                });
            Some(out)
        } else {
            None
        }
    }


    pub fn find(ct_file: &CTFile, key: &str) -> Option<CTMan> {
        let readme_content = ct_file.get_readme_content().unwrap();
        CTMan::find_from_readme(readme_content, Some(key)).first().map(|m| m.to_owned())
    }

    fn find_from_readme(readme_content: String, key: Option<&str>) -> Vec<CTMan> {
        let parser = Parser::new(&readme_content);
        let mut level = 0;
        let mut should_search = false;
        let mut should_read_origin_level = 0;
        let mut ct_man: Vec<CTMan> = Vec::new();
        let mut man: Option<CTMan> = None;
        for event in parser {
            match event {
                Event::Start(Tag::Heading(lvl)) => {
                    if !ct_man.is_empty() && level > lvl && key != None {
                        //println!("£££ {:#?}", ct_man);
                        return ct_man;
                    }
                    if let Some(ref man) = man {
                        CTMan::build_or_append_man_entry(level, &mut ct_man, man.clone());
                    }
                    level = lvl;
                    should_search = true;
                    if level <= should_read_origin_level {
                        should_read_origin_level = 0
                    }
                }
                Event::End(Tag::Heading(_lvl)) => {
                    should_search = false
                }
                Event::Text(txt) | Event::Code(txt) => {
                    let text = &txt.into_string();
                    if !should_search {
                        if let Some(ref mut man) = man {
                            man.content += &text.to_string();
                            man.content += "\n";
                        }
                    } else {
                        let current_man = CTMan { title: text.to_string(), content: "".to_string(), level, parent: None, subpart: Vec::new() };

                        match key {
                            Some(key) =>
                                if should_search && text.to_lowercase() == key.to_lowercase() {
                                    should_read_origin_level = level;
                                    man = Some(current_man);
                                } else if should_read_origin_level > 0 {
                                    man = Some(current_man);
                                },
                            None => {
                                man = Some(current_man);
                                if should_read_origin_level <= 0 {
                                    should_read_origin_level = level;
                                    //man = Some(current_man);
                                }
                            }
                        }
                    }
                }
                _ => ()
            }
        }
        if let Some(ref man) = man {
            CTMan::build_or_append_man_entry(level, &mut ct_man, man.clone());
        }

        //println!(">>> {:#?}", &ct_man);

        ct_man
    }

    fn build_or_append_man_entry(level: u32, ct_man: &mut Vec<CTMan>, new_man: CTMan) {
        if let Some(first) = ct_man
            .iter_mut()
            .filter_map(|man| man.find_sub_part(level))
            .collect::<Vec<_>>()
            .last_mut() {
            first.add_sub_part(new_man);
        } else {
            ct_man.push(new_man);
        }
    }

    pub fn help(ct_file: &CTFile) {
        if let Some(readme_content) = CTMan::all(ct_file) {
            println!("{}", "Found the following manual topics in README.md".blue());
            readme_content.iter().map(|v| v.1)
                .for_each(|v| println!("{}", v.title));
        }
    }


    pub fn print(&self) {
        println!("{}", self.title.blue());
        println!("{}", self.content);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_README: &str = r"
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
        let man = CTMan::find_from_readme(SAMPLE_README.to_string(), Option::from("dev build and run")).first().map(|e| e.to_owned());
        assert!(man.is_some());
        println!("++++ {:#?}", man.unwrap());
        /* let mans = CTMan::read_from_readme(SAMPLE_README.to_string());
         println!("{:?}", mans.get("dev build and run"));
        // println!("------ \n {:?}", mans);
         assert_eq!(mans.keys().len(), 6);
         assert!(mans.contains_key("links"));
         assert!(mans.contains_key("build"));*/
    }

    #[test]
    fn test_read_from_readme_find_empty() {
        let man = CTMan::find_from_readme(SAMPLE_README.to_string(), None).first().map(|e| e.to_owned());
        assert!(man.is_some());
        println!("++++ {:#?}", man.unwrap());
        /* let mans = CTMan::read_from_readme(SAMPLE_README.to_string());
         println!("{:?}", mans.get("dev build and run"));
        // println!("------ \n {:?}", mans);
         assert_eq!(mans.keys().len(), 6);
         assert!(mans.contains_key("links"));
         assert!(mans.contains_key("build"));*/
    }

    #[test]
    fn test_read_from_readme_links() {
        let man = CTMan::find_from_readme(SAMPLE_README.to_string(), Option::from("links")).first().map(|e| e.to_owned());
        assert!(man.is_some());
        let man = man.unwrap();
        //println!("++++ {:#?}", &man);
        assert_eq!(&man.title, "Links");
        assert!(&man.subpart.is_empty());
    }

    #[test]
    fn test_read_from_readme_run() {
        let man = CTMan::find_from_readme(SAMPLE_README.to_string(), Option::from("run")).first().map(|e| e.to_owned());
        assert!(man.is_some());
        let man = man.unwrap();
        //println!("++++ {:#?}", &man);
        assert_eq!(&man.title, "run");
        assert!(!&man.subpart.is_empty());
        assert_eq!(&man.subpart.len(), &1);
        assert_eq!(&man.subpart[0].content, "Dance dudes\n");
        assert_eq!(&man.subpart[0].subpart.len(), &2);
    }

    const SHORT_README: &str = r"
# first title

Content

## first subsection

Content subsection

# second title

Content second title

## second subsection

Content second subsection
    ";

    #[test]
    fn test_read_from_short_readme_find_empty() {
        let man = CTMan::find_from_readme(SHORT_README.to_string(), None).first().map(|e| e.to_owned());
        assert!(man.is_some());
        println!("++++ {:#?}", man.unwrap());
        /* let mans = CTMan::read_from_readme(SAMPLE_README.to_string());
         println!("{:?}", mans.get("dev build and run"));
        // println!("------ \n {:?}", mans);
         assert_eq!(mans.keys().len(), 6);
         assert!(mans.contains_key("links"));
         assert!(mans.contains_key("build"));*/
    }
}

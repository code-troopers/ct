extern crate itertools;
extern crate regex;
extern crate serde;
extern crate serde_json;

use self::itertools::Itertools;
use self::regex::Regex;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;
use crate::errors::CTErrors;


#[derive(Serialize, Deserialize,Debug)]
pub struct CTPorts{
    pid: isize,
    name: String,
    listen: Vec<String>,
    cwd: Option<String>,
}

impl CTPorts{

    //result is not cached but it should not be invoked more than once per ct call, so this is not a big deal
    pub fn available() -> bool {
        Command::new("lsof")
                      .arg("-v")
                      .stdout(Stdio::null())
                      .stderr(Stdio::null())
                      .spawn().is_ok()
    }

    pub fn all() -> Result<Vec<CTPorts>, CTErrors>{
        let lsof_output = CTPorts::run_lsof();
        let all_ports = CTPorts::all_from_lsof_output(String::from_utf8_lossy(&lsof_output.stdout).to_string());
        Ok(all_ports)
    }

    fn run_lsof() -> Output{
        Command::new("lsof")
            .args(vec!["-iTCP", "-sTCP:LISTEN", "-P", "-Fcn"])
            .output().unwrap()
    }

    fn all_from_lsof_output(lsof_output: String) -> Vec<CTPorts>{
        //println!(">>> {:?}", lsof_output);
        let file_regex = Regex::new("^f[0-9]*$").unwrap();
        let chunks = lsof_output.split("\np");
        chunks.into_iter()
            .map(|s| {
                s.split('\n').filter(|s| !file_regex.is_match(s)).collect::<Vec<&str>>()
            })
            .filter(|v| v.len() > 2 )
            .map(|vec| {
                let pid = vec[0].replace("p", "").parse::<isize>().unwrap_or(0);
                CTPorts { pid,
                          name: vec[1][1..].to_string(),
                          listen: vec.split_at(2).1.to_vec().iter()
                              .filter( |s| !s.is_empty())
                              .map(|s| s[1..].to_string())
                              .unique()
                              .collect::<Vec<String>>(),
                          cwd: CTPorts::get_cwd_for_pid(pid),
                }
            })
            .filter(|c| c.pid != 0)
            .collect::<Vec<CTPorts>>()
        //println!("{:?}", cleaned_chunks);
    }

    fn get_cwd_for_pid(pid: isize) -> Option<String>{
        let process_info = Command::new("lsof")
            .args(vec!["-p",&pid.to_string(),"-Ffn"])
            .output().ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned());
        CTPorts::read_cwd_from_lsof_output(process_info)
    }

    fn read_cwd_from_lsof_output(process_info: Option<String>) -> Option<String> {
        if let Some(pi) = process_info {
            let process_info_chunks = pi.split('\n').collect::<Vec<&str>>();
            let fcwd_index = process_info_chunks.iter().position(|s| s.starts_with("fcwd"));
            if let Some(index) = fcwd_index {
                if process_info_chunks.len() > index {
                    let cwd_with_leading = process_info_chunks[index + 1];
                    if !cwd_with_leading.is_empty() {
                        return Some(cwd_with_leading[1..].to_string())
                    }
                }
            }
        }
        None
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn should_parse_lsof_output(){
        let sample_output = r"p35985
cShimo-Setapp
f52
n*:61600
p42166
cBetterTouchTool
f16
n*:57411
f17
n*:57411
p47915
cidea
f7
nlocalhost:10001
f226
nlocalhost:6942
f718
nlocalhost:17434
f737
nlocalhost:63342
f761
nlocalhost:9123";
        let parsed = CTPorts::all_from_lsof_output(sample_output.to_string());
        assert_eq!(3, parsed.len());
        let first = &parsed[0];
        assert_eq!(35985, first.pid);
        assert_eq!("Shimo-Setapp", first.name);
        assert_eq!(1, first.listen.len());
        assert_eq!("*:61600", &first.listen[0]);

        let last = &parsed[2];
        assert_eq!(47915, last.pid);
        assert_eq!("idea", last.name);
        assert_eq!(5, last.listen.len());
        assert_eq!("localhost:10001", &last.listen[0]);
        assert_eq!("localhost:9123", &last.listen[4]);
    }

    #[test]
    fn should_parse_cwd_from_lsof(){
        let sample_output = r"p47915
fcwd
n/Users/cgatay/Library/Application Support/JetBrains/Toolbox/apps/IDEA-U/ch-0/181.4445.4/IntelliJ IDEA 2018.1 EAP.app/Contents/bin
ftxt
n/Users/cgatay/Library/Application Support/JetBrains/Toolbox/apps/IDEA-U/ch-0/181.4445.4/IntelliJ IDEA 2018.1 EAP.app/Contents/MacOS/idea
ftxt
n/usr/share/icu/icudt59l.dat
ftxt
n/Users/cgatay/Library/Application Support/JetBrains/Toolbox/apps/IDEA-U/ch-0/181.4445.4/IntelliJ IDEA 2018.1 EAP.app/Contents/jdk/Contents/Home/jre/lib/jli/libjli.dylib
";
        let maybe_cwd = CTPorts::read_cwd_from_lsof_output(Some(sample_output.to_string()));
        assert!(maybe_cwd.is_some());
        assert_eq!("/Users/cgatay/Library/Application Support/JetBrains/Toolbox/apps/IDEA-U/ch-0/181.4445.4/IntelliJ IDEA 2018.1 EAP.app/Contents/bin", maybe_cwd.unwrap())
    }
}
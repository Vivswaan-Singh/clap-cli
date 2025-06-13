
use std::error::Error;
use std::fs;
use std::{thread,sync::Arc};
use colored::*;

pub fn grep_parallel( pattern: String, files: Vec<String>)->Result<(),Box<dyn Error>>{
            let n=files.len();
            let cpus=num_cpus::get()/2;
            let (div,max_threads ) =  if n>cpus {
                ((n/cpus),cpus)
            }
            else{
                (1,n)
            };
            let mut handles=Vec::new();
            let file_names=Arc::new(files);
            let query=Arc::new(pattern);
            for t in 0..max_threads{
                let s=t*div;
                let e= if t==max_threads-1 {
                    n
                }
                else{
                    (t+1)*div
                };
                let temp=Arc::clone(&file_names);
                let word=Arc::clone(&query);
                let handle=thread::spawn(move ||->Result<Vec<String>, std::io::Error>{
                    let mut op=vec![];
                    for i in temp[s..e].iter(){
                        let content=fs::read_to_string(&i)?;
                        op.push(format!("Word {} occurs in Document {} in following lines ",word.red(),&i.yellow()));
                        op.append(&mut search(&word,&content));
                        op.push(String::from(""));
                    }
                    Ok(op)
                });
                handles.push(handle);
            }
            for h in handles{
                let op=h.join().unwrap()?;
                for lines in op{
                    println!("{}",lines);
                }
            }
            Ok(())
}

fn search<'a>(query: &'a str, file: &'a str)->Vec<String>{
    let mut ans=Vec::new();
    let mut cnt = 1;
    for line in file.lines(){
        if line.contains(query){
            let s=format!("Line {}: {}",cnt,line.trim());
            ans.push(s);
        }
        cnt+=1;
    }
    ans
}


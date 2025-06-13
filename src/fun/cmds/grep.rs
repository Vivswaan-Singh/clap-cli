
use std::error::Error;
use std::fs;
use std::{thread,sync::Arc};
use colored::*;

pub fn grep_parallel( pattern: String, files: Vec<String>)->Result<(),Box<dyn Error>>{
            let n=files.len();
            let div;
            let max_threads;
            if n>4 {
                div=n/4;
                max_threads=4;

            }
            else{
                div=1;
                max_threads=n;
            }
            let mut handles=Vec::new();
            for t in 0..max_threads{
                println!("");
                let word=pattern.clone();
                let s=t*div;
                let e;
                if t==max_threads-1 {
                    e=n;
                }
                else{
                    e=(t+1)*div;
                }
                let file_names=Arc::new(files.clone());
                let handle=thread::spawn(move ||->Result<Vec<Vec<String>>, std::io::Error>{
                    let mut op=vec![];
                    let temp=Arc::clone(&file_names);
                    for i in temp[s..e].iter(){
                        let content=fs::read_to_string(&i)?;
                        let mut ans=vec![(format!("Word {} occurs in Document {} in following lines ",word.red(),&i.yellow()))];
                        ans.append(&mut search(&word,&content));
                        ans.push(String::from(""));
                        op.push(ans);
                    }
                    Ok(op)
                });
                handles.push(handle);
            }
            
            for h in handles{
                let op=h.join().unwrap()?;
                for lines in op{
                    for line in lines{
                        println!("{}",line);
                    }
                }
            }
            Ok(())
}

fn search<'a>(query: &'a str, file: &'a str)->Vec<String>{
    let mut ans=Vec::new();
    let mut cnt = 1;
    for line in file.lines(){
        if line.contains(query){
            let mut s=format!("Line {}: ",cnt);
            s.push_str(line.trim());
            ans.push(s);
            cnt+=1;
        }
    }
    ans
}


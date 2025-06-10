

pub fn search<'a>(query: &'a str, file: &'a str)->Vec<&'a str>{
    let mut ans=Vec::new();
    for line in file.lines(){
        if line.contains(query){
            ans.push(line.trim());
        }
    }
    ans
}
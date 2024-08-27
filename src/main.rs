use std::{env, io};
use std::fmt::Error;
use std::fs::File;
use std::io::Read;

// exemplo od chat para dar destaque
// use regex::Regex;

// fn highlight_matches(input: &str, pattern: &str) -> String {
//     let re = Regex::new(pattern).unwrap();
//     let mut highlighted = String::new();
//     let mut last_end = 0;

//     for mat in re.find_iter(input) {
//         // Adiciona a parte da string que não corresponde ao padrão
//         highlighted.push_str(&input[last_end..mat.start()]);
        
//         // Adiciona a parte correspondente ao padrão com destaque
//         highlighted.push_str(&format!("[[{}]]", &mat.as_str()));
        
//         // Atualiza o índice do último final
//         last_end = mat.end();
//     }

//     // Adiciona a parte restante da string
//     highlighted.push_str(&input[last_end..]);

//     highlighted
// }

// fn main() {
//     let text = "The quick brown fox jumps over the lazy dog.";
//     let pattern = r"\b\w{4}\b"; // Padrão para encontrar palavras de 4 letras
//     let result = highlight_matches(text, pattern);
//     println!("{}", result);
// }

fn main() {
    // pega o valor da variável de ambiente do cli, 
    let case_insensitive: bool = if env::var("CASE_I").unwrap_or_else(|_| String::from("0")).parse::<u8>().unwrap() == 1 {true} else {false}; 
    // True: Case insensitive, false: Case Sensitive
    // TODO: Pegar por flag do cli também
    
    let (query, files_name) = read_args().unwrap();
    // println!("{query}, {files_name:?}");
    // println!("{}", read_file("teste.txt").unwrap());
    // run(&query, &file_name, &case_i);
}

fn run(query: &str, file_name: &Vec<String>, case: &bool){
    // fazer logica para criar mutliplas threads
    // let cont = read_file(&file_name).unwrap();
    // println!("Procurando por {query} em {file_name}:");
    // for (idx, linha) in search(query, &cont, case){
    //     println!("{file_name}:{idx}: {linha}");
    // }
}

// mudar para mais de um file por exemplo
fn read_args() -> Result<(String, Vec<String>), String>{
    let args:Vec<String> = env::args().collect();
    if args.len() < 3{
        return Err(String::from("Erro ao pegar mais argumentos")); // provavelmente tem jeitos melhores de fazer isso
    }
    let mut v: Vec<String> = Vec::new();
    for i in 2..args.len(){
        v.push(args[i].clone());
    }
    Ok::<(String, Vec<String>), String>((args[1].clone(),v))
    // primeiro o pattern(nao regex apenas busca normal)
}

fn read_file(file_name: &str) -> Result<String, io::Error>{
    let mut f = File::open(&file_name)?;
    let mut conteudo = String::new();
    f.read_to_string(&mut conteudo)?;   
    Ok(conteudo)
}


fn search<'a>(query: &str, contents: &'a str, case: &bool) -> Vec<(usize, String)> {
    let mut res: Vec<(usize, String)> = Vec::new();
    let hquery = highlight(&query);
    if *case {
        // resolver esse case de forma melhor
        let query = query.to_lowercase();
        for (idx, linha) in contents.lines().enumerate(){
            if linha.to_lowercase().contains(&query){
                // resolver o replace do case, tentar dar replace por cada indice
                res.push((idx + 1, linha.replace(&query, &hquery)));

            }
        }
    } else {
        for (idx, linha) in contents.lines().enumerate(){
            if linha.contains(query){
                res.push((idx + 1, linha.replace(query, &hquery)));
            }
        }
    }
    res
}

fn highlight(s: &str) -> String{
    let a = format!("{}{}{}", "\x1b[7m", s, "\x1b[0m"); // inverte cores
    a
}
use std::env;
use std::fs::File;
use std::io::prelude::*;

// mudar para mais de um file por exemplo
fn read_args() -> (String, String){
    let args:Vec<String> = env::args().collect();
    if args.len() < 3{
        println!("Deveria ter mais {} argumentos, no formato grp `query` `filename`", 3 -args.len());
        std::process::exit(1);
    }
    (args[1].clone(), args[2].clone())
}

fn read_file(file_name: &str) -> String{
    let mut f = File::open(&file_name).expect("Arquivo não encontrado");
    let mut conteudo = String::new();
    f.read_to_string(&mut conteudo).expect("Não pode ler por algum motivo");   
    conteudo
}

// retornar result depois, com erros melhorados
fn run(query: &str, file_name: &str, case: &bool){
    let cont = read_file(&file_name);
    println!("Procurando por {query} em {file_name}:");
    for (idx, linha) in search(query, &cont, case){
        println!("{file_name}:{idx}: {linha}");
    }
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

fn main() {
    // basicamente pega o valor e case ele não seja passado retorna zero
    let case_brute: u8 = env::var("CASE_I").unwrap_or_else(|_| String::from("0")).parse().unwrap(); 
    let mut case_i: bool = false; // case vai determinar se busca vai ser case_insensitive(true) ou não(false)
    if case_brute == 1 {
        case_i = true;
    }
    
    let (query, file_name) = read_args();
    run(&query, &file_name, &case_i);
}



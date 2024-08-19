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
    let x = search(query, &cont, case);
    for (idx, linha) in x{
        println!("{file_name}:{idx}: {linha}");
    }
}

fn search<'a>(query: &str, contents: &'a str, case: &bool) -> Vec<(usize, String)> {
    let mut res: Vec<(usize, String)> = Vec::new();
    if *case {
        let query = query.to_lowercase();
        for (idx, linha) in contents.lines().enumerate(){
            if linha.to_lowercase().contains(&query){
                res.push((idx + 1, String::from(linha)));
            }
        }
    } else {
        for (idx, linha) in contents.lines().enumerate(){
            if linha.contains(query){
                res.push((idx + 1, String::from(linha)));
            }
        }
    }
    res
}

fn main() {
    // melhorar isso daqui, muito aninhamento
    let case = match env::var("CASE_I") {
       Ok(dado) => 
        match dado.parse().unwrap() {
            0 => false,
            1 => true,
            _ => panic!("Invalid value")
       },
       Err(_) => false
    };
    let (query, file_name) = read_args();
    run(&query, &file_name, &case);
}

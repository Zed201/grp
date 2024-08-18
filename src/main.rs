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
fn run(query: &str, file_name: &str){
    let cont = read_file(&file_name);
    println!("{query}, {file_name} conteudo:\n {cont}");

}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

fn main() {
    let (query, file_name) = read_args();
    run(&query, &file_name);
}

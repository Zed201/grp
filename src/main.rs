use std::str::FromStr;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};
use std::{env, io};
// use std::fmt::{format, Error};
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

fn main() {
    let case_insensitive: bool = if env::var("CASE_I").unwrap_or_else(|_| String::from("0")).parse::<u8>().unwrap() == 1 {true} else {false}; 
    // True: Case insensitive, false: Case Sensitive
    // TODO: Pegar por flag do cli também

    let (query, files_name) = read_args().unwrap();
    let _ = run(&query, &files_name, &case_insensitive);
}

fn run(query: &str, files_name: &Vec<String>, case: &bool) -> Result<(), String>{
    let m_print: Arc<Mutex<bool>> = Arc::new(Mutex::new(false)); // mutex apenas para printar tudo junto de cada thread
    let mut handles = Vec::new();
    for i in files_name{
        
        let mutex_clone = Arc::clone(&m_print);
        // TODO:resolver esses clones 
        let file_name = i.clone();
        let case = case.clone();
        let query = String::from_str(query).unwrap(); // concertar isso para a referência funcionar

        // TODO:tratar erros pro threads e se não ele aborda tudo
        let t = thread::spawn(move || {
            let cont = read_file(&file_name).unwrap();
            let _unused = mutex_clone.lock().unwrap();
            println!("{}:", highlight(file_name.as_str()));
            for (linha, cont_linha) in search(query.as_str(), &cont, &case).unwrap(){
                println!("{linha}:{cont_linha}");
            }
        });

        handles.push(t);

    }
    for i in handles{
        i.join().unwrap();
    }
    Ok(())
    
}

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

fn read_file(file_name: &String) -> Result<String, io::Error>{
    let mut f = File::open(&file_name)?;
    let mut conteudo = String::new();
    f.read_to_string(&mut conteudo)?;   
    Ok(conteudo)
}

// TODO:optmizar melhor
fn search<'a>(query: &str, contents: &'a str, case: &bool) -> Result<Vec<(usize, String)>, String> {
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
    Ok(res)
}

// Reformular com format
fn highlight(s: &str) -> String{
    format!("{}{}{}", "\x1b[7m", s, "\x1b[0m") // inverte cores
}
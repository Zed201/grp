use std::fmt::format;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};
use std::{env, io};
use std::fs::File;
use std::io::{Read, Write};

use colored::{Color, Colorize};
use colored::Color::*;
use regex::RegexBuilder;

const file_color: Color = Green;


fn main() {
    let case_insensitive: bool = if env::var("CASE_I").unwrap_or_else(|_| String::from("0")).parse::<u8>().unwrap() == 1 {true} else {false}; 
    // !True: Case insensitive, false: Case Sensitive
    // fazer variavel de ambiente para colocar cor
    // TODO: Pegar por flag do cli também

    let (query, files_name) = read_args().unwrap();
    let _ = run(query, &files_name, case_insensitive);
}

fn run(query: String, files_name: &Vec<String>, case: bool) -> Result<(), String>{
    let m_print: Arc<Mutex<bool>> = Arc::new(Mutex::new(false)); // mutex apenas para printar tudo junto de cada thread
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    
    for i in files_name{
        
        let mutex_clone: Arc<Mutex<bool>> = Arc::clone(&m_print);
        
        let file_name: String = i.clone();
        let query = String::clone(&query); 
        
        // TODO:tratar erros pro threads e se não ele aborda tudo
        let t = thread::spawn(move || {
            match read_file(&file_name) {
                Ok(conteudo) => {
                    let p = search(&query, &conteudo, &case).unwrap();
                    let _unused = mutex_clone.lock().unwrap();
                    println!("{}:", highlight(&file_name, file_color));
                    for (linha, cont_linha) in p{
                        println!("{linha}:{cont_linha}");
                    }
                },
                Err(msg_erro) => {
                    println!("{}", highlight(&msg_erro, Red))
                }
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
        return Err(String::from("Quantidade de argumentos inválida")); // provavelmente tem jeitos melhores de fazer isso
    }
    let mut v: Vec<String> = Vec::new();
    for i in 2..args.len(){
        v.push(args[i].clone());
    }
    Ok::<(String, Vec<String>), String>((args[1].clone(),v))
    // primeiro o pattern(nao regex apenas busca normal)
}

fn read_file(file_name: &String) -> Result<String, String>{
    let mut conteudo = String::new();
    match File::open(file_name) {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut conteudo);
        },
        Err(_) => {
            return Err(format!("Arquio {} não pode ser aberto", file_name));
        }
    }
    Ok(conteudo)
}

// TODO:optmizar melhor
fn search<'a>(query: &str, contents: &'a str, case: &bool) -> Result<Vec<(usize, String)>, String> {
    let mut res: Vec<(usize, String)> = Vec::new();
    let pat = RegexBuilder::new(query).case_insensitive(*case).build().unwrap(); // implementação se case

    for (linha, linha_txt) in contents.lines().enumerate(){
        let mut tmp_str = String::new();
        let mut last_end = 0;
        let mut flag = false;
        for mat in pat.find_iter(linha_txt){
            flag = true;
            tmp_str.push_str(&linha_txt[last_end..mat.start()]);
            tmp_str.push_str(&highlight(mat.as_str(), Red));
            last_end = mat.end();
        }
        tmp_str.push_str(&linha_txt[last_end..]);
        if flag {
            res.push((linha, tmp_str));
        }
    }

    Ok(res)
}

// TOOD: add argumento de cor
fn highlight(s: &str, cor: Color) -> String{
    format!("{}", s.color(cor)) //TODO: Fazer variavel para controlar
}
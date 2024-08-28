use std::fmt::Error;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};
use std::env;
use std::io::Read;
use std::fs::File;

use colored::{Color, Color::*, Colorize, ColoredString};
use regex::RegexBuilder;

const FILE_COLOR: Color = Green;
const PAT_COLOR: Color = Magenta;

fn main() {
    let case_insensitive: bool = if env::var("CASE_I").unwrap_or_else(|_| String::from("0")).parse::<u8>().unwrap() == 1 {true} else {false}; 
    let cor_destaque = env::var("COR").unwrap_or_else(|_| String::new());
    // ! True: Case insensitive, false: Case Sensitive

    let (query, files_name) = read_args().unwrap_or_else(|err_msg| {
        println!("{}", err_msg.red());
        std::process::exit(1);
    });
    let _ = run(query, &files_name, case_insensitive, string_to_color(cor_destaque)); 
}

fn run(query: String, files_name: &Vec<String>, case: bool, cor: Color) -> Result<(), String>{
    let m_print: Arc<Mutex<bool>> = Arc::new(Mutex::new(false)); // mutex apenas para printar tudo junto de cada thread
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    
    for i in files_name{
        
        let mutex_clone: Arc<Mutex<bool>> = Arc::clone(&m_print);
        
        let file_name: String = i.clone();
        let query = String::clone(&query); 
        
        let t = thread::spawn(move || {
            match read_file(&file_name) {
                Ok(conteudo) => {
                    let p = search(&query, &conteudo, &case, &cor).unwrap();
                    let _unused = mutex_clone.lock().unwrap();
                    println!("{}:", highlight(&file_name, FILE_COLOR));
                    for (linha, cont_linha) in p {
                        println!("{linha}:{cont_linha}");
                    }
                },
                Err(msg_erro) => {
                    println!("{}", highlight(&msg_erro, Red));
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
fn search<'a>(query: &str, contents: &'a str, case: &bool, cor_destaque: &Color) -> Result<Vec<(ColoredString, String)>, Error> {
    let mut res: Vec<(colored::ColoredString, String)> = Vec::new();
    
    let pat = RegexBuilder::new(query).case_insensitive(*case).build().unwrap(); // implementação se case

    for (linha, linha_txt) in contents.lines().enumerate(){
        let mut tmp_str = String::new();
        let mut last_end = 0;
        let mut flag = false;
        for mat in pat.find_iter(linha_txt){
            flag = true;
            tmp_str.push_str(&linha_txt[last_end..mat.start()]);
            tmp_str.push_str(&highlight(mat.as_str(), *cor_destaque));
            last_end = mat.end();
        }
        tmp_str.push_str(&linha_txt[last_end..]);
        if flag {
            res.push((linha.to_string().red(), tmp_str));
        }
    }

    Ok(res)
}

fn highlight(s: &str, cor: Color) -> String{
    s.color(cor).to_string() 
}

fn string_to_color(cor: String) -> Color{
    match cor {
        _ if cor == "Blue" => Blue,
        _ if cor == "Red" => Red,
        _ if cor == "Green" => Green,
        _ if cor == "Yellow" => Yellow,
        _ if cor == "Magenta" => Magenta,
        _ => PAT_COLOR
    }
}
use std::fs;
use std::io::{self, Write};

use utils::{flush, read_line};

mod book;
mod utils;

fn main() {
    let mut running = true;
    let mut input_string = String::new();
    let file = match fs::File::create("database.txt") {
        Ok(file) => file,
        Err(error) => {
            panic!("Não foi possível inicializar o arquivo de dados: {}", error);
        }
    };

    while running {
        header();
        println!("1 - Buscar livros cadastrados");
        println!("2 - Cadastrar novo livro");
        println!("3 - Remover livro");
        println!("4 - Todos os livros");
        println!("5 - Sair");

        clear_screen();
        io::stdin().read_line(&mut input_string).unwrap();

        let option = match input_string.trim_end().parse::<i32>() {
            Ok(value) => value,
            Err(error) => {
                println!("Número inválido, erro {}. Dígite um número", { error });
                0
            }
        };

        input_string.clear();
        match option {
            1 => {}
            2 => register_book(&file),
            5 => running = false,
            _ => continue,
        }
    }
}

fn register_book(mut file: &fs::File) {
    println!("Cadastro do livro!");
    let mut input = String::new();
    read_line("Titulo do Livro: ", &mut input).unwrap();
    let title = input.trim().to_owned();
    input.clear();

    read_line("Autor: ", &mut input).unwrap();
    let author = input.trim().to_owned();
    input.clear();

    let mut publication_date: Option<chrono::NaiveDate> = None;

    while publication_date.is_none() {
        input.clear();
        read_line("Data da publicação (dd/mm/yyyy)", &mut input).unwrap();
        publication_date = match chrono::NaiveDate::parse_from_str(input.trim(), "%d/%m/%Y") {
            Ok(date) => Some(date),
            Err(error) => {
                println!("Data inválida: {}", error);
                None
            }
        };
    }

    let book = book::Book {
        title,
        author,
        publication_date: publication_date.unwrap(),
    };

    println!("Vai escrever: {}", book.serialize());
    match file.write(book.serialize().as_bytes()) {
        Ok(writed) => println!("Escreveu {writed} bytes"),
        Err(error) => {
            println!("Não foi possível salvar livro.\nLivro não foi salvo!\n erro: {error}")
        }
    };
}

fn header() {
    println!("### Sistema de Biblioteca ###\n");
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

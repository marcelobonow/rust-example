use std::io;

fn main() {
    let mut running = true;
    let mut input_string = String::new();

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
            2 => {}
            5 => running = false,
            _ => continue,
        }
    }
}

fn header() {
    println!("### Sistema de Biblioteca ###\n");
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

struct User (String, String, bool, String);

fn main() {
    
    let mut pessoa = User (String::from("Jurandir"), String::from("j.migliorini@gmail.com"), true, String::from("masculino"));
    println!("username: {}, email: {}, ativo: {} e genero: {}", pessoa.0, pessoa.1, pessoa.2, pessoa.3);
    pessoa.0 = String::from("j.migliorini");
    println!("username: {}, email: {}, ativo: {} e genero: {}", pessoa.0, pessoa.1, pessoa.2, pessoa.3);
}
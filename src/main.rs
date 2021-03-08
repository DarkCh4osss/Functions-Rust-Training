use std::io;
use std::process::Command;
use std::process;

fn main() {
    let mut num = String::new();
    let mut _invio: String;
    
    println!("Inserisci la lunghezza del vettore:");
    io::stdin().read_line(&mut num).expect("Errore");
    let num: u8 = num.trim().parse().expect("Errore");
    let mut vet: Vec<u8> = vec![];

    loop {
        clear_screen();
        let mut scelta = String::new();
        println!("Scegli:\n1. Inserimento\n2. Visualizzazione\n3. Eliminazione\n4. Ricerca\n5. Esci\n");
        io::stdin().read_line(&mut scelta).expect("Errore");
        let scelta: u8 = scelta.trim().parse().expect("Errore");

        match scelta {
            1 => {
                clear_screen();
                inserimento(num, &mut vet);
                println!("");
                pause();
            }
            2 => {
                clear_screen();
                visualizzazione(&vet);
                println!("");
                pause();
            }
            3 => {
                clear_screen();
                eliminazione(&mut vet);
                println!("");
                pause();
            }
            4 => {
                clear_screen();
                ricerca(&mut vet);
                println!("");
                pause();
            }
            5 => {
                clear_screen();
                pause();
                process::exit(0x0100);
            }
            _ => {
                clear_screen();
                println!("Scelta non valida!");
                pause();
            }

        }
    }
}

fn inserimento(num: u8, vet: &mut Vec<u8>) {

    for _i in 0..num {
        println!("Inserisci un numero:");
        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Errore");
        let x: u8 = x.trim().parse().expect("Errore");
        vet.push(x);
    }
}

fn visualizzazione(vet: &Vec<u8>) {
    print!("[ ");
    for _i in vet {
        print!("{} ", _i);
    }
    print!("]\n");
}

fn eliminazione(vet: &mut Vec<u8>) {
    println!("Inserisci il posto da eliminare:");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Errore");
    let x: usize = x.trim().parse().expect("Errore");

    vet.drain(x..(x+1));
}

fn ricerca(vet: &mut Vec<u8>) {
    println!("Inserisci il valore da cercare:");
    let mut _x = String::new();
    io::stdin().read_line(&mut _x).expect("Errore");
    let _x: u8 = _x.trim().parse().expect("Errore");
    let mut trovato: bool = false;

    for _i in vet {
        if _x == *_i {
            trovato = true;
        }
    }

    if trovato {
        println!("{} è stato trovato!", _x);
    } else {
        println!("{} non è stato trovato!", _x);
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn pause() {
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
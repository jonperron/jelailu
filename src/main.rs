extern crate chrono;
extern crate rusqlite;

use chrono::prelude::*;
use std::io;
use rusqlite::Connection;

#[derive(Debug, Clone)]
struct Book {
	id: u32,
	title : String,
	author: String,
	editor: String,
	year: u32,
	isbn : String,
}

fn main() {
	// create connection to the database
	let conn = Connection::open("library.db").unwrap();
	// initialize database
	conn.execute("CREATE TABLE IF NOT EXISTS books(
					id 				INTEGER PRIMARY KEY,
					title 			TEXT NOT NULL,
					author			TEXT NOT NULL,
					editor			TEXT NOT NULL,
					year 			INT NOT NULL,
					isbn			TEXT NOT NULL
				)",&[]).unwrap();

	println!("Jelailu, La Gestion de votre Bibliothèque facile !");
	println!("v. 0.0.1, Avril 2017");
	main_menu();
}


fn main_menu() {
	println!("--------------");
	println!("1) Ajouter un livre.");
	println!("2) Vos livres.");
	println!("3) Quitter.");
	println!("--------------");
	println!("Votre choix ?");

	loop {
		let mut user_choice = String::new();

		io::stdin().read_line(&mut user_choice)
						.expect("Failed to read line");

		let user_choice: u32 = match user_choice.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("Vous avez choisi le : {}", user_choice);

		match user_choice {
			1 => { insert(); break;}
			2 => { book_list(); break;}
			3 => { break; }
			_ => { println!("Choix invalide, recommencez !");}
		}
	}
}

fn insert(){
	println!("Ajouter un livre.");
	let conn = Connection::open("library.db").unwrap();

	println!("Titre:");
	let mut title = String::new();
	io::stdin().read_line(&mut title).expect("Failed to read title.");
	let title = title.trim();

	println!("Auteur:");
	let mut author = String::new();
	io::stdin().read_line(&mut author).expect("Failed to read author.");
	let author = author.trim();

	println!("Editeur:");
	let mut editor = String::new();
	io::stdin().read_line(&mut editor).expect("Failed to read editor.");
	let editor = editor.trim();

	println!("Année de parution:");
	let mut year = String::new();
	io::stdin().read_line(&mut year).expect("Failed to read year.");
	let year = year.trim();

	let year: u32 = match year.trim().parse() {
		Ok(num) => num,
		Err(_) => 2017,
	};

	println!("ISBN:");
	let mut isbn = String::new();
	io::stdin().read_line(&mut isbn).expect("Failed to read isbn.");
	let isbn = isbn.trim();

	conn.execute("INSERT INTO books (title, author, editor, year, isbn) VALUES (?1, ?2, ?3, ?4, ?5)", &[&title,&author,&editor,&year,&isbn]).unwrap();
	println!("Livre ajouté!");

	book_list();
}

fn book_list() {
	let conn = Connection::open("library.db").unwrap();
	let mut book_list = conn.prepare("SELECT * FROM books").unwrap();

	let book_iter = book_list.query_map(&[], |row| {
		Book {
			id: row.get(0),
			title: row.get(1),
			author: row.get(2),
			editor: row.get(3),
			year: row.get(4),
			isbn: row.get(5)
		}
	}).unwrap();

	println!("Vos livres:");
	for book in book_iter {
		let book_ = book.unwrap();
		println!("ID : {:?}, Titre : {:?}, Auteur: {:?}", &book_.id, &book_.title, &book_.author);
		println!("");
	}
	println!("");
	book_menu();
}

fn book_menu() {
	println!("--------------");
	println!("1) Ajouter un livre");
	println!("2) Mettre à jour un livre");
	println!("3) Supprimer un livre");
	println!("4) Menu principal");
	println!("--------------");
	println!("Votre choix ?");

	loop {
		let mut user_choice = String::new();

		io::stdin().read_line(&mut user_choice)
						.expect("Failed to read line");

		let user_choice: u32 = match user_choice.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("Vous avez choisi le : {}", user_choice);

		match user_choice {
			1 => { insert(); break;}
			2 => { update(); break;}
			3 => { delete();break; }
			4 => { main_menu(); break;}
			_ => { println!("Choix invalide, recommencez !");}
		}
	}
}


fn update() {
	println!("ID du livre à modifier:");
	let conn = Connection::open("library.db").unwrap();

	let mut book_id = String::new();
	io::stdin().read_line(&mut book_id).expect("Failed to read id.");

	let book_id: u32 = match book_id.trim().parse() {
		Ok(num) => num,
		Err(_) => 0,
	};

	if book_id != 0 {
		let book = conn.query_row("SELECT * FROM books WHERE id = (?)",&[&book_id], |row| { //ToDos : handle exception
			Book {
				id: row.get(0),
				title: row.get(1),
				author: row.get(2),
				editor: row.get(3),
				year: row.get(4),
				isbn: row.get(5)
			}
		}).expect("Livre introuvable !");
		
		println!("Titre ({:?}) :", &book.title);
		let mut title = String::new();
		io::stdin().read_line(&mut title).expect("Failed to read title.");
		if title != "\n" {
			let title = title.trim();
		} else {
			let title = &book.title;
		}

		println!("Auteur ({:?}) :", &book.author);
		let mut author = String::new();
		io::stdin().read_line(&mut author).expect("Failed to read title.");
		if author != "\n" {
			let author = author.trim();
		} else {
			let author = &book.author;
		}		

		println!("Editeur ({:?}) :", &book.editor);
		let mut editor = String::new();
		io::stdin().read_line(&mut editor).expect("Failed to read title.");
		if editor != "\n" {
			let editor = editor.trim();
		} else {
			let editor = &book.editor;
		}	

		println!("Année ({:?}) :", &book.year);
		let mut year = String::new();
		io::stdin().read_line(&mut year).expect("Failed to read title.");
		if year != "\n" {
			let year = year.trim();
		
			let year: u32 = match year.trim().parse() {
				Ok(num) => num,
				Err(_) => 2017,
			};
		} else {
			let year = &book.year;
		}

		println!("ISBN ({:?}) :", &book.isbn);
		let mut isbn = String::new();
		io::stdin().read_line(&mut isbn).expect("Failed to read title.");
		if isbn != "\n" {
			let isbn = isbn.trim();
		} else {
			let isbn = &book.isbn;
		}

		conn.execute("UPDATE books SET title = ?1, author = ?2, editor = ?3, year = ?4, isbn = ?5) WHERE id = ?0", &[&book_id,&title,&author,&editor,&year,&isbn]).unwrap();
	} else {
		println!("Livre introuvable !");
		println!("");		
	}

	book_list();
}

fn delete() {
	println!("ID du livre à supprimer:");
	let conn = Connection::open("library.db").unwrap();

	let mut book_id = String::new();
	io::stdin().read_line(&mut book_id).expect("Failed to read id.");

	let book_id: u32 = match book_id.trim().parse() {
		Ok(num) => num,
		Err(_) => 0,
	};

	if book_id != 0 {
		match conn.execute("DELETE FROM books WHERE id = ?1", &[&book_id]) {
			Ok(updated) =>  println!("Livre supprimé !"),
			Err(err) => println!("Livre introuvable !"),
		};
	} else {
		println!("Livre introuvable !");
		println!("");
	}

	book_list();
}

//-----------------------------------------------------------------------------
//--- Test threads 
//-----------------------------------------------------------------------------
//--- Author: Kornilov LN (Starmark)
//--- Github: https://github.com/KornilovLN
//--- e-mail: ln.KornilovStar@gmail.com
//--- e-mail: ln.starmark@ekatra.io
//--- e-mail: ln.starmark@gmail.com
//--- date:   2.09.2023 01:39:00
//-----------------------------------------------------------------------------
//--- Программа тестирования работы threads, mutex,.. языка RUST
//-----------------------------------------------------------------------------

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

extern crate ansi_escapes;
extern crate ansi_term;

use ansi_term::Colour;
use ansi_term::Colour::{Black, Blue, Cyan, Green, Red, White, Yellow};
use ansi_term::Style;

mod md_about;

//--- Полученная сервером строка мщжет начинаться с "Первый!:"   "Второй!:"
fn out(dt: String) {
	for tok in dt.split(" "){
    	if tok == "Первый!:" {
    		println!("\tПоток: {}", Colour::Green.bold().paint(&dt));
    	}else if tok == "Второй!:" {
    		println!("\tПоток: {}", Colour::Yellow.bold().paint(&dt));
    	} 	
   	}
}

//--- В общей памяти 2 вектора v и vst
//--- 2 потока передают свои вектора в гл. поток на прием
//--- данные векторов в каждом потоке перемещаются (move)
//--- что не дает гл потоку их использовать после перемещения
fn main() {

	let about = md_about::StAbout::new(
    	"Leonid", 
		"Nikolaevich", 
		"Kornilov",
		"Kornilov LN (Starmark)", 
		"https://github.com/KornilovLN/threads.git",
		"ln.KornilovStar@gmail.com",
		"30.08.2023 19:24:00",
    );	
	about.out();
	about.waiter(2);
	
	//--- приветствуем
	println!("\n\t{}","2 потока по каналам tx, tx1 шлют вектора v и vst в гл. поток"); 
	println!("\n\t{}", Colour::Red.bold().paint("Тест работы потоков".to_string()));
	

	let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
	let vstr = vec![String::from("раз"),
					String::from("два"),
					String::from("три"),
					String::from("четыре"),
					String::from("пять"),
					String::from("шесть"),
					String::from("семь"),
					String::from("восемь"),
	];

	//--- каналы: tx и tx1 передатчики, а rx - приемник
	let (tx, rx) = mpsc::channel();
	let tx1 = mpsc::Sender::clone(&tx);

	//--- этот поток использует канал tx и работает с вектором vstr	//--- sleep == 2 sec
	thread::spawn(move || {
		for elm in vstr {
			let mut val = String::from("Первый!: ");
			val.push_str(&elm);
			tx.send(val).unwrap();

			thread::sleep(Duration::from_secs(2));
		}
	});

	//--- этот поток использует канал tx1 и работает с вектором v	//--- sleep == 1 sec
	thread::spawn(move || {
		for el in v {
			let mut vl = String::from("Второй!: ");
			vl.push_str(&el.to_string());
			tx1.send(vl).unwrap();

			thread::sleep(Duration::from_secs(1));
		}
	});

	//--- главный поток выводит приход в консоль
	for received in rx {
		out(received);
	}

	//====================================================================

	use std::sync::{Mutex, Arc};

	println!("\t{}", 
			 Colour::Red.bold()
			 .paint("\nПример 2: 10 потоков ув. счетчик с мьютексом и с Atomic умного указателя"
			 .to_string()));


	//--- вот counter под мьютексом и умным указателем
	let counter = Arc::new(Mutex::new(0));

	//--- контейнер для создаваемых потоков
	let mut handles = vec![];

	//--- всего их будет 10
	for _ in 0..10 {
		//--- атомарно клонируем счетчик
		let counter = Arc::clone(&counter);

		//--- создаем следующий поток и
		let handle = thread::spawn(move || {
			//--- блокируем счетчик				
			let mut num = counter.lock().unwrap();
			//--- наращиваем его
			*num += 1;
			//--- печатаем для наглядности
			println!("{}", *num);
			//--- засыпаем, давая ранее созданным потокам работать
			thread::sleep(Duration::from_secs(1));
		});
		//--- вновь созданный поток помещаем в контейнер
		handles.push(handle);
	}
	
	//--- останавливаем потоки
	for handle in handles {
		handle.join().unwrap();
	}

	//--- печатаем результат
	println!("Результат: {}", *counter.lock().unwrap());

}



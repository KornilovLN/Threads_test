//-----------------------------------------------------------------------------
//--- Модуль About 
//-----------------------------------------------------------------------------
//--- Author: Kornilov LN (Starmark)
//--- Github: https://github.com/KornilovLN
//--- e-mail: ln.KornilovStar@gmail.com
//--- e-mail: ln.starmark@ekatra.io
//--- e-mail: ln.starmark@gmail.com
//--- date:   14.08.2023 13:10:00
//-----------------------------------------------------------------------------
//--- 
//--- 
//-----------------------------------------------------------------------------

extern crate datetime;
use std::{thread, time};
use chrono::Utc;
//use datetime::Duration;


#[derive(Debug)]
pub struct StAbout {
	firstname: String,	//--- имя
	secondname: String,	//--- отчество
	mainname: String,	//--- фамилия
	author: String,		//--- полный идентификатор автора
	github: String,		//--- Github 
	e_mail: String,		//--- почтовый ящик
	datetime: String,	//--- 14.08.2023 13:10:00
}

impl StAbout {
    pub fn new(
		firstn: &'static str, 
		secondn: &'static str, 
		mainn: &'static str,
		auth: &'static str, 
		gith: &'static str,
		mail: &'static str,
		dttm: &'static str,
		) -> StAbout {
			  Self{ firstname: firstn.to_string(), 
			  		secondname: secondn.to_string(), 
					mainname: mainn.to_string(),
					author: auth.to_string(), 
					github: gith.to_string(),
					e_mail: mail.to_string(),
					datetime: dttm.to_string(),
			  }  
	}
}

impl StAbout {
	pub fn waiter(&self, pause: u64) {
		thread::sleep(time::Duration::from_secs(pause));
	}
}


impl StAbout {
	pub fn get_datetime(&self) -> i64 {
		let dt = Utc::now();
		let timestamp: i64 = dt.timestamp();
		println!("Current timestamp is {}", timestamp);
		
		timestamp		
	}
}

impl StAbout {
	pub fn out(&self) {
	    Self::target(self);
	    println!("\t--- About ---------------------------------------------------------------");
		println!("\tAuthor:      {}", self.author);
		println!("\tFirst name:  {}", self.firstname);
		println!("\tSecond name: {}", self.secondname);
		println!("\tMain name:   {}", self.mainname);
		println!("\tGithub:      {}", self.github);
		println!("\te-mail:      {}", self.e_mail);
		println!("\tDate Time:   {}", self.datetime);
		println!("\t-------------------------------------------------------------------------");
	}
	
	pub fn tostring(&self) -> String {

	    let mut s = String::with_capacity(256);
	    s.push_str("Structure StAbout");
	    
	    s.push_str("\nAuthor:      ");
	    s.push_str(&self.author);
	    s.push_str("\nFirst name:  ");
	    s.push_str(&self.firstname);
	    s.push_str("\nSecond name: ");
	    s.push_str(&self.secondname);

		s.push_str("\nMain name:   ");
	    s.push_str(&self.mainname);
	    s.push_str("\nGithub:      ");
	    s.push_str(&self.github);
	    s.push_str("\ne-mail:      ");
	    s.push_str(&self.e_mail);
	    s.push_str("\nDate Time:   ");
	    s.push_str(&self.datetime);

	    s
	}
}

impl StAbout {
	pub fn target(&self) {
		println!("\t-------------------------------------------------------------------------");
		println!("\tThreads");
		println!("\n\tПрограмма тестирования работы threads, mutex,.. языка RUST");
		println!("\t-------------------------------------------------------------------------");
	}
}

use crossterm::{execute, event, cursor, terminal};
use std::sync::{Arc, Mutex};
use std::io::prelude::*;
use std::io::{stdout, Write, BufReader};
use std::net::{TcpStream, TcpListener};
use std::time::{Duration, Instant};
use std::thread::{self};
use std::fs;

#[derive(Debug, PartialEq)]
enum State { Edit, Quit, Save, Load, Wait }

#[derive(Debug, Clone)]
enum History {
    // primitive
    InsertChar(usize, usize, char),             // line, pos, char
    DelChar(usize, usize),                      // line, pos
    InsertLine(usize),                          // line
    DelLine(usize),                             // line

    // evaluated
    InsertString(usize, usize, String, usize),  // line, pos, content, size
    DelString(usize, usize, usize),             // line, pos, size
}

impl PartialEq for History {
    fn eq(&self, other: &Self) -> bool {
        match self {
            History::InsertChar(_, _, _) => { if let History::InsertChar(_, _, _) = other { true } else { false } },
            History::InsertLine(_) => { if let History::InsertLine(_) = other { true } else { false } },
            History::DelChar(_, _) => { if let History::DelChar(_, _) = other { true } else { false } },
            History::DelLine(_) => { if let History::DelLine(_) = other { true } else { false } },
            _ => false
        }
    }
}

struct EditorMemory {
    mem: Vec<String>, 
    log: Vec<History>,
    con: Vec<History>,
    state: State, 
    control: String, 
    line: usize, 
    pos: usize,
    parse_ok: bool,
    quit_ok: bool,
}

impl EditorMemory {
    fn new() -> Self {
        EditorMemory { 
            mem: vec![String::new()], 
            log: Vec::new(), 
            con: Vec::new(),
            state: State::Wait, 
            control: String::new(), 
            line: 0, 
            pos: 0,
            parse_ok: false,
            quit_ok: false
        }
    }

    fn read_to_control(&mut self) {
        if let event::Event::Key(event) = event::read().unwrap() {
            match event.code {
                event::KeyCode::Enter => { self.parse_ok = true; },
                event::KeyCode::Backspace => { if self.control.len() > 0 { self.control.pop(); } },
                event::KeyCode::Esc => { self.quit_ok = true; }
                event::KeyCode::Char(ch) => { self.control.push(ch); }
                _ => {}
            }
        }
    }

    fn parse_control(&mut self) {
        self.state = match self.control.as_str() {
            "e" | "E" | "Edit" | "EDIT" => State::Edit,
            "s" | "S" | "Save" | "SAVE" => State::Save,
            "l" | "L" | "Load" | "LOAD" => State::Load,
            "q" | "Q" | "Quit" | "QUIT" => State::Quit,
            _ => State::Wait,
        };

        self.control.clear();
        self.parse_ok = false;
    }

    fn condense_log(&mut self) -> String {

        let mut condensed: Vec<History> = Vec::new();
        let mut ins_str = String::new();
        let mut del_buf = 0;

        let mut ptr_a: usize = 0;
        let mut ptr_b: usize = 0;

        while ptr_a < self.log.len() {
            match self.log[ptr_a] {
                History::InsertChar(line, pos, ch) => {
                    ins_str.push(ch);
                    ptr_b = ptr_a + 1;

                    while ptr_b < self.log.len() {
                        if let History::InsertChar(other_line, other_pos, other_ch) = self.log[ptr_b] {
                            if line == other_line && other_pos - pos == ins_str.len() {
                                ins_str.push(other_ch);
                            } else { break; }
                        } else { break; }

                        ptr_b += 1;
                    }

                    condensed.push(History::InsertString(line, pos, ins_str.clone(), ins_str.len()));
                    ins_str.clear();
                    ptr_a = ptr_b;
                },
                History::DelChar(line, pos) => {
                    del_buf = 1;
                    ptr_b = ptr_a + 1;

                    while ptr_b < self.log.len() {
                        if let History::DelChar(other_line, other_pos) = self.log[ptr_b] {
                            if line == other_line && pos - other_pos == del_buf {
                                del_buf += 1;
                            } else { break; }
                        } else { break; }

                        ptr_b += 1;
                    }

                    condensed.push(History::DelString(line, pos + 1 - del_buf, del_buf));
                    ptr_a = ptr_b;
                },
                _ => { 
                    condensed.push(self.log[ptr_a].clone()); 
                    ptr_a += 1;
                }
            }
        }

        self.con = condensed;
        let mut con_str = self.con.iter().map(|e| format!("{e:?}")).collect::<Vec<String>>().join("|");
        con_str.push('\n');

        self.log.clear();
        return con_str;
    }

    #[inline(always)]
    fn flush(&mut self) {

        execute!(stdout(), cursor::Hide).unwrap();
        print!("\x1B[2J\x1B[1;1H");
        // terminal::Clear(terminal::ClearType::All);

        // control
        println!("[{:?}::{}] {{Commands: Edit; Load; Save; Exit}} \r", self.state, self.control);

        // cursor?
        // let lines = mem.split('\n').collect::<Vec<&str>>();
        println!("{}:{} | {}:{}\r", self.line, self.pos, self.mem.len(), self.mem[self.line].len());

        // body
        for i in 0..self.mem.len() {
            if i < self.mem.len() - 1 { println!("{}\r", self.mem[i]); }
            else { println!("{}", self.mem[i]); }
        }

        // log
        // println!("\r===================\r");
        // for history in self.log.iter() {
        //     println!("{:?}\r", history);
        // }

        // log consumption
        // self.condense_log();

        println!("\n\r== [ LOG : Size={} ] =================\r", {self.log.len()});
        for history in self.con.iter() {
            println!("{:?}\r", history);
        }

        // cursor reposition
        if self.state == State::Edit {
            execute!(
                stdout(),
                // cursor::MoveTo(cursor_pos.0, cursor_pos.1),
                cursor::MoveTo(self.pos as u16, (self.line + 2) as u16),
                cursor::Show
            ).unwrap();
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let interval = Duration::from_millis(500);
    let mut next = Instant::now() + interval;
    let mut cmd_recv: Vec<String> = Vec::new();

    loop {
        if Instant::now() > next {
            // println!("[{:?}] server read", stream);
            let mut reader = BufReader::new(&mut stream);
            let mut resp = String::new();
            
            // loop {
            match reader.read_line(&mut resp) {
                Ok(_) => { 
                    if resp.trim().len() > 0 { 
                        // println!("server recv: {}\r", resp); 
                        cmd_recv.push(resp);
                    } 
                },
                Err(_) => { break; } // println!("\terr: server could not read: {:?}", e); break; }
            }
            // }

            // println!("[{:?}] server write", stream);
            stream.write_all(format!("server write at {:?}\n", Instant::now()).as_bytes()).expect("server cannot write");
            stream.flush().expect("server could not flush");

            println!("\n\r== [ CMD ] =================\r");
            for (l, cmd) in cmd_recv.iter().enumerate() {
                println!("{}: {}\r", l, cmd);
            }
            
            next += interval;
        }
    }
}

fn start_server() {
    // println!("server: new server started");
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();

    thread::spawn(move || {
        loop {
            match server.accept() {
                Ok((s, _)) => { 
                    println!("server: client accepted at {:?}", s);
                    thread::spawn(move || handle_connection(s)); 
                },
                Err(_) => (),
            }
        }
    });
}

fn start_client(mut editor_memory: EditorMemory) {
    println!("client: new client started");
    let mut client = TcpStream::connect("127.0.0.1:8080").unwrap();
    let content_mutex = String::from("\n");
    let arc = Arc::new(Mutex::new(content_mutex));
    let arc_shared = arc.clone();

    thread::spawn(move || {
        let interval = Duration::from_secs(1);
        let mut next = Instant::now() + interval;
    
        loop {
            if Instant::now() > next {
                // println!("[{:?}] client read", client);
                let mut read_buf: String = String::new();
                
                client.set_read_timeout(Some(Duration::from_secs(1)))
                    .expect("client cannot set read timeout");
                let mut reader = BufReader::new(&mut client);
                match reader.read_line(&mut read_buf) {
                    Ok(_) => { 
                        // println!("client recv: {}", read_buf); 
                },
                    Err(e) => {     
                        // println!("\terr: client cannot read: {:?}", e); 
                    }
                };
    
                // println!("[{:?}] client write", client);
                // client.write_all(format!("client write at {:?}\n", Instant::now()).as_bytes()).expect("client cannot write");
                let mut content = arc_shared.lock().unwrap();
                client.write_all(content.as_bytes())
                    .expect("client cannot write");
                content.clear();
                client.flush().expect("client could not flush");
                next += interval;
            }
        }
    });

    loop {
        editor_memory.flush();

        match editor_memory.state {
            State::Wait => {         
                editor_memory.read_to_control(); // &mut control, &mut parse_ok, &mut quit_ok);

                if editor_memory.quit_ok { break; }
                
                if editor_memory.parse_ok {
                    editor_memory.parse_control();
                    editor_memory.flush();
                }
            },
            State::Quit => { break; },
            State::Load => {
                editor_memory.read_to_control();

                if editor_memory.parse_ok {
                    let filename = editor_memory.control.to_string();
                    editor_memory.control.clear();
                    editor_memory.parse_ok = false;
                    let fs_res = fs::read_to_string(filename);
                    match fs_res {
                        Ok(s) => { 
                            editor_memory.mem = s.split('\n').map(|e| e.to_string()).collect(); 
                            editor_memory.line = editor_memory.mem.len() - 1;
                            if editor_memory.mem.len() > 0 {
                                editor_memory.pos = editor_memory.mem.last().unwrap().len();
                            } else {
                                editor_memory.pos = 0;
                            }
                        }
                        Err(e) => {}
                    }

                    editor_memory.state = State::Wait;
                }
                
                editor_memory.flush();
            },
            State::Save => {
                editor_memory.read_to_control();

                if editor_memory.parse_ok {
                    let filename = editor_memory.control.to_string();
                    editor_memory.control.clear();
                    editor_memory.parse_ok = false;

                    let mut fs_res = fs::File::create(filename).unwrap();
                    for line in editor_memory.mem.clone().iter() {
                        fs_res.write_all(format!("{}\r", line).as_bytes()).unwrap();
                    }

                    editor_memory.state = State::Wait;
                }

                editor_memory.flush();
            }

            State::Edit => {
                loop {
                    if let event::Event::Key(event) = event::read().unwrap() {
                        match event.code {
                            event::KeyCode::Left => {  
                                if editor_memory.pos > 0 { editor_memory.pos -= 1; }
                            },
                            event::KeyCode::Right => { 
                                if editor_memory.pos < editor_memory.mem[editor_memory.line].len() { editor_memory.pos += 1; }
                            },
                            event::KeyCode::Up => {
                                if editor_memory.line > 0 {
                                    editor_memory.line -= 1;
                                    if editor_memory.pos > editor_memory.mem[editor_memory.line].len() { editor_memory.pos = editor_memory.mem[editor_memory.line].len(); }
                                }
                            },
                            event::KeyCode::Down => {
                                if editor_memory.line < editor_memory.mem.len() - 1 {
                                    editor_memory.line += 1;
                                    if editor_memory.pos > editor_memory.mem[editor_memory.line].len() { editor_memory.pos = editor_memory.mem[editor_memory.line].len(); }
                                }
                            },
                            event::KeyCode::Enter => { 
                                editor_memory.mem.insert(editor_memory.line + 1, String::new());
                                editor_memory.log.push(History::InsertLine(editor_memory.line + 1));
                                editor_memory.line += 1;
                                editor_memory.pos = 0;
                            },
                            event::KeyCode::Backspace => { 
                                if editor_memory.mem[editor_memory.line].len() > 0 { 
                                    editor_memory.mem[editor_memory.line].remove(editor_memory.pos - 1); 
                                    editor_memory.log.push(History::DelChar(editor_memory.line, editor_memory.pos - 1));
                                    editor_memory.pos -= 1;
                                } else if editor_memory.mem.len() > 1 {
                                    editor_memory.mem.remove(editor_memory.line);
                                    editor_memory.log.push(History::DelLine(editor_memory.line));
                                    editor_memory.line -= 1;
                                    editor_memory.pos = editor_memory.mem[editor_memory.line].len();
                                }
                            },
                            event::KeyCode::Char(ch) => { 
                                editor_memory.mem[editor_memory.line].insert(editor_memory.pos, ch); 
                                editor_memory.log.push(History::InsertChar(editor_memory.line, editor_memory.pos, ch));
                                editor_memory.pos += 1;
                            }
                            event::KeyCode::Esc => { 
                                editor_memory.state = State::Wait; 
                                editor_memory.flush();
                                break; 
                            }
                            _ => {}
                        }
                    }

                    editor_memory.flush();

                    if editor_memory.log.len() >= 20 {
                        let mut content = arc.lock().unwrap();
                        *content = editor_memory.condense_log();
                    }
                }
            }
        }     
    }
}

fn main() {
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");

    let mut editor_memory = EditorMemory::new();
    start_server(); 
    
    start_client(editor_memory);
}
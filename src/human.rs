use std::io;

#[inline]
fn trim_newline(s: &mut String) {
    #[cfg(target_family = "unix")]
    s.truncate(s.len() - 1);
    #[cfg(target_os = "windows")]
    s.truncate(s.len() - 2);
}

pub fn human_move() -> (usize, usize){
    let mut str_buffer = String::new();    
    io::stdin().read_line(&mut str_buffer).unwrap();
    trim_newline(&mut str_buffer);
    //todo!("input validation");

    let numbered = str_buffer.parse::<usize>().unwrap();
    let x = (numbered % 10) - 1;
    let y = (numbered / 10) - 1;

    (x, y)
}


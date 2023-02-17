use std::io;

#[inline]
fn trim_newline(s: &mut String) {
    #[cfg(target_family = "unix")]
    s.truncate(s.len() - 1);
    #[cfg(target_os = "windows")]
    s.truncate(s.len() - 2);
}

fn get_user_input(str_buffer: &mut String) -> (usize, usize){
    io::stdin().read_line(str_buffer).unwrap();
    trim_newline(str_buffer);
    //todo!("input validation");

    let numbered = str_buffer.parse::<usize>().unwrap();
    let x = (numbered % 10) - 1;
    let y = (numbered / 10) - 1;

    (x, y)
}

pub(crate) fn human_move() -> (usize, usize) {
    let mut str_buffer = String::new();    
    get_user_input(&mut str_buffer)
}

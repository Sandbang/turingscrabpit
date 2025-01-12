
use actions::decrement;
use actions::increment;
use actions::loop_bypass;
use actions::read;
use actions::right;
use actions::left;
mod actions;
fn main() {
    let input = ">++[<+++>-]<.".to_owned();
    let tape:[u8; 30000] = [0u8;30000];
    //mutability is required for arrays, rust doesn't support the creation of monads that would this possibility.
    let mut tape = tape.to_vec();
    let pointer: usize = 0;
    let cur_pos = 0;
    interpreter(tape, &string_to_char(input), pointer, cur_pos);
}   

fn interpreter(tape: Vec<u8>, input: &Vec<char>, ptr: usize, cur_pos: usize) -> (Vec<u8>, &Vec<char>, usize, usize) {
    if cur_pos < input.len() {
        match input[cur_pos] {
            '>' => {
                let ptr = right(ptr); 
                return interpreter(tape.clone(), &input, ptr, cur_pos + 1);
            },
            '<' => {
                println!("<");
                let ptr = left(ptr);
                return interpreter(tape.clone(), &input, ptr, cur_pos + 1);
            },
            '+' => {
                let (ptr, tape) = increment(tape.clone(), ptr);
                return interpreter(tape.clone(), &input, ptr, cur_pos + 1);
            },
            '-' => {
                let (ptr, tape) = decrement(tape.clone(), ptr);
                return interpreter(tape.clone(), &input, ptr, cur_pos + 1);
            },
            '.' =>{
                println!("{}", tape[ptr] as char);
                return interpreter(tape.clone(), &input, ptr, cur_pos + 1);
            },
            ',' => {
                let (ptr, tape) = read(tape.clone(), ptr);
                return interpreter(tape.clone(), &input, ptr, cur_pos + 1);
            },
            '[' => {
                if tape[ptr] == 0 {                  
                    let ( input, cur_posS, bracket_count) = loop_bypass( input,  cur_pos + 1, 0);
                    return interpreter(tape, input, ptr, cur_posS);
                } 
                else {
                    let (tapeS, input, ptrS, cur_posS) = interpreter(tape.clone(), &input, ptr, cur_pos + 1);
                    return interpreter(tapeS, input, ptrS, cur_pos);
                }
            },
            ']' => {
                return (tape, input, ptr, cur_pos);
            },
            _ => {
            }
        } 
    }
    return (tape, input, ptr, cur_pos + 1);
}
fn string_to_char(inp: String) -> Vec<char> {
    return inp.chars().collect(); 
}


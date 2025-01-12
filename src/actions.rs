pub const memory: usize = 30000;

pub fn left (ptr: usize) -> usize {
    if ptr == 0 {
        return memory-1;
    }
    else {
        return ptr - 1;
    }
}

pub fn right (ptr: usize) -> usize {
    if ptr == memory-1 {
        return 0;
    }
    else {
        return ptr + 1;
    }
}


pub fn increment (mut tape: Vec<u8>, ptr: usize) -> (usize, Vec<u8>) {
    if tape[ptr] == 255 {
        tape[ptr] = 0;
    }
    else {
        tape[ptr] = tape[ptr] + 1;
    }
    return (ptr, tape);
}

pub fn decrement (mut tape: Vec<u8>, ptr: usize) -> (usize, Vec<u8>) {
    if tape[ptr] == 0 {
        tape[ptr] = 255;
    }
    else {
        tape[ptr] = tape[ptr] - 1;
    }
    return (ptr, tape);
}

//this would also require a monad which you cannot implement in rust, so it uses regular stateful methods.
pub fn read(mut tape: Vec<u8>, ptr: usize) -> (usize, Vec<u8>) {
    let mut inp = "".to_owned();
    std::io::stdin().read_line(&mut inp).expect("Read line failed.");
    if inp.chars().nth(0).unwrap().is_ascii() {
        tape[ptr] = inp.chars().nth(0).unwrap() as u8;; 
        return (ptr, tape);
    }
    else {
        panic!("Not ASCII");
    }
}

pub fn loop_bypass(input: &Vec<char>, cur_pos: usize, bracket_count: usize) -> (&Vec<char>, usize, usize) {
    match input[cur_pos] {
        '[' => {
            return loop_bypass( input,  cur_pos  + 1, bracket_count + 1);
        }
        ']' => {
            if bracket_count == 0 {
                return ( input,  cur_pos  + 1, bracket_count);
                
            }
            else {
                return loop_bypass( input,  cur_pos  + 1, bracket_count -1);
            }
        }
        _ => {
            return loop_bypass( input,  cur_pos  + 1, bracket_count);
        }
    }
}

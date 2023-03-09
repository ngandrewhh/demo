use core::panic;
use std::cell::RefCell;
use std::rc::Rc;
use std::{fs, cmp};
use std::io::{self, *};
use std::any::type_name;
use std::collections::{HashSet, HashMap};

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn q1() {
    let file_name = "inp_q1.txt";
    let file_content = fs::read_to_string(file_name).unwrap();

    let vec = file_content.split("\n\n").collect::<Vec<&str>>();
    let vec_copy = vec.clone();
    let vec_parsed = vec_copy.iter().map(|x| x);

    let mut vec_sum: Vec<i32> = vec_parsed.map(|i| i.split("\n").map(|x| x.parse().unwrap_or(0)).collect::<Vec<i32>>().iter().sum::<i32>()).collect();

    // For extracting max index
    // let max_calorie_index = vec_sum
    //                     .iter()
    //                     .zip(0u32..)
    //                     .filter(|&(x, i)| *x == *vec_sum.iter().max().unwrap())
    //                     .map(|(_, i)| i)
    //                     .collect::<Vec<u32>>()[0];

    // For extracting max calories
    // let max_calorie = vec_sum
    //                     .iter()
    //                     .filter(|&&x| x == *vec_sum.iter().max().unwrap())
    //                     .collect::<Vec<&i32>>()[0];

    // probably this is easier.
    vec_sum.sort();
    vec_sum.reverse();
    let max_calorie: i32 = vec_sum[0..3].iter().sum();

    println!("q1a: {}", vec_sum[0]);
    println!("q1b: {}", max_calorie);
}

fn q2a_helper(a: &str, b: &str) -> i32 {
    match (a, b) {
        ("A", "X") => 4,
        ("A", "Y") => 8,
        ("A", "Z") => 3,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 7,
        ("C", "Y") => 2,
        ("C", "Z") => 6,
        _ => 0
    }
}

fn q2b_helper(a: &str, b: &str) -> i32 {
    match (a, b) {
        ("A", "Y") => 4,
        ("A", "X") => 3,
        ("A", "Z") => 8,
        ("B", "Y") => 5,
        ("B", "X") => 1,
        ("B", "Z") => 9,
        ("C", "Y") => 6,
        ("C", "X") => 2,
        ("C", "Z") => 7,
        _ => 0
    }
}

pub fn q2() {
    let file_name = "inp_q2.txt";
    let file_content = fs::read_to_string(file_name).unwrap();

    let vec = file_content.split("\n").collect::<Vec<&str>>();
    let vec_copy = vec.clone();

    let mut score_a = 0;
    let mut score_b = 0;
    for s in vec_copy {
        let mut it = s.split(" ");
        let a = it.next().unwrap();
        let b = it.next().unwrap();

        // println!("{a} vs {b}");
        score_a += q2a_helper(a, b);
        score_b += q2b_helper(a, b);
    }

    println!("q2a: {score_a}");
    println!("q2b: {score_b}");
}

fn share_char(a: &str, b: &str) -> String {
    // get which one is shorter
    let (shorter, longer) = if a.len() > b.len() {
        (b, a)
    }  else {
        (a, b)
    };

    // fill the set with the characters from the shorter string
    let set: HashSet<char> = shorter.chars().collect();

    longer.chars().filter(|c| set.contains(&c)).collect::<String>()
}

pub fn q3a() {
    let file_name = "inp_q3.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec_str = lines.map(|x| x.unwrap()).collect::<Vec<String>>();

    // let it = vec_str.chunks(2).map(|x| x.to_vec()).collect::<Vec<_>>();
    let it = vec_str.iter().map(|x| x.split_at(x.len() / 2)).collect::<Vec<_>>();

    let mut acc = 0u32;
    for arr in it.iter() {
        let a = arr.0;
        let b = arr.1;
        let c = share_char(&a, &b).chars().next().unwrap();
        let priority = c.to_digit(36).unwrap() - 9 + ((c.is_uppercase() as u32) * 26);
        
        // println!("priority: {c}={priority}");
        acc += priority;
        // println!("{}", ); //.unwrap() - 26);
        // break;
    }
        
    println!("q3a: {acc}");
}

pub fn q3b() {
    let file_name = "inp_q3.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec_str = lines.map(|x| x.unwrap()).collect::<Vec<String>>();

    let it = vec_str.chunks(3).map(|x| x.to_vec()).collect::<Vec<_>>();
    // let it = vec_str.iter().map(|x| x.split_at(x.len() / 2)).collect::<Vec<_>>();

    let mut acc = 0u32;
    for arr in it.into_iter() {

        let a = &arr[0];
        let b = &arr[1];
        let c = &arr[2];

        let shared_a_b = share_char(&a, &b);
        let ch = share_char(&shared_a_b, &c).chars().next().unwrap();

        let priority = ch.to_digit(36).unwrap() - 9 + ((ch.is_uppercase() as u32) * 26);
        
        acc += priority;
    }
        
    println!("q3b: {acc}");
}

pub fn q4a() {
    let file_name = "inp_q4.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec_str = lines.map(|x| x.unwrap()).collect::<Vec<String>>();

    let mut acc = 0;
    for arr in vec_str.iter() {
        let pair = arr.split(",").collect::<Vec<_>>();
        let pair_a = pair[0].split("-").map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let pair_b = pair[1].split("-").map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        // println!("{:?}{:?}", pair_a, pair_b);
        if ((pair_a[0] >= pair_b[0]) && (pair_a[1] <= pair_b[1])) || ((pair_b[0] >= pair_a[0]) && (pair_b[1] <= pair_a[1])) {
            acc += 1;
        }
    }

    println!("q4a: {acc}");
}

pub fn q4b() {
    let file_name = "inp_q4.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec_str = lines.map(|x| x.unwrap()).collect::<Vec<String>>();

    let mut acc = 0;
    for arr in vec_str.iter() {
        let pair = arr.split(",").collect::<Vec<_>>();
        let mut pair_a = pair[0].split("-").map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut pair_b = pair[1].split("-").map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if pair_a[0] > pair_b[0] {
            let tmp = pair_b;
            pair_b = pair_a;
            pair_a = tmp;
        }
        
        if !(pair_a[1] < pair_b[0]) {
            // println!("{:?}{:?} T", pair_a, pair_b);
            acc += 1;
        } else {
            // println!("{:?}{:?} F", pair_a, pair_b);
        }
    }

    println!("q4b: {acc}");
}

pub fn q5a() {
    let file_name = "inp_q5.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 

    // read the original stacks
    let mut stacks_ori = lines.map(|x| x.unwrap()).collect::<Vec<String>>();
    let mut instructions = stacks_ori.split_off(stacks_ori.iter().position(|x| x == "").unwrap());
    stacks_ori.reverse();
    instructions.reverse();
    instructions.pop();
    instructions.reverse();

    // prepare the stacked goods with Vec as stacks
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..10 {
        let v: Vec<char> = Vec::new();
        stacks.push(v);
    }

    let mut it = stacks_ori.iter();
    it.next();

    for line in it {
        // println!("{:?}", line);
        let v_ch: Vec<char> = line.chars().collect();
        let it_ch = v_ch.chunks(4);

        for (ix, ch) in it_ch.enumerate() {
            if ch[1] != ' ' {
                stacks[ix + 1].push(ch[1]);
            }
        }
    }

    for ins in instructions.iter() {
        // println!("{ins}");
        let v_ins: Vec<String> = ins.split(" ").map(String::from).collect();
        let mut it_ins = v_ins.iter();
        
        it_ins.next();
        let amount: i32 = it_ins.next().unwrap().parse().unwrap();

        it_ins.next();
        let from: i32 = it_ins.next().unwrap().parse().unwrap();

        it_ins.next();
        let to: i32 = it_ins.next().unwrap().parse().unwrap();

        for _ in 0..amount {
            let item = stacks[from as usize].pop().unwrap();
            stacks[to as usize].push(item);
        }
        // println!("{:?}", stacks);
    }

    let top = stacks.iter()
                            .map(|x| x.last().unwrap_or(&' '))
                            .filter(|&&x| x != ' ')
                            .collect::<String>();
    println!("q5a: {:?}", top);
}

pub fn q5b() {
    let file_name = "inp_q5.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 

    // read the original stacks
    let mut stacks_ori = lines.map(|x| x.unwrap()).collect::<Vec<String>>();
    let mut instructions = stacks_ori.split_off(stacks_ori.iter().position(|x| x == "").unwrap());
    stacks_ori.reverse();
    instructions.reverse();
    instructions.pop();
    instructions.reverse();

    // prepare the stacked goods with Vec as stacks
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..10 {
        let v: Vec<char> = Vec::new();
        stacks.push(v);
    }

    let mut it = stacks_ori.iter();
    it.next();

    for line in it {
        // println!("{:?}", line);
        let v_ch: Vec<char> = line.chars().collect();
        let it_ch = v_ch.chunks(4);

        for (ix, ch) in it_ch.enumerate() {
            if ch[1] != ' ' {
                stacks[ix + 1].push(ch[1]);
            }
        }
    }

    for ins in instructions.iter() {
        // println!("{ins}");
        let v_ins: Vec<String> = ins.split(" ").map(String::from).collect();
        let mut it_ins = v_ins.iter();
        
        it_ins.next();
        let amount: i32 = it_ins.next().unwrap().parse().unwrap();

        it_ins.next();
        let from: i32 = it_ins.next().unwrap().parse().unwrap();

        it_ins.next();
        let to: i32 = it_ins.next().unwrap().parse().unwrap();

        let mut tmp_stack: Vec<char> = Vec::new();

        for _ in 0..amount {
            let item = stacks[from as usize].pop().unwrap();
            // stacks[to as usize].push(item);
            tmp_stack.push(item);
        }

        tmp_stack.reverse();
        stacks[to as usize].extend(tmp_stack);
    }

    let top = stacks.iter()
                            .map(|x| x.last().unwrap_or(&' '))
                            .filter(|&&x| x != ' ')
                            .collect::<String>();
    println!("q5a: {:?}", top);
}

fn q6_helper(distinct_ch: usize) -> usize {
    let file_name = "inp_q6.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 
    
    let line = vec.iter().next().unwrap();
    // println!("{:?}", line);

    // memory
    let mut met: HashMap<char, usize> = HashMap::new();
    let mut passwd: String = String::new();
    let mut incum: usize = 0;

    for (ix, ch) in line.chars().enumerate() {
        // println!("pre : ix={ix}, ch={ch}, incum={incum}, passwd={passwd}, met={met:?}");

        let op_pos = met.get(&ch);

        if op_pos.is_some() {
            let pos = op_pos.unwrap();
            incum = cmp::max((*pos).checked_add(1).unwrap(), incum);
            let tmp_str = &line[incum..=ix];
            // println!("assigning {incum}: {ch} to incum: {tmp_str}");
        }

        // println!("post: ix={ix}, ch={ch}, incum={incum}, passwd={passwd}, met={met:?}");

        if ix.checked_sub(incum).unwrap() == (distinct_ch - 1) {
            passwd = line[incum..=ix].to_string();
            break;
        }

        met.insert(ch, ix);

        // if ix > 100 {
        //     break;
        // }
    }

    incum + distinct_ch
}

pub fn q6a() {
    let ans = q6_helper(4);
    println!("q6a: pos={:?}", ans);
}

pub fn q6b() {
    let ans = q6_helper(14);
    println!("q6a: pos={:?}", ans);
}

pub fn q7() {
    fn get_path(vec_path: &Vec<String>) -> String {
        "/".to_string() + &vec_path.join("/")
    }

    let file_name = "inp_q7.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 

    // let cmd = vec.clone().into_iter().filter(|x| x.starts_with("$")).collect::<Vec<String>>();
    let mut mem: HashMap<String, i32> = HashMap::new();
    let mut stack: Vec<String> = Vec::new();
    let mut it = vec.iter();

    for (ix, line) in it.enumerate() {
        let path = get_path(&stack);
        // println!("line={line:?}, path={path:?}, mem={mem:?}");

        match line {
            line if line.starts_with("$ cd ") => {
                let args = line.split(" ").collect::<Vec<_>>();
                if args[2] == ".." {
                    stack.pop();
                } else {
                    stack.push(args[2].to_string());
                };
            },
            line if line.starts_with("$ ls") => {},
            line if line.starts_with("dir ") => {},
            _ => {
                let args = line.split(" ").collect::<Vec<_>>();
                // println!(".");

                for i in 0..stack.len() {
                    let path = get_path(&stack[0..=i].to_vec());
                    // println!("[add] path={path}, args={args:?}");
                    let fsize = mem.get(&path).unwrap_or(&0);
                    mem.insert(path, fsize + args[0].parse::<i32>().unwrap());
                }

            }
        }
    }

    let mut acc = 0;
    for (_, value) in mem.iter() {
        if value <= &100_000 {
            acc += value;
        }
    }

    println!("q7a: {:?}", acc);

    let total_occupied_size = mem.get("//").unwrap();
    let required_delete_size = 30000000 - (70000000 - total_occupied_size);
    let mut mem_sub = mem.iter().filter(|&(_, v)| v > &required_delete_size).collect::<Vec<_>>();
    mem_sub.sort_by_key(|&(_, v)| v);

    let fs_min_size = mem_sub[0].1;
    println!("q7b: {fs_min_size:?}");
}

pub fn q8a() {
    let file_name = "inp_q8.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 

    // read data
    let mut arr: Vec<Vec<i32>> = Vec::new();

    for line in vec {
        let arr_sub: Vec<_> = line.chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect();
        arr.push(arr_sub); 
    }

    // println!("{:?}", arr);
    let mut ix = 0;
    let mut acc = 0;
    'outer: for r in 1..arr.len()-1 {
        'inner: for c in 1..arr[r].len()-1 {
            // println!("\nix #{ix} height {:?} at r[{r}]c[{c}]", arr[r][c]);
            ix += 1;
            let (mut block_left, mut block_up, mut block_right, mut block_down) = (false, false, false, false);

            // left clear
            for ll in (0..c).rev() {
                // println!("cmp left: {:?} vs (anchor) {:?}", arr[r][ll], arr[r][c]);
                if arr[r][ll] >= arr[r][c] {
                    // println!("break left!");
                    block_left = true;
                    break;
                }
            }

            // up clear
            for uu in (0..r).rev() {
                // println!("cmp up: {:?} vs (anchor) {:?}", arr[uu][c], arr[r][c]);
                if arr[uu][c] >= arr[r][c] {
                    // println!("break up!");
                    block_up = true;
                    break;
                }
            }

            // right
            for rr in c+1..arr[r].len() {
                // println!("cmp right: {:?} vs (anchor) {:?}", arr[r][rr], arr[r][c]);
                if arr[r][rr] >= arr[r][c] {
                    // println!("break right!");
                    block_right = true;
                    break;
                }
            }

            // down
            for dd in r+1..arr.len() {
                // println!("cmp down: {:?} vs (anchor) {:?}", arr[dd][c], arr[r][c]);
                if arr[dd][c] >= arr[r][c] {
                    // println!("break down!");
                    block_down = true;
                    break;
                }
            }

            if !(block_left && block_up && block_right && block_down) {
                // println!("add 1!");
                acc += 1;
            }

            // if ix > 100 {
            //     println!("q8: {:?}", acc + 4 * (arr.len()-1));
            //     return;
            // }
        }
    }

    println!("q8a: {:?}", acc + 4 * (arr.len()-1));

}

pub fn q8b() {
    let file_name = "inp_q8.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 

    // read data
    let mut arr: Vec<Vec<i32>> = Vec::new();

    for line in vec {
        let arr_sub: Vec<_> = line.chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect();
        arr.push(arr_sub); 
    }

    // println!("{:?}", arr);
    let mut ix = 0;
    let mut acc = 0;
    for r in 1..arr.len()-1 {
        for c in 1..arr[r].len()-1 {
            // println!("\nix #{ix} height {:?} at r[{r}]c[{c}]", arr[r][c]);
            ix += 1;
            let (mut left, mut up, mut right, mut down) = (0, 0, 0, 0);

            // left clear
            for ll in (0..c).rev() {
                // println!("cmp left: {:?} vs (anchor) {:?}", arr[r][ll], arr[r][c]);
                left += 1;
                if arr[r][ll] >= arr[r][c] {
                    break;
                }
            }

            // up clear
            for uu in (0..r).rev() {
                // println!("cmp up: {:?} vs (anchor) {:?}", arr[uu][c], arr[r][c]);
                up += 1;
                if arr[uu][c] >= arr[r][c] {
                    break;
                }
            }

            // right
            for rr in c+1..arr[r].len() {
                // println!("cmp right: {:?} vs (anchor) {:?}", arr[r][rr], arr[r][c]);
                right += 1;
                if arr[r][rr] >= arr[r][c] {
                    break;
                }
            }

            // down
            for dd in r+1..arr.len() {
                // println!("cmp down: {:?} vs (anchor) {:?}", arr[dd][c], arr[r][c]);
                down += 1;
                if arr[dd][c] >= arr[r][c] {
                    break;
                }
            }

            acc = cmp::max(acc, up * left * right * down);
        }
    }

    println!("q8b: {:?}", acc);
}

pub fn q9a() {
    let file_name = "inp_q9.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 

    // read data
    type Pos = (i32, i32);
    let mut h_pos: Pos = (0, 0);
    let mut t_pos: Pos = (0, 0);
    let mut visited: HashSet<Pos> = HashSet::new();

    fn l1_distance(a: Pos, b: Pos) -> Pos {
        (a.0 - b.0, a.1 - b.1)
        // let sq = (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2);
        // (sq as f32).sqrt()
    }

    fn add(a: Pos, b: Pos) -> Pos {
        (a.0 + b.0, a.1 + b.1)
    }

    for (ix, line) in vec.iter().enumerate() {
        // println!("line={line}");
        let args = line.split(" ").collect::<Vec<_>>();
        let dx = args[0];
        let steps = args[1].parse::<i32>().unwrap();

        for _ in 0..steps {
            let h_pos_new = match dx {
                "L" => Ok((h_pos.0-1, h_pos.1)),
                "R" => Ok((h_pos.0+1, h_pos.1)),
                "U" => Ok((h_pos.0, h_pos.1+1)),
                "D" => Ok((h_pos.0, h_pos.1-1)),
                _ => Err(())
            };
            h_pos = h_pos_new.unwrap();
            let dist = l1_distance(h_pos, t_pos);

            // println!("pre > h_pos={h_pos:?}, t_pos={t_pos:?}, distance={dist:?}");
            
            // static compile the corresponding moves wrt to distance
            let t_step: Pos = match dist {
                ( 2,  0) => ( 1,  0),
                (-2,  0) => (-1,  0),
                ( 0,  2) => ( 0,  1),
                ( 0, -2) => ( 0, -1),
                ( 1,  2) => ( 1,  1),
                ( 1, -2) => ( 1, -1),
                (-1,  2) => (-1,  1),
                (-1, -2) => (-1, -1),
                ( 2,  1) => ( 1,  1),
                ( 2, -1) => ( 1, -1),
                (-2,  1) => (-1,  1),
                (-2, -1) => (-1, -1),

                ( 2,  2) => ( 1,  1),
                (-2,  2) => (-1,  1),
                ( 2, -2) => ( 1, -1),
                (-2, -2) => (-1, -1),
                _ => (0, 0),
            };

            t_pos = add(t_pos, t_step);
            visited.insert(t_pos);
            // println!("post> h_pos={h_pos:?}, t_pos={t_pos:?}, t_step={t_step:?}");
        }
    }

    println!("q9a: {:?}", visited.len());
}

pub fn q9b() {
    let file_name = "inp_q9.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 

    // read data
    type Pos = (i32, i32);
    let mut v_pos: Vec<Pos> = Vec::new();
    let mut visited: HashSet<Pos> = HashSet::new();

    for i in 0..10 { v_pos.push((0, 0)); }

    fn l1_distance(a: Pos, b: Pos) -> Pos {
        (a.0 - b.0, a.1 - b.1)
        // let sq = (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2);
        // (sq as f32).sqrt()
    }

    fn add(a: Pos, b: Pos) -> Pos {
        (a.0 + b.0, a.1 + b.1)
    }

    for (ix, line) in vec.iter().enumerate() {
        // println!("line={line}");
        let args = line.split(" ").collect::<Vec<_>>();
        let dx = args[0];
        let steps = args[1].parse::<i32>().unwrap();

        for _ in 0..steps {
            let mut pos = v_pos[0];
            let pos_new = match dx {
                "L" => Ok((pos.0-1, pos.1)),
                "R" => Ok((pos.0+1, pos.1)),
                "U" => Ok((pos.0, pos.1+1)),
                "D" => Ok((pos.0, pos.1-1)),
                _ => Err(())
            };
            pos = pos_new.unwrap();
            v_pos[0] = pos;

            for knot in 1..v_pos.len() {
                let mut pos = v_pos[knot];
                let dist = l1_distance(v_pos[knot-1], v_pos[knot]);

                // println!("pre > pos={v_pos:?}");
                // static compile the corresponding moves wrt to distance
                let t_step: Pos = match dist {
                    ( 2,  0) => ( 1,  0),
                    (-2,  0) => (-1,  0),
                    ( 0,  2) => ( 0,  1),
                    ( 0, -2) => ( 0, -1),
                    ( 1,  2) => ( 1,  1),
                    ( 1, -2) => ( 1, -1),
                    (-1,  2) => (-1,  1),
                    (-1, -2) => (-1, -1),
                    ( 2,  1) => ( 1,  1),
                    ( 2, -1) => ( 1, -1),
                    (-2,  1) => (-1,  1),
                    (-2, -1) => (-1, -1),

                    ( 2,  2) => ( 1,  1),
                    (-2,  2) => (-1,  1),
                    ( 2, -2) => ( 1, -1),
                    (-2, -2) => (-1, -1),
                    _ => (0, 0),
                };

                let knot_pos = add(v_pos[knot], t_step);
                v_pos[knot] = knot_pos;
            }

            visited.insert(v_pos[9]);
            // println!("post> pos={v_pos:?}\n");
        }

        // if ix > 5 { break; }
    }

    println!("q9b: {:?}", visited.len());
}

pub fn q10a() {
    fn addx(acc: &mut i32, x: i32) {
        *acc += x;
    }

    let file_name = "inp_q10.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 

    let mut acc: i32 = 1;
    let mut sig: i32 = 0;
    let mut x: i32 = 0;
    let mut cycle: i32 = 1;
    let mut cmd_remaining_cycle: i32 = 0;

    let mut it = vec.iter();
    let mut line_opt = it.next();
    let mut line: &String = &String::new();
    let mut args: Vec<&str> = Vec::new();

    while line_opt.is_some() {
        if cmd_remaining_cycle == 0 {
            line = line_opt.unwrap();
            args = line.split(" ").collect::<Vec<_>>();

            match args.len() {
                1 => {
                    x = 0;
                    cmd_remaining_cycle = 1;
                },
                2 => {
                    x = args[1].parse().unwrap();
                    cmd_remaining_cycle = 2;
                }
                _ => panic!()
            }
        }

        cycle += 1;
        cmd_remaining_cycle -= 1;

        if cmd_remaining_cycle == 0 {
            addx(&mut acc, x);
            line_opt = it.next();
        }

        // println!("cycle={cycle}, acc={acc}, x={x}, line={line}, rc={cmd_remaining_cycle}");
        // if cycle > 30 { break; }

        if cycle.rem_euclid(40) == 20 {
            // println!("adding {cycle}*{acc}={:?} to sig", cycle * acc);
            sig += cycle * acc;
        }
    }
    println!("q10a: {:?}", sig);
}

pub fn q10b() {
    fn addx(acc: &mut i32, x: i32) {
        *acc += x;
    }

    let file_name = "inp_q10.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 

    let mut acc: i32 = 1;
    let mut sig: i32 = 0;
    let mut x: i32 = 0;
    let mut cycle: i32 = 0;
    let mut cmd_remaining_cycle: i32 = 0;

    let mut it = vec.iter();
    let mut line_opt = it.next();
    let mut line: &String = &String::new();
    let mut args: Vec<&str> = Vec::new();
    let mut disp: String = String::new();

    while line_opt.is_some() {
        
        cycle += 1;
        // println!("\nbefore.. cycle={cycle}, acc={acc}, x={x}, disp=\n{disp}");

        if cmd_remaining_cycle == 0 {
            line = line_opt.unwrap();
            args = line.split(" ").collect::<Vec<_>>();

            match args.len() {
                1 => {
                    x = 0;
                    cmd_remaining_cycle = 1;
                },
                2 => {
                    x = args[1].parse().unwrap();
                    cmd_remaining_cycle = 2;
                }
                _ => panic!()
            }
        }

        let rem = (cycle - 1).rem_euclid(40);
        
        if acc+1 >= rem && rem >= acc-1 {
            disp.push('#');
        } else {
            disp.push('.');
        }

        if cycle.rem_euclid(40) == 0 {
            disp.push('\n');
        }

        cmd_remaining_cycle -= 1;

        if cmd_remaining_cycle == 0 {
            addx(&mut acc, x);
            line_opt = it.next();
        }

        // println!("after .. cycle={cycle}, acc={acc}, x={x}, disp=\n{disp}");
    }
    println!("q10b: \n{:}", disp);
}

pub fn q11_helper(worry_divisor: i64, rounds: i32) -> i64 {
    #[derive(Debug)]
    enum MonkeyOp {
        Add,
        Mul,
        AddSelf,
        MulSelf,
        Err
    }

    #[derive(Debug)]
    struct Monkey {
        id: i64,
        items: Vec<i64>,
        divisible: i64,
        branch: (i32, i32),
        op: (MonkeyOp, i64),
        inspect_count: i64,
        worry_divisor: i64,
    }

    impl Monkey {
        fn build(id: i64, items: Vec<i64>, op: (MonkeyOp, i64), divisible: i64, branch_true: i32, branch_false: i32, worry_divisor: i64) -> Self {
            Monkey { 
                id: id, 
                items: items, 
                op: op, 
                divisible: divisible, 
                branch: (branch_false, branch_true), 
                inspect_count: 0, 
                worry_divisor: worry_divisor 
            }
        }

        fn inspect(self: &mut Monkey) -> (i64, i32) {
            let item = self.items.pop().unwrap();
            self.inspect_count += 1;
            let item_new: i64 = match self.op.0 {
                MonkeyOp::Mul => item * self.op.1,
                MonkeyOp::Add => item + self.op.1,
                MonkeyOp::MulSelf => item * item,
                MonkeyOp::AddSelf => item + item,
                _ => 0
            };
            let item_new = item_new / self.worry_divisor;
            if item_new.rem_euclid(self.divisible) == 0 {
                (item_new, self.branch.1)
            } else {
                (item_new, self.branch.0)
            }
        }
    }

    let file_name = "inp_q11.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut it = vec.iter(); 

    loop {
        let id = it.next().unwrap().strip_prefix("Monkey ").unwrap().strip_suffix(":").unwrap().parse::<_>().unwrap();
        let mut items = it.next().unwrap().trim().strip_prefix("Starting items: ").unwrap().split(", ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let op: Vec<&str> = it.next().unwrap().trim().strip_prefix("Operation: new = ").unwrap().split(" ").collect();
        
        let monkey_op = match op[..] {
            ["old", "*", "old"] => (MonkeyOp::MulSelf, 0),
            ["old", "+", "old"] => (MonkeyOp::AddSelf, 0),
            ["old", "*", _] => (MonkeyOp::Mul, op[2].parse().unwrap()),
            ["old", "+", _] => (MonkeyOp::Add, op[2].parse().unwrap()),
            _ => panic!()
        };
        
        let divisible = it.next().unwrap().trim().strip_prefix("Test: divisible by ").unwrap().parse::<_>().unwrap();
        let branch_true = it.next().unwrap().trim().strip_prefix("If true: throw to monkey ").unwrap().parse::<_>().unwrap();
        let branch_false = it.next().unwrap().trim().strip_prefix("If false: throw to monkey ").unwrap().parse::<_>().unwrap();
        let mut monkey = Monkey::build(id, items, monkey_op, divisible, branch_true, branch_false, worry_divisor);
        
        // println!("{:?}", monkey);
        // let res = monkey.inspect();
        // println!("{:?}{:?}", monkey, res);
        monkeys.push(monkey);

        let breaker = it.next();
        if breaker.is_none() { break; }
    }

    let LCM: i64 = monkeys.iter().map(|x| x.divisible).product();

    for round in 0..rounds {
        
        for ix in 0..monkeys.len() {
            // println!(".. Before");
            // for (ix, monkey) in monkeys.iter().enumerate() {
            //     println!("Monkey {ix}: {:?}", monkey.items);
            // }

            while monkeys[ix].items.len() > 0 {
                let (item_new, to_monkey) = monkeys[ix].inspect();

                // uses the fact that all divisors in the branch condition is prime
                let item_new = item_new.rem_euclid(LCM);

                monkeys[to_monkey as usize].items.push(item_new);
            }

            // println!(".. After");
            // for (ix, monkey) in monkeys.iter().enumerate() {
            //     println!("Monkey {ix}: {:?}", monkey.items);
            // }
        }

        // println!("\n== After round {} ==", round+1);
        // for (ix, monkey) in monkeys.iter().enumerate() {
        //     println!("Monkey {ix} inspected items {} times.", monkey.inspect_count);
        // }
    }

    let mut inspect_counts = monkeys.iter().map(|x| x.inspect_count).collect::<Vec<_>>();

    // for (ix, ic) in inspect_counts.iter().enumerate() {
    //     println!("Monkey {ix} inspected items {} times.", ic);
    // }

    inspect_counts.sort();
    inspect_counts.reverse();
    inspect_counts[0] * inspect_counts[1]
}

pub fn q11a() {
    let ans = q11_helper(3, 20);
    println!("q11a: {}", ans);
}

pub fn q11b() {
    let ans = q11_helper(1, 10000);
    println!("q11b: {}", ans);
}

pub fn q12a() {

    let file_name = "inp_q12.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 

    let mut cmap: Vec<Vec<char>> = Vec::new();
    let mut hmap: Vec<Vec<i32>> = Vec::new();
    let mut smap: Vec<Vec<i32>> = Vec::new();

    let vis: HashSet<(usize, usize)> = HashSet::new();
    let path: Vec<(usize, usize)> = Vec::new();   
    let mut start: (usize, usize, i32, HashSet<_>, Vec<_>) = (0, 0, 0, vis, path);
    let mut goal: (usize, usize) = (0, 0);

    for (rx, line) in vec.iter().enumerate() {
        let mut cmap_line: Vec<char> = Vec::new();
        let mut hmap_line: Vec<i32> = Vec::new();
        let mut smap_line: Vec<i32> = Vec::new();

        for (cx, ch) in line.chars().enumerate() {
            // println!("{ch}");
            let h = match ch {
                'S' => {
                    start.0 = rx;
                    start.1 = cx;
                    0
                }
                'E' => {
                    goal.0 = rx;
                    goal.1 = cx;
                    26
                }
                'a' => {
                    ch.to_digit(36).unwrap() as i32 - 9
                }
                _ => ch.to_digit(36).unwrap() as i32 - 9,
            };

            cmap_line.push(ch);
            hmap_line.push(h);
            smap_line.push(99999999);
        }

        cmap.push(cmap_line);
        hmap.push(hmap_line);
        smap.push(smap_line);
    }

    let mut frontier = Vec::<(usize, usize, i32, HashSet<(usize, usize)>, Vec<(usize, usize)>)>::new();
    let mut min_steps = 99999999;
    frontier.push(start);

    while frontier.len() > 0 {
        let pos = frontier.pop().unwrap();
        let mut visited = pos.3;
        let mut path = pos.4;
        visited.insert((pos.0, pos.1));
        path.push((pos.0, pos.1));

        // min map - if any other path arrives at this point with fewer step,
        // then no need to expand the given path.
        if pos.2 >= smap[pos.0][pos.1] {
            continue;
        } else {
            smap[pos.0][pos.1] = pos.2;
        }

        if pos.0 == goal.0 && pos.1 == goal.1 {
            min_steps = cmp::min(min_steps, pos.2);
            if pos.2 == 472 {
               println!("GOAL! Steps: {}", pos.2);
               for &(r, c) in path.iter() {
                    cmap[r][c] = '.';
                    // println!("ch={} pos=({}, {})", cmap[r][c], r, c);
               }

               let final_vec = cmap.iter().map(|x| x.iter().collect::<String>());
               for line in final_vec {
                    println!("{}", line);
               }

            }
        }
        
        if pos.0 > 0 {
            // println!("UP");
            if !visited.contains(&(pos.0 - 1, pos.1)) {
                if hmap[pos.0 - 1][pos.1] - hmap[pos.0][pos.1] <= 1 {
                    frontier.push((pos.0 - 1, pos.1, pos.2 + 1, visited.clone(), path.clone()))
                }
            }
        }
        
        if pos.0 < hmap.len() - 1 {
            // println!("DOWN");
            if !visited.contains(&(pos.0 + 1, pos.1)) {
                if hmap[pos.0 + 1][pos.1] - hmap[pos.0][pos.1] <= 1 {
                    frontier.push((pos.0 + 1, pos.1, pos.2 + 1, visited.clone(), path.clone()))
                }
            }
        }

        if pos.1 > 0 {
            // println!("LEFT");
            if !visited.contains(&(pos.0, pos.1 - 1)) {
                if hmap[pos.0][pos.1 - 1] - hmap[pos.0][pos.1] <= 1 {
                    frontier.push((pos.0, pos.1 - 1, pos.2 + 1, visited.clone(), path.clone()))
                }
            }
        }

        if pos.1 < hmap[0].len() - 1 {
            // println!("RIGHT");
            if !visited.contains(&(pos.0, pos.1 + 1)) {
                if hmap[pos.0][pos.1 + 1] - hmap[pos.0][pos.1] <= 1 {
                    frontier.push((pos.0, pos.1 + 1, pos.2 + 1, visited.clone(), path.clone()))
                }
            }
        }

        // println!("pos=({}, {}), frontier={frontier:?}", pos.0, pos.1);
        // println!("pos=({}, {})", pos.0, pos.1);
    }

    println!("q12a: {min_steps:?}",);

    // for pos_a in is_a.iter() {
    //     println!("steps to a: {}", smap[pos_a.0][pos_a.1]);
    // }   
}

pub fn q12b() {

    let file_name = "inp_q12.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 

    let mut cmap: Vec<Vec<char>> = Vec::new();
    let mut hmap: Vec<Vec<i32>> = Vec::new();
    let mut smap: Vec<Vec<i32>> = Vec::new();

    let mut is_a: HashSet<(usize, usize)> = HashSet::new();
    let vis: HashSet<(usize, usize)> = HashSet::new();
    let path: Vec<(usize, usize)> = Vec::new();   
    let mut start: (usize, usize, i32, HashSet<_>, Vec<_>) = (0, 0, 0, vis, path);
    let mut goal: (usize, usize) = (0, 0);

    for (rx, line) in vec.iter().enumerate() {
        let mut cmap_line: Vec<char> = Vec::new();
        let mut hmap_line: Vec<i32> = Vec::new();
        let mut smap_line: Vec<i32> = Vec::new();

        for (cx, ch) in line.chars().enumerate() {
            // println!("{ch}");
            let h = match ch {
                'E' => {
                    start.0 = rx;
                    start.1 = cx;
                    26
                }
                'S' | 'a' => {
                    is_a.insert((rx, cx));
                    1
                }
                _ => ch.to_digit(36).unwrap() as i32 - 9,
            };

            cmap_line.push(ch);
            hmap_line.push(h);
            smap_line.push(99999999);
        }

        cmap.push(cmap_line);
        hmap.push(hmap_line);
        smap.push(smap_line);
    }

    let mut frontier = Vec::<(usize, usize, i32, HashSet<(usize, usize)>, Vec<(usize, usize)>)>::new();
    let mut min_steps = 99999999;
    frontier.push(start);
    goal = (20, 0);

    while frontier.len() > 0 {
        let pos = frontier.pop().unwrap();
        let mut visited = pos.3;
        let mut path = pos.4;
        visited.insert((pos.0, pos.1));
        path.push((pos.0, pos.1));

        // min map - if any other path arrives at this point with fewer step,
        // then no need to expand the given path.
        if pos.2 >= smap[pos.0][pos.1] {
            continue;
        } else {
            smap[pos.0][pos.1] = pos.2;
        }

        if is_a.contains(&(pos.0, pos.1)) {
            min_steps = cmp::min(min_steps, pos.2);
            // println!("GOAL! Steps: {}", pos.2);
        }
        
        if pos.0 > 0 {
            if !visited.contains(&(pos.0 - 1, pos.1)) {
                if hmap[pos.0 - 1][pos.1] - hmap[pos.0][pos.1] >= -1 {
                    frontier.push((pos.0 - 1, pos.1, pos.2 + 1, visited.clone(), path.clone()))
                }
            }
        }
        
        if pos.0 < hmap.len() - 1 {
            if !visited.contains(&(pos.0 + 1, pos.1)) {
                if hmap[pos.0 + 1][pos.1] - hmap[pos.0][pos.1] >= -1 {
                    frontier.push((pos.0 + 1, pos.1, pos.2 + 1, visited.clone(), path.clone()))
                }
            }
        }

        if pos.1 > 0 {
            if !visited.contains(&(pos.0, pos.1 - 1)) {
                if hmap[pos.0][pos.1 - 1] - hmap[pos.0][pos.1] >= -1 {
                    frontier.push((pos.0, pos.1 - 1, pos.2 + 1, visited.clone(), path.clone()))
                }
            }
        }

        if pos.1 < hmap[0].len() - 1 {
            if !visited.contains(&(pos.0, pos.1 + 1)) {
                if hmap[pos.0][pos.1 + 1] - hmap[pos.0][pos.1] >= -1 {
                    frontier.push((pos.0, pos.1 + 1, pos.2 + 1, visited.clone(), path.clone()))
                }
            }
        }

        // println!("pos=({}, {}), frontier={frontier:?}", pos.0, pos.1);
        // println!("pos=({}, {})", pos.0, pos.1);
    }

    println!("q12b: {min_steps:?}");
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
enum Thing {
    Vec(Vec<Thing>),
    Number(i32),
}

fn parse_signal(signal: String) -> Vec<Thing> {
    let mut head: Vec<Thing> = Vec::new();
    let mut stack: Vec<Box<Vec<Thing>>> = Vec::new();
    let mut ptr: Box<Vec<Thing>> =  Box::new(head);
    let mut curr: Vec<Thing> = Vec::new();
    let mut mem: String = String::new();

    for ch in signal.chars() {
        match ch {
            '[' => {
                stack.push(ptr);
                curr = Vec::new();
                ptr = Box::new(curr);
            },
            ']' => {
                if mem.len() != 0 {
                    let num = Thing::Number(mem.parse::<i32>().unwrap());
                    (*ptr).push(num);
                    mem = String::new();
                }

                let tmp = ptr;
                ptr = stack.pop().unwrap();
                (*ptr).push(Thing::Vec(*tmp));
            },
            ',' => {
                if mem.len() != 0 {
                    let num = Thing::Number(mem.parse::<i32>().unwrap());
                    (*ptr).push(num);
                    mem = String::new();
                }
            },
            _ => {
                mem.push(ch);
            }
        }
        // println!("stack={:?}, ptr={:?}, mem={:?}", stack, ptr, mem);
    }

    *ptr
}

fn cmp_results_ori(vec_left: &Vec<Thing>, vec_right: &Vec<Thing>) -> bool {
    // This is malformed as the base case is hard to define given it's always wrapped in a Vec.
    // println!("\ncalling cmp_results with: \nleft={:?}, \nright={:?}", vec_left, vec_right);

    let mut it_left = vec_left.iter();
    let mut it_right = vec_right.iter();
    let mut flag = true;

    loop {
        let (ol, or) = (it_left.next(), it_right.next());
        
        match (ol, or) {
            (None, None) | (None, Some(_)) => { break; }
            (Some(_), None) => { return false; },
            _ => {}
        };

        let (l, r) = (ol.unwrap(), or.unwrap());

        match (l, r) {
            (Thing::Number(i), Thing::Number(j)) => { 
                if i < j {
                    break;
                }
                if i > j {
                    flag &= false;
                    break;
                }
            },

            (Thing::Vec(i), Thing::Vec(j)) => { 
                if i.len() == 0 && j.len() == 0 {
                    continue;
                }
                return cmp_results_ori(i, j);
            },

            (Thing::Number(i), Thing::Vec(j)) => { 
                let v = vec![Thing::Number(*i)];
                return cmp_results_ori(&v, j);
            },

            (Thing::Vec(i), Thing::Number(j)) => { 
                let v = vec![Thing::Number(*j)];
                return cmp_results_ori(i, &v);
            },
        }
    }

    return flag;
}

fn cmp_results(l: &Thing, r: &Thing) -> i32 {
    // println!("\ncalling cmp_results with: \nleft={:?}, \nright={:?}", vec_left, vec_right);

    let mut flag = true;

    match (l, r) {
        (Thing::Number(i), Thing::Number(j)) => { 
            // println!("> Case: Num Num");
            return i - j
        },

        (Thing::Vec(i), Thing::Vec(j)) => { 
            // println!("> Case: Vec Vec");
            let it = i.iter().zip(j);
            for (x, y) in it {
                let cmp_res = cmp_results(x, y);
                if cmp_res != 0 { 
                    return cmp_res 
                };
            }
            return i.len() as i32 - j.len() as i32;
        },

        (Thing::Number(i), Thing::Vec(j)) => { 
            // println!("> Case: Num Vec");
            let v = Thing::Vec(vec![Thing::Number(*i)]);
            return cmp_results(&v, r);
        },

        (Thing::Vec(i), Thing::Number(j)) => { 
            // println!("> Case: Vec Num");
            let v = Thing::Vec(vec![Thing::Number(*j)]);
            return cmp_results(l, &v);
        },
    }
}

pub fn q13a() {

    let file_name = "inp_q13.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 

    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut it = vec.iter();
    
    loop {
        let left = it.next().unwrap();
        let right = it.next().unwrap();
        pairs.push((left.to_string(), right.to_string()));

        let br = it.next();

        if br.is_none() {
            break;
        }
    }

    let mut vec_ans: Vec<usize> = Vec::new();

    for (ix, (left, right)) in pairs.iter().enumerate() {
        // println!("\npair #{}", ix+1);
        // println!("left={left:?}");
        // println!("right={right:?}");
        let parsed_left = &parse_signal(left.to_string())[0];
        let parsed_right = &parse_signal(right.to_string())[0];

        match cmp_results(&parsed_left, &parsed_right) {
            i if i < 0 => { 
                // println!("true"); 
                vec_ans.push(ix+1); 
            }
            _ => { 
                // println!("false"); 
            }
        }
    }

    println!("q13a: {}", vec_ans.iter().sum::<usize>());
}


pub fn q13b() {
    // this is too long.
    let file_name = "inp_q13.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 

    let mut it = vec.iter();
    let mut merge: Vec<Vec<Thing>> = Vec::new();
    
    // data preparation and add encoder/decoder
    loop {
        let left = it.next().unwrap();
        let right = it.next().unwrap();
        let parsed_left = parse_signal(left.to_string());
        let parsed_right = parse_signal(right.to_string());
        let thing_left = &parsed_left[0];
        let thing_right = &parsed_right[0];

        match cmp_results(&thing_left, &thing_right) {
            i if i < 0 => {
                let mut v = Vec::new();
                v.push(thing_left.clone());
                v.push(thing_right.clone());
                merge.push(v);
            },
            i if i > 0 => {
                let mut v = Vec::new();
                v.push(thing_right.clone());
                v.push(thing_left.clone());
                merge.push(v);
            },
            _ => { panic!(); }
        }

        let br = it.next();

        if br.is_none() {
            break;
        }
    }

    let two = parse_signal("[[2]]".to_string());
    let six = parse_signal("[[6]]".to_string());
    let thing_two = &two[0];
    let thing_six = &six[0];
    let mut v = Vec::new();
    v.push(thing_two.clone());
    v.push(thing_six.clone());
    merge.push(v);

    // merge sort
    fn merge_sort(mut left: Vec<Thing>, mut right: Vec<Thing>) -> Vec<Thing> {
        let mut out: Vec<Thing> = Vec::new();
        left.reverse();
        right.reverse();

        loop {
            // println!(">> inner, out={out:?}");
            // println!(">> inner, left={left:?}");
            // println!(">> inner, right={right:?}\n");
            let left_ = left.pop();
            let right_ = right.pop();

            match (left_, right_) {
                (Some(l), Some(r)) => {
                    if cmp_results(&l, &r) < 0 {
                        out.push(l);
                        right.push(r);
                    } else {
                        out.push(r);
                        left.push(l);
                    }
                },
                (Some(l), None) => {
                    out.push(l);
                },
                (None, Some(r)) => {
                    out.push(r);
                },
                (None, None) => {
                    break;
                },
            }
        }

        out
    }

    let mut acc = 1;

    // doing the merge sort by looping until there's only 1 element left (i.e. all item merged).
    while merge.len() > 1 {
        let mut merge_tmp = Vec::new();

        loop {
            let left = merge.pop();
            let right = merge.pop();

            // println!("left={left:?}");
            // println!("right={right:?}");

            match (left, right) {
                (Some(l), Some(r)) => {
                    let out = merge_sort(l, r);
                    // println!(">> merge sort, out={out:?}");
                    merge_tmp.push(out);
                },
                (Some(l), None) => {
                    merge_tmp.push(l);
                },
                (None, Some(r)) => {
                    merge_tmp.push(r);
                },
                (None, None) => {
                    break;
                },
            }

            // println!(">> merge result, tmp={merge_tmp:?}\n");
        }

        merge.extend(merge_tmp);
    }   

    for (ix, line) in merge[0].iter().enumerate() {
        if line == thing_two 
        || line == thing_six
        {
            // println!("ix={}, line={line:?}", ix+1);    
            acc *= ix + 1;
        }
    }

    println!("q13b: {}", acc);
}


pub fn q13b_no_sort() {

    let file_name = "inp_q13.txt";
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 

    type Signal = Vec<Thing>;
    let mut it = vec.iter();
    let mut merge: Vec<Vec<Signal>> = Vec::new();
    
    // data preparation and add encoder/decoder
    let two = &parse_signal("[[2]]".to_string())[0];
    let six = &parse_signal("[[6]]".to_string())[0];
    let mut lt_two = 0;
    let mut lt_six = 0;

    loop {
        let br = it.next();
        if br.is_none() {
            break;
        }

        let line = br.unwrap();

        if line.len() > 0 {
            let parsed = &parse_signal(line.to_string())[0];
            if cmp_results(&parsed, &two) < 0
            {
                lt_two += 1;
            }
            if cmp_results(&parsed, &six) < 0
            {
                lt_six += 1;
            }
        }
    }
    println!("<2={lt_two}, <6={lt_six}");
    println!("q13b: {}", (lt_two + 1) * (lt_six + 2));
}
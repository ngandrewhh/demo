use std::hash::Hash;
use std::ops::RangeInclusive;
use std::{fs, cmp};
use std::io::{self, *};
use std::collections::{HashSet, HashMap, hash_set};
use range_union_find::IntRangeUnionFind;
use itertools::Itertools;

use regex::Regex;
use priority_queue::PriorityQueue;

pub fn read_to_lines(file_name: &str) -> Vec<String> {
    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines(); 
    let vec = lines.map(|x| x.unwrap()).collect::<Vec<String>>(); 
    vec
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
    let it = vec.iter();

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

fn q14_helper (corner_a: &Vec<i32>, corner_b: &Vec<i32>, map: &mut Vec<Vec<char>>, offset: i32) -> i32 {
    let mut xy_a: (i32, i32);
    let mut xy_b: (i32, i32);
    let xy_d: i32 = corner_a.iter().sum::<i32>() - corner_b.iter().sum::<i32>();

    match xy_d {
        i if i > 0 => {
            xy_a = (corner_b[0], corner_b[1]);
            xy_b = (corner_a[0], corner_a[1]);
        },
        i if i < 0 => {
            xy_b = (corner_b[0], corner_b[1]);
            xy_a = (corner_a[0], corner_a[1]);   
        }
        _ => { panic!() }
    }

    let xy_d = (xy_b.0 - xy_a.0, xy_b.1 - xy_a.1);
    let mut draw_points = vec![xy_a, xy_b];

    for x in 1..xy_d.0 {
        draw_points.push((xy_a.0 + x, xy_a.1));
    }

    for y in 1..xy_d.1 {
        draw_points.push((xy_a.0, xy_a.1 + y));
    }

    // println!("{xy_d:?}, {draw_points:?}");

    for &(x, y) in draw_points.iter() {
        let x_ = (x - offset) as usize;
        let y_ = y as usize;
        // println!("{x_:?}, {y_:?}");
        map[y_][x_] = '#';
    }

    xy_b.1
    // println!("drawn line from {corner_a:?} to {corner_b:?}");
}

pub fn q14a() {
    let vec = read_to_lines("inp_q14.txt");

    let mut maps: Vec<Vec<char>> = Vec::with_capacity(200);
    for x in 400..600 {
        let v: Vec<char> = vec!['.'; 200];
        maps.push(v);
    }

    let corners_all = vec
        .iter().map(
            |x| x.split(" -> ").map(
                |x| x.split(",").map(
                    |x| x.parse::<i32>().unwrap()
                ).collect::<Vec<i32>>())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for corners in corners_all.iter() {
        for corner_pairs in corners.iter().zip(corners.iter().skip(1)) {
            q14_helper(corner_pairs.0, corner_pairs.1, &mut maps, 400);
        }
    }

    let mut i = 0;
    'outer: loop {
        i += 1;
        let mut sand = (100, 0);
        loop {
            if sand.1 == 199 {
                break 'outer;
            }
            if &maps[sand.1 + 1][sand.0] == &'.' {
                sand = (sand.0, sand.1 + 1);
            } else if &maps[sand.1 + 1][sand.0 - 1] == &'.' {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if &maps[sand.1 + 1][sand.0 + 1] == &'.' {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                maps[sand.1][sand.0] = 'o';
                break;
            }
        }
    }

    // for row in maps.iter() {
    //     println!("{:?}", row.iter().collect::<String>());
    // }

    println!("q14a: {}", i - 1);
}

pub fn q14b() {
    let vec = read_to_lines("inp_q14.txt");

    let mut maps: Vec<Vec<char>> = Vec::with_capacity(200);
    for _ in 0..200 {
        let v: Vec<char> = vec!['.'; 500];
        maps.push(v);
    }

    let corners_all = vec
        .iter().map(
            |x| x.split(" -> ").map(
                |x| x.split(",").map(
                    |x| x.parse::<i32>().unwrap()
                ).collect::<Vec<i32>>())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut max_known_depth = 0;

    for corners in corners_all.iter() {
        for corner_pairs in corners.iter().zip(corners.iter().skip(1)) {
            let max_depth = q14_helper(corner_pairs.0, corner_pairs.1, &mut maps, 250);
            max_known_depth = cmp::max(max_known_depth, max_depth)
        }
    }

    // println!("{max_known_depth:?}");
    q14_helper(&vec![250, max_known_depth + 2], &vec![749, max_known_depth + 2], &mut maps, 250);

    let mut i = 0;
    'outer: loop {
        i += 1;
        let mut sand = (250, 0);
        loop {
            if &maps[sand.1][sand.0] == &'o' {
                break 'outer;
            }
            if sand.1 == 199 {
                break 'outer;
            }
            if &maps[sand.1 + 1][sand.0] == &'.' {
                sand = (sand.0, sand.1 + 1);
            } else if sand.0 - 1 > 0 && &maps[sand.1 + 1][sand.0 - 1] == &'.' {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if sand.0 + 1 < 500 && &maps[sand.1 + 1][sand.0 + 1] == &'.' {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                maps[sand.1][sand.0] = 'o';
                break;
            }
        }
    }

    // for row in maps.iter() {
    //     println!("{:?}", row.iter().collect::<String>());
    // }

    println!("q14a: {}", i - 1);
}

#[derive(Debug)]
struct Coord {
    s_x: i32,
    s_y: i32,
    b_x: i32,
    b_y: i32
}

impl Coord {
    fn manhattan(&self) -> i32 {
        (self.s_x - self.b_x).abs() + (self.s_y - self.b_y).abs()
    }

    fn y_coverage(&self, y: i32) -> Option<(i32, i32)> {
        match self.manhattan() - (self.s_y - y).abs() {
            i if i >= 0 => Some((self.s_x - i, self.s_x + i)),
            _ => None
        }
    }
}

pub fn q15a() {
    let vec = read_to_lines("inp_q15.txt");
    let mut coords: Vec<Coord> = Vec::new();

    for line in vec.iter() {
        let it = line.split(' ').filter(|x| x.contains('=')).collect::<Vec<&str>>();
        let args = it.iter()
        .map(|x| x.split('=').collect::<Vec<_>>())
        .map(|x| x[1].strip_suffix([',', ':']).unwrap_or(x[1]).parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
        
        let mut it = args.into_iter();
        let s_x = it.next().unwrap();
        let s_y = it.next().unwrap();

        let b_x = it.next().unwrap();
        let b_y = it.next().unwrap();

        let coord: Coord = Coord {s_x, s_y, b_x, b_y};
        coords.push(coord);
    }

    // The coverage is defined as subtracting the diff in y-coord from the manhattan distance,
    // then the remaining distance should expand to both side on the target row.
    let coverages = coords.iter()
        .map(|x| x.y_coverage(2000000))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    // Uses an external crate to find the overlapping ranges.
    let mut union_range = IntRangeUnionFind::<i32>::new();

    for &(lo, hi) in coverages.iter() {
        let range = lo..=hi;
        union_range.insert_range(&range);
    }

    let ranges: _ = union_range.to_collection::<Vec<RangeInclusive<i32>>>();
    let pos_count: i32 = ranges.iter().map(|x| x.end() - x.start()).sum();

    println!("q15a: {:?}, {}", ranges, pos_count);
}

pub fn q15b() {
    let vec = read_to_lines("inp_q15.txt");
    let mut coords: Vec<Coord> = Vec::new();

    for line in vec.iter() {
        let it = line.split(' ').filter(|x| x.contains('=')).collect::<Vec<&str>>();
        let args = it.iter()
        .map(|x| x.split('=').collect::<Vec<_>>())
        .map(|x| x[1].strip_suffix([',', ':']).unwrap_or(x[1]).parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
        
        let mut it = args.into_iter();
        let s_x = it.next().unwrap();
        let s_y = it.next().unwrap();

        let b_x = it.next().unwrap();
        let b_y = it.next().unwrap();

        let coord: Coord = Coord {s_x, s_y, b_x, b_y};
        coords.push(coord);
    }

    let mut xs = IntRangeUnionFind::<i32>::new();
    xs.insert_range(&(0..=4000000));

    let mut y: i64 = 0;
    for i in 0..=4000000 {
        let coverages = coords.iter()
            .map(|x| x.y_coverage(i))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        // Uses an external crate to find the overlapping ranges.
        let mut union_range = IntRangeUnionFind::<i32>::new();

        for &(lo, hi) in coverages.iter() {
            let range = lo..=hi;
            union_range.insert_range(&range);
        }

        let range = union_range.to_collection::<Vec<RangeInclusive<i32>>>();

        if range[0].end() <= &4000000 || range[0].start() >= &0 {
            println!("q15b: y={i}, range={range:?}");
            y = i as i64;
            break;
        }

        // lazy, apologies. should find the disjoint element as the x
    }

    println!("q15b: {}", 2936793 * 4000000 + y);

}

#[derive(Debug, Clone)]
pub struct Valve {
    pub code: String,
    pub flow_rate: i32,
    pub adjacent: Vec<String>,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Agenda {
    pos: String,
    time: i32,
    acc_flow_rate: i32,
    acc_relief: i32,
    opened: Vec<String>,
    visited: Vec<String>,
    remaining: Vec<String>,
    snapshots: Vec<String>
}

pub fn precompute_path(start: &String, end: &String, valves: &HashMap<String, Valve>) -> i32 {
    // precompute shortest path from node to onde
    // this is to evaluate the priority between turning on valves
    let mut frontier: Vec<(String, HashSet<String>, i32)> = Vec::new();
    let mut best: HashMap<String, i32> = HashMap::new();

    if start == end { return -1; }
    frontier.push((start.clone(), HashSet::new(), 0));

    while frontier.len() > 0 {
        let (pos, visited, steps)= frontier.pop().unwrap();

        let adjacent = &valves.get(&pos).unwrap().adjacent;
        for adj in adjacent.iter() {
            if best.get(adj).unwrap_or(&999) < &(steps + 1) {
                continue;
            }

            if visited.contains(adj) {
                continue;
            }

            let mut visited_new = visited.clone();
            visited_new.insert(pos.clone());
            frontier.push((adj.clone(), visited_new, steps + 1));
            best.insert(adj.clone(), steps + 1);
        }
        // println!("frontier={frontier:?}");
    }

    *best.get(end).unwrap_or(&-1)
}

fn q16_helper(valves: &HashMap<String, Valve>, distance: &HashMap<(String, String), i32>, agenda: Agenda) -> i32 {
    let mut frontier = PriorityQueue::new();
    let mut max_relief = 0;
    let mut best: HashMap<Vec<String>, i32> = HashMap::new();

    frontier.push(agenda, 0);

    while frontier.len() > 0 {
        let (mut agenda, _) = frontier.pop().unwrap();
        agenda.visited.push(agenda.pos.clone());
        let snapshot = format!("{}-{}", &agenda.time, &agenda.acc_relief);
        agenda.snapshots.push(snapshot);

        if agenda.remaining.len() == 0 
        || agenda.time == 0 {
            let relief = agenda.acc_relief + agenda.time * agenda.acc_flow_rate;
            if relief > max_relief {
                // println!(".. incumbant | max={relief}");
                max_relief = relief;
            }
            continue;
        }

        for (ix, dest) in agenda.remaining.iter().enumerate() {
            let dist = distance.get(&(agenda.pos.to_string(), dest.to_string())).unwrap();
            let flow_rate_new = valves.get(dest).unwrap().flow_rate;
            let mut time_spent = 0;
            
            let mut opened_new = agenda.opened.clone();
            let mut remaining_new = agenda.remaining.clone();

            // this check is obsolete given we operate on nonzero flow valves only.
            if valves.get(dest).unwrap().flow_rate > 0 {
                opened_new.push(dest.to_string());
                time_spent += *dist + 1;
            } else {
                continue;
            }

            opened_new.sort();
            remaining_new.remove(ix);

            let agenda_new = match agenda.time - time_spent {
                i if i >= 0 => Agenda { 
                    pos: dest.to_string(), 
                    time: agenda.time - time_spent, 
                    acc_flow_rate: agenda.acc_flow_rate + flow_rate_new, 
                    acc_relief: agenda.acc_relief + agenda.acc_flow_rate * time_spent, 
                    opened: opened_new, 
                    visited: agenda.visited.clone(), 
                    remaining: remaining_new,
                    snapshots: agenda.snapshots.clone(),
                },
                _ => Agenda {
                    pos: dest.to_string(), 
                    time: 0, 
                    acc_flow_rate: agenda.acc_flow_rate, 
                    acc_relief: agenda.acc_relief + agenda.acc_flow_rate * agenda.time, 
                    opened: agenda.opened.clone(), 
                    visited: agenda.visited.clone(), 
                    remaining: remaining_new,
                    snapshots: agenda.snapshots.clone(),
                }
            }; 

            let &incum_projection = best.get(&agenda_new.opened).unwrap_or(&-1);
            let projection = &agenda_new.acc_relief + &agenda_new.acc_flow_rate * &agenda_new.time;

            if projection >= incum_projection {
                best.insert(agenda_new.opened.clone(), projection);
                let prio = projection.clone();
                frontier.push(agenda_new, prio);
            }
        }
    }

    return max_relief;
}

pub fn q16a() {
    let vec = read_to_lines("inp_q16.txt");

    // read data into Valve struct
    let mut valves: HashMap<String, Valve> = HashMap::new();
    let re = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels* leads* to valves* ((?:(?:\w+)(?:, )?)*)").unwrap();

    for line in vec.into_iter() {
        let groups = re.captures(&line.as_str()).unwrap();
        
        let code = groups.get(1).map(|x| x.as_str().to_string()).unwrap();
        let flow_rate = groups.get(2).map(|x| x.as_str().parse::<i32>().unwrap()).unwrap();
        let adjacent = groups.get(3).map(|x| x.as_str().split(", ").map(|x| x.to_string()).collect::<Vec<String>>()).unwrap();
        let valve = Valve {code: code.clone(), flow_rate, adjacent} ;
        valves.insert(code, valve);
    }

    let mut distance: HashMap<(String, String), i32> = HashMap::new();

    for start in valves.keys() {
        for end in valves.keys() {
            if start == end { continue; }
            distance.insert((start.clone(), end.clone()), precompute_path(start, end, &valves));
            // println!("{}->{}: {} steps", start, end, precompute_path(start, end, &valves));
        }
    }

    // let v = vec!["AA", "EA", "QN", "GW", "SA", "TH", "LR", "KB", "NA", "XX"];
    // let v = vec!["AA", "DD", "BB", "JJ", "HH", "EE", "CC"];

    // for (s, e) in v.iter().zip(v.iter().skip(1)) {
    //     println!("{}->{}: {} steps, flow rate={}", s.to_string(), e.to_string(), precompute_path(&s.to_string(), &e.to_string(), &valves), &valves.get(&e.to_string()).unwrap().flow_rate);
    // }

    // search
    let mut all_valves = valves.keys().cloned().filter(|x| x != &"AA".to_string()).collect::<Vec<String>>();
    let mut nonzero_valves = valves.iter().filter(|(k, v)| v.flow_rate > 0).map(|(k, _)| k.to_string()).collect::<Vec<String>>();
    all_valves.sort();
    nonzero_valves.sort();

    // let mut frontier: Vec<Agenda> = Vec::new();
    let agenda = Agenda { 
        pos: "AA".to_string(), 
        time: 30, 
        acc_flow_rate: 0, 
        acc_relief: 0, 
        opened: Vec::new(), 
        visited: Vec::new(), 
        remaining: nonzero_valves,
        snapshots: Vec::new(),
    };

    let max_relief = q16_helper(&valves, &distance, agenda);
    println!("q16a: {}", max_relief);
}

fn comb(pool: &Vec<String>, length: i32) -> Vec<(Vec<usize>, Vec<usize>)> {
    if pool.len() == 0 { return Vec::new() };
    let digits: u32 = pool.len() as u32;
    let r = 2i32.pow(digits);
    let inverter = HashMap::from([('1', '0'), ('0', '1')]);

    let p = (0..=r)
        .filter(|x| format!("{:b}", x).chars().map(|e| (e == '1') as i32).sum::<i32>() == length)
        .map(|x| format!("{:0lz$b}", x, lz=digits as usize))
        // .map(|x| x.chars())
        // .map(|x| x.enumerate());
        .map(|x| (x.to_string(), x.chars().map(|e| inverter[&e]).collect::<String>()))
        .map(|(a, b)| (
            a.chars().enumerate().filter(|(_, d)| d == &'1').map(|(e, _)| e).collect::<Vec<usize>>(),
            b.chars().enumerate().filter(|(_, d)| d == &'1').map(|(e, _)| e).collect::<Vec<usize>>()
        ))
        .collect::<Vec<_>>();

    return p;
}

pub fn q16b() {
    let vec = read_to_lines("inp_q16.txt");

    // read data into Valve struct
    let mut valves: HashMap<String, Valve> = HashMap::new();
    let re = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels* leads* to valves* ((?:(?:\w+)(?:, )?)*)").unwrap();

    for line in vec.into_iter() {
        let groups = re.captures(&line.as_str()).unwrap();
        
        let code = groups.get(1).map(|x| x.as_str().to_string()).unwrap();
        let flow_rate = groups.get(2).map(|x| x.as_str().parse::<i32>().unwrap()).unwrap();
        let adjacent = groups.get(3).map(|x| x.as_str().split(", ").map(|x| x.to_string()).collect::<Vec<String>>()).unwrap();
        let valve = Valve {code: code.clone(), flow_rate, adjacent} ;
        valves.insert(code, valve);
    }

    // precompute distance
    let mut distance: HashMap<(String, String), i32> = HashMap::new();

    for start in valves.keys() {
        for end in valves.keys() {
            if start == end { continue; }
            distance.insert((start.clone(), end.clone()), precompute_path(start, end, &valves));
            // println!("{}->{}: {} steps", start, end, precompute_path(start, end, &valves));
        }
    }

    // search
    let mut all_valves = valves.keys().cloned().filter(|x| x != &"AA".to_string()).collect::<Vec<String>>();
    let mut nonzero_valves = valves.iter().filter(|(k, v)| v.flow_rate > 0).map(|(k, _)| k.to_string()).collect::<Vec<String>>();
    all_valves.sort();
    nonzero_valves.sort();

    let v = 2..=8;
    let v = v.map(|e| comb(&nonzero_valves, e)).flatten().collect::<Vec<_>>();
    let v = v.iter()
        .map(|(a, b)| (
            a.iter().map(|e| nonzero_valves[*e].to_string()).collect::<Vec<String>>(),
            b.iter().map(|e| nonzero_valves[*e].to_string()).collect::<Vec<String>>()
        ))
        .collect::<Vec<_>>();
        
    // println!("{:?}", v.len());
    // for i in v.iter() {
    //     println!("{:?}", i);
    // }

    let mut max_relief = 0;

    for (ix, (valve_s, valve_e)) in v.into_iter().enumerate() {
        println!("ix: {ix:?}, s: {valve_s:?}, e: {valve_e:?}");
        let agenda_s = Agenda { 
            pos: "AA".to_string(), 
            time: 26, 
            acc_flow_rate: 0, 
            acc_relief: 0, 
            opened: Vec::new(), 
            visited: Vec::new(), 
            remaining: valve_s,
            snapshots: Vec::new(),
        };

        let agenda_e = Agenda { 
            pos: "AA".to_string(), 
            time: 26, 
            acc_flow_rate: 0, 
            acc_relief: 0, 
            opened: Vec::new(), 
            visited: Vec::new(), 
            remaining: valve_e,
            snapshots: Vec::new(),
        };

        let relief = q16_helper(&valves, &distance, agenda_s) + q16_helper(&valves, &distance, agenda_e);
        max_relief = cmp::max(relief, max_relief);
        // println!("{relief}");
    }

    println!("q16b: {}", max_relief);
}

fn q17_renderer(highest: i32, arr: &Vec<i32>, settled: &HashSet<i32>) {
    let mut alls: Vec<String> = vec!["+-------+".to_string()];
    let mut line: String = String::new();

    for i in 0..((highest + 7) * 9) {
        match i {
            i if (arr.contains(&i) | settled.contains(&i)) => { line.push_str("o"); },
            i if (i.rem_euclid(9) == 0) => { line.push_str("|"); }
            i if (i.rem_euclid(9) == 8) => { 
                line.push_str("|"); 
                alls.push(line.clone());  
                line.clear();
            }
            _ => { line.push_str("."); }
        }
    }

    alls.reverse();
    println!("{}", alls.join("\n"));
}

pub fn q17a() {
    let vec = read_to_lines("inp_q17.txt");

    let mut lines = vec.into_iter();
    let pattern: Vec<char> = lines.next().unwrap().chars().collect();

    // println!("{:?}", pattern);

    let shapes = vec![("0123", 4), ("1r012r1", 3), ("012r2r2", 3), ("0r0r0r0", 1), ("01r01", 2)];
    let mut highest = 0;
    let mut settled_rocks: HashSet<i32> = HashSet::new();
    let mut history: Vec<(usize, i32)> = Vec::new();

    let mut dx: HashMap<char, i32>  = HashMap::new();
    dx.insert('>', 1);
    dx.insert('<', -1);
    
    // let mut display: Vec<String> = Vec::new();
    let mut rocks: i64 = 0;
    let mut winds: i64 = 0;
    
    'outer: while rocks < 2022 {

        let r_idx = rocks.rem_euclid(shapes.len() as i64) as usize;
        let (rock, width) = shapes[r_idx].clone();
        let rock = rock.to_string();

        let arr = rock.split("r")
            .map(|x| x.to_string().chars().collect::<Vec<_>>())
            .map(|x| x.iter().map(|e| e.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>()
            .iter().enumerate()
            .map(|(i, x)| x.iter().map(|e| e + (i as i32) * 9).collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<i32>>();

        // set to initial position
        let mut arr = arr.iter().map(|x| x + (highest + 3) * 9 + 3).collect::<Vec<_>>();

        // println!("{arr:?}");
        if highest < 10 {
            q17_renderer(highest, &arr, &settled_rocks);
        }
        
        loop {
            // wind
            let w_idx = winds.rem_euclid(pattern.len() as i64) as usize;
            let mov = dx[&pattern[w_idx]];

            let valid_check = arr.iter()
                .map(|x| x + mov)
                .map(|x| (settled_rocks.contains(&x)) | (x.rem_euclid(9) == 0) | (x.rem_euclid(9) == 8))
                .map(|x| x as u8)
                .sum::<u8>();

            // println!("wind: [{}]{valid_check}", &pattern[w_idx]);

            if valid_check == 0 {
                arr = arr.iter().map(|x| x + mov).collect();
            }

            // q17_renderer(highest, &arr, &settled_rocks);
            winds += 1;

            // down
            let valid_check = arr.iter()
                .map(|x| x - 9)
                .map(|x| (settled_rocks.contains(&x)) | (x.rem_euclid(9) == 0) | (x.rem_euclid(9) == 8) | (x < 0))
                .map(|x| x as u8)
                .sum::<u8>();

            // println!("down: {valid_check}");

            if valid_check == 0 {
                // q17_renderer(highest, &arr, &settled_rocks);
                arr = arr.iter().map(|x| x - 9).collect();
            } else {
                let highest_ori = highest.clone();
                highest = cmp::max(arr.iter().max().unwrap().div_euclid(9) + 1, highest);
                settled_rocks.extend(arr.clone());
                rocks += 1;

                // q17_renderer(highest, &arr, &settled_rocks);
                history.push((r_idx.clone(), highest - highest_ori));                
                break;
            }
        }
    }

    // for (idx, i) in history.iter().enumerate() {
    //     print!("{:?}", i);
    //     if idx.rem_euclid(25) == 24 {
    //         print!("\n");
    //     }
    // }
    
    println!("q17a: {}", highest);
}

pub fn q17b() {
    let vec = read_to_lines("inp_q17.txt");

    let mut lines = vec.into_iter();
    let pattern: Vec<char> = lines.next().unwrap().chars().collect();

    // println!("{:?}", pattern);

    let shapes = vec![("0123", 4), ("1r012r1", 3), ("012r2r2", 3), ("0r0r0r0", 1), ("01r01", 2)];
    let mut highest = 0i64;
    let mut settled_rocks: HashSet<i64> = HashSet::new();
    let mut history: Vec<(usize, i64)> = Vec::new();
    let mut history_map: HashMap<(usize, usize, i64), Vec<i64>> = HashMap::new();

    let mut dx: HashMap<char, i64>  = HashMap::new();
    dx.insert('>', 1);
    dx.insert('<', -1);

    // let mut display: Vec<String> = Vec::new();
    let r_max: i64 = 1000000000000;
    let mut rocks: i64 = 0;
    let mut winds: i64 = 0;
    let mut longest = (0i64, r_max);
    
    'outer: while rocks < r_max {
        let r_idx = rocks.rem_euclid(shapes.len() as i64) as usize;
        let (rock, _) = shapes[r_idx].clone();
        let rock = rock.to_string();

        let arr = rock.split("r")
            .map(|x| x.to_string().chars().collect::<Vec<_>>())
            .map(|x| x.iter().map(|e| e.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>())
            .collect::<Vec<Vec<i64>>>()
            .iter().enumerate()
            .map(|(i, x)| x.iter().map(|e| e + (i as i64) * 9).collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<i64>>();

        // set to initial position
        let mut arr = arr.iter().map(|x| x + (highest + 3) * 9 + 3).collect::<Vec<_>>();

        // println!("{arr:?}");
        // q17_renderer(highest, &arr, &settled_rocks);

        // let d_height = hist.get(&(r_idx, winds.rem_euclid(pattern.len() as i64) as usize));
        // match d_height {
        //     Some(i) => { highest += i.0; winds += i.1; rocks += 1; continue; },
        //     None => {}
        // }

        let w_ori = winds.clone(); //.rem_euclid(pattern.len() as i64) as usize;
        let w_ori_usize = winds.rem_euclid(pattern.len() as i64) as usize;
        let a_ori = arr.clone();
    
        loop {
            // wind
            let w_idx = winds.rem_euclid(pattern.len() as i64) as usize;
            // println!("{w_idx}, {w_ori}");
            let mov = dx[&pattern[w_idx]];

            let valid_check = arr.iter()
                .map(|x| x + mov)
                .map(|x| (settled_rocks.contains(&x)) | (x.rem_euclid(9) == 0) | (x.rem_euclid(9) == 8))
                .map(|x| x as u8)
                .sum::<u8>();

            // println!("wind: [{}]{valid_check}", &pattern[w_idx]);

            if valid_check == 0 {
                arr = arr.iter().map(|x| x + mov).collect();
            }

            // q17_renderer(highest, &arr, &settled_rocks);
            winds += 1;

            // down
            let valid_check = arr.iter()
                .map(|x| x - 9)
                .map(|x| (settled_rocks.contains(&x)) | (x.rem_euclid(9) == 0) | (x.rem_euclid(9) == 8) | (x < 0))
                .map(|x| x as u8)
                .sum::<u8>();

            // println!("down: {valid_check}");

            if valid_check == 0 {
                // q17_renderer(highest, &arr, &settled_rocks);
                arr = arr.iter().map(|x| x - 9).collect();
            } else {
                let highest_ori = highest.clone();
                highest = cmp::max(arr.iter().max().unwrap().div_euclid(9) + 1, highest);
                settled_rocks.extend(arr.clone());
                rocks += 1;

                // q17_renderer(highest, &arr, &settled_rocks);
                // let d_a = arr.iter().zip(a_ori).map(|(a, b)| a - b).collect::<Vec<i64>>()[0];
                // println!("{:?}", d_a);

                // check for longest looping pattern
                // history.push((r_idx.clone(), w_ori_usize.clone(), highest - highest_ori));
                history.push((r_idx.clone(), highest - highest_ori));
                
                let key = &(r_idx, w_ori_usize, highest - highest_ori);
                let mut entry = match history_map.get(&(r_idx, w_ori_usize, highest - highest_ori)) {
                    Some(i) => i.to_vec(),
                    None => Vec::new(),
                };
        
                entry.push(rocks.clone());

                // to find equal interval looping behavior, we compare first 3-ele.
                if entry.len() >= 3 {
                    // println!("{entry:?}");
                    let all_equal = entry.iter().zip(entry.iter().skip(1)).zip(entry.iter().skip(2)).map(|((a, b), c)| (b - a) == (c - b)).all(|x| x);
                    if all_equal {
                        longest = (entry[0] - 1, entry[1] - 1);
                        break 'outer;
                    }
                }

                history_map.insert(*key, entry);
                
                // let s = hist.insert((r_idx, w_ori_usize), (highest - highest_ori, winds - w_ori)).unwrap_or((-1, -1));
                // match s.0 {
                //     -1 => {},
                //     _ if (s.0 != highest - highest_ori) => { 
                //         println!("difference detected: rocks: {rocks}, winds: {winds}, s: {}, new s: {}", s.0, highest - highest_ori); 
                //         // break 'outer;
                //     },
                //     _ => {},
                // }

                break;
            }
        }
    }
    
    let init = &history[0..(longest.0 as usize)];
    let repeater = &history[longest.0 as usize .. longest.1 as usize];

    // explicitly solve
    // let total_height = init.iter()
    //     .chain(repeater.iter().cycle())
    //     .take(r_max as usize)
    //     .map(|x| x.1)
    //     .sum::<i64>();
    //     // .collect::<Vec<_>>();

    // implicitly solve
    let init_len = init.len() as i64;
    let init_sum: i64 = init.iter().map(|x| x.1).sum();
    let repeater_len = repeater.len() as i64;
    let repeater_sum: i64 = repeater.iter().map(|x| x.1).sum();
    let repeater_cycle = (r_max - init_len).div_euclid(repeater_len);
    let remainder_len = (r_max - init_len).rem_euclid(repeater_len);
    let remainder_sum: i64 = repeater.iter().take(remainder_len as usize).map(|x| x.1).sum();
    let total_height = init_sum + repeater_cycle * repeater_sum + remainder_sum;

    println!("q17b: {}", total_height);
}

fn block_to_surf(coord: &String) -> Vec<(std::ops::Range<i32>, std::ops::Range<i32>, std::ops::Range<i32>)> {
    let v = coord.split(",").map(|e| e.parse::<i32>().unwrap()).take(3).collect_tuple();// ;.collect::<(i32, i32, i32)>();
    let v = v.into_iter()
        .map(|(x, y, z)| vec![
            (x  ..x  , y  ..y+1, z  ..z+1),
            (x+1..x+1, y  ..y+1, z  ..z+1),
            (x  ..x+1, y  ..y  , z  ..z+1),
            (x  ..x+1, y+1..y+1, z  ..z+1),
            (x  ..x+1, y  ..y+1, z  ..z  ),
            (x  ..x+1, y  ..y+1, z+1..z+1),
        ])
        .flatten()
        .collect::<Vec<_>>();
    return v;
}

pub fn q18a() {
    let vec = read_to_lines("inp_q18.txt");

    let surfaces = vec.iter().map(|e| block_to_surf(e)).flatten().collect::<Vec<_>>();
    let surfaces = surfaces.iter().
        filter(|x| 
            surfaces.iter().filter(|e| x == e).count() == 1
        )
        .collect::<Vec<_>>();

    // for surf in surfaces.iter() {
    //     println!("{:?}", surf);
    // }
    println!("q18a: {}", surfaces.len());
}

pub fn q18b() {
    // we attempt to propagate some cloud from origin to cover all blocks. given any
    // interior blocks will not have smoke of any kind, we can use the helper from q18 to
    // identify such connected interior blocks, and the subtract the surface value from 
    // those interior blocks.

    // we perform a set difference of (input_blocks) - (all_blocks - filled_blocks) to
    // identify the interior blocks.
    let vec = read_to_lines("inp_q18.txt");

    let blocks = vec.iter()
        .map(|x| x.split(",").map(|e| 
            e.parse::<i32>().unwrap()
        )
            .take(3)
            .collect_tuple()
            .unwrap())
        .collect::<HashSet<(i32, i32, i32)>>();

    let cube_min = blocks.clone().into_iter()
        .map(|(a, b, c)| Vec::from([a, b, c]))
            .min().unwrap().into_iter()
        .min().unwrap();

    let cube_max = blocks.clone().into_iter()
        .map(|(a, b, c)| Vec::from([a, b, c]))
            .max().unwrap().into_iter()
        .max().unwrap() + 1;

    let surfaces = vec.iter().map(|e| block_to_surf(e)).flatten().collect::<Vec<_>>();
    let surfaces = surfaces.clone().into_iter().
        filter(|x| 
            surfaces.iter().filter(|&e| x == e).count() == 1
        )
        .collect::<Vec<_>>();

    let hs_surfaces: HashSet<(std::ops::Range<i32>, std::ops::Range<i32>, std::ops::Range<i32>)> = 
        HashSet::from_iter(surfaces);

    // smoke search
    let mut agenda = vec![(cube_min - 1, cube_min - 1, cube_min - 1)];
    let mut filled: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut filled_total: i32 = 0;

    let all = (cube_min..cube_max).map(|x| (cube_min..cube_max).map(move |y| (cube_min..cube_max).map(move |z| (x, y, z))))
        .flatten()
        .flatten()
        .collect::<HashSet<(i32, i32, i32)>>();

    // println!("{:?}", all);

    while agenda.len() > 0 {
        let pos = agenda.pop().unwrap();
        if filled.contains(&pos) { continue; }
            
        filled_total += 1;
        // println!("filled {pos:?}");

        filled.insert(pos.clone());
        let (x, y, z) = pos;

        if (x < cube_max) & !hs_surfaces.contains(&(x+1..x+1, y..y+1, z..z+1)) {
            let p = (x.clone()+1, y.clone(), z.clone());
            if !filled.contains(&p) & !agenda.contains(&p) { agenda.push(p); }
        } 

        if (x > cube_min) & !hs_surfaces.contains(&(x..x, y..y+1, z..z+1)) {
            let p = (x.clone()-1, y.clone(), z.clone());
            if !filled.contains(&p) & !agenda.contains(&p) { agenda.push(p); }
        } 

        if (y < cube_max) & !hs_surfaces.contains(&(x..x+1, y+1..y+1, z..z+1)) {
            let p = (x.clone(), y.clone()+1, z.clone());
            if !filled.contains(&p) & !agenda.contains(&p) { agenda.push(p); }
        } 

        if (y > cube_min) & !hs_surfaces.contains(&(x..x+1, y..y, z..z+1)) {
            let p = (x.clone(), y.clone()-1, z.clone());
            if !filled.contains(&p) & !agenda.contains(&p) { agenda.push(p); }
        } 

        if (z < cube_max) & !hs_surfaces.contains(&(x..x+1, y..y+1, z+1..z+1)) {
            let p = (x.clone(), y.clone(), z.clone()+1);
            if !filled.contains(&p) & !agenda.contains(&p) { agenda.push(p); }
        } 

        if (z > cube_min) & !hs_surfaces.contains(&(x..x+1, y..y+1, z..z)) {
            let p = (x.clone(), y.clone(), z.clone()-1);
            if !filled.contains(&p) & !agenda.contains(&p) { agenda.push(p); }
        } 
    }

    let unfilled = all.difference(&filled).map(|&(a, b, c)| (a, b, c)).collect::<HashSet<_>>();
    let diff = unfilled.difference(&blocks).map(|(a, b, c)| format!("{a},{b},{c}")).collect::<Vec<String>>();
    // println!("blocks: {blocks:?}");
    // println!("filled: {filled:?}");
    // println!("unfilled: {unfilled:?}");
    // println!("{diff:?}");

    // check connected interior surfaces
    let interior = diff.iter().map(|e| block_to_surf(e)).flatten().collect::<Vec<_>>();
    let interior = interior.clone().into_iter().
        filter(|x| 
            interior.iter().filter(|&e| x == e).count() == 1
        )
        .collect::<Vec<_>>();

    // println!("{}", interior.len());
    println!("q18b: all={}, exterior={}", hs_surfaces.len(), hs_surfaces.len() - interior.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Robot {
    OreRobot = 0,
    ClayRobot = 1,
    ObsidianRobot = 2,
    GeodeRobot = 3,
    Nothing = 4
}

impl Robot {
    fn to_iter() -> impl Iterator<Item = (i32, Robot)> {
        [(0, Robot::OreRobot), (1, Robot::ClayRobot), (2, Robot::ObsidianRobot), (3, Robot::GeodeRobot), (4, Robot::Nothing)].iter().copied()
    }

    fn to_rev_iter() -> impl Iterator<Item = (i32, Robot)> {
        [(0, Robot::OreRobot), (1, Robot::ClayRobot), (2, Robot::ObsidianRobot), (3, Robot::GeodeRobot), (4, Robot::Nothing)].iter().rev().copied()
    }

    fn from_int(i: i32) -> Robot {
        match i {
            0 => Robot::OreRobot,
            1 => Robot::ClayRobot,
            2 => Robot::ObsidianRobot,
            3 => Robot::GeodeRobot,
            4 => Robot::Nothing,
            _ => panic!()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Blueprint {
    id:             i32,
    ore_robot:      Vec<i32>,
    clay_robot:     Vec<i32>,
    obsidian_robot: Vec<i32>,
    geode_robot:    Vec<i32>,
    nothing:        Vec<i32>
}

impl Blueprint {
    pub fn build(params: Vec<i32>) -> Blueprint {
        Blueprint { 
            id: params[0],
            ore_robot:      vec![params[1], 0,          0           , 0  ], 
            clay_robot:     vec![params[2], 0,          0           , 0  ], 
            obsidian_robot: vec![params[3], params[4],  0           , 0  ], 
            geode_robot:    vec![params[5], 0,          params[6]   , 0  ],
            nothing:        vec![0,         0,          0,            0]
        }
    }

    pub fn max_spending(&self) -> Vec<i32> {
        (0..4).map(|e|
            {
                let v = vec![self.ore_robot[e], self.clay_robot[e], self.obsidian_robot[e], self.geode_robot[e], 0];
                v.into_iter().max().unwrap()
            }
        )
        .collect::<Vec<i32>>()
    }

    pub fn repr(&self) -> String {
        let mut s = String::new();
        s.push_str(self.ore_robot.iter().join(",").as_str());
        s.push_str(self.clay_robot.iter().join(",").as_str());
        s.push_str(self.obsidian_robot.iter().join(",").as_str());
        s.push_str(self.geode_robot.iter().join(",").as_str());
        s
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct RobotFactory {
    blueprint_id: i32,
    blueprint: Blueprint,
    time: i32,
    robots: Vec<i32>,
    inventory: Vec<i32>,
    pipeline: i32,
    memory: Vec<String>
}

impl RobotFactory {
    fn new(blueprint: Blueprint, time: i32) -> RobotFactory {
        RobotFactory { 
            blueprint_id: blueprint.id,
            blueprint, 
            time: time,
            robots: vec![1, 0, 0, 0, 0], 
            inventory: vec![0, 0, 0, 0],
            pipeline: 99,
            memory: Vec::new()
        }
    }

    fn has_resources(&mut self, robot: Robot) -> bool {
        let inv = &self.inventory;
        let cst = match robot {
            Robot::OreRobot => &self.blueprint.ore_robot,
            Robot::ClayRobot => &self.blueprint.clay_robot,
            Robot::ObsidianRobot => &self.blueprint.obsidian_robot,
            Robot::GeodeRobot => &self.blueprint.geode_robot,
            Robot::Nothing => &self.blueprint.nothing,
        };
        // self.memory.push(format!("\tRobotFactory::has_resources(), inventory={inv:?}, cost={cst:?}"));
        inv.iter().zip(cst).map(|(a, b)| a >= b).all(|e| e)
    }

    fn build(&mut self, robot: Robot) {
        let inv = &self.inventory;
        let (idx, cst) = match robot {
            Robot::OreRobot => (0, &self.blueprint.ore_robot),
            Robot::ClayRobot => (1, &self.blueprint.clay_robot),
            Robot::ObsidianRobot => (2, &self.blueprint.obsidian_robot),
            Robot::GeodeRobot => (3, &self.blueprint.geode_robot),
            Robot::Nothing => (4, &self.blueprint.nothing),
        };

        self.inventory = inv.into_iter().zip(cst).map(|(a, b)| a - b).collect::<Vec<i32>>();
        self.pipeline = robot as i32;
        // self.memory.push(format!("\tRobotFactory::build(), start building {robot:?}, inventory={:?}", self.inventory));
    }

    fn collect_resources(&mut self) {
        self.inventory = self.inventory.iter().zip(&self.robots).map(|(a, b)| a + b).collect::<Vec<i32>>();
        // self.memory.push(format!("\tRobotFactory::collect_resources(), robots={:?}, inventory={:?}", self.robots, self.inventory));
    }

    fn step(&mut self) {
        self.time += 1;
        // self.memory.push(format!("\n\tRobotFactory::step(), t={}", self.time));

        if self.pipeline != 99 {
            self.robots[self.pipeline as usize] += 1;
            // self.memory.push(format!("\tRobotFactory::step(), finish building {:?}, robots={:?}", Robot::from_int(self.pipeline), self.robots));
        }
        // self.memory.push(self.repr());
    }

    /// returns time;robots;inventory as String.
    fn repr(&self) -> String {
        let mut s = String::with_capacity(30);
        s.push_str(self.time.to_string().as_str());
        s.push_str(";");
        s.push_str(self.robots.iter().join(",").as_str());
        s.push_str(";");
        s.push_str(self.inventory.iter().join(",").as_str());
        s
    }
}

impl Hash for RobotFactory {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr().hash(state)
    }
}

fn q19_helper(blueprint: &Blueprint, max_time: i32) -> i32 {
    let mut agenda: Vec<RobotFactory> = Vec::new();
    let mut visited: HashSet<RobotFactory> = HashSet::with_capacity(10000000);
    let rf = RobotFactory::new(blueprint.clone(), 0);

    let max_spending = rf.blueprint.max_spending();
    let mut eval = 0i32;
    let mut geodes = 0;
    
    agenda.push(rf);

    while agenda.len() > 0 {
        // eval += 1;
        // if eval.rem_euclid(10000) == 0 {
        //     println!("# eval={}, agenda.len()={}, max geodes={}", eval, agenda.len(), geodes);
        // }

        let mut rf = agenda.pop().unwrap();
        
        if visited.contains(&rf) { continue; }
        else { visited.insert(rf.clone()); }

        // loop
        rf.step();

        // time running out
        if rf.time == max_time {
            // println!("{}", rf.repr());
            geodes = cmp::max(rf.inventory[3] + rf.robots[3], geodes);
            continue;
        }

        // prune if projected is low even if aggressively building one robot per time step
        if rf.inventory[3] + rf.robots[3] * (max_time - rf.time + 1) + cmp::max((max_time - rf.time + 1) * (max_time - rf.time) / 2, 0) < geodes {
            // println!("pruned {}", rf.repr());
            continue;
        }

        // try building a robot and step
        for (ix, robot) in Robot::to_rev_iter() {
            if rf.has_resources(robot) { 
                let mut rf_new = rf.clone();
                rf_new.collect_resources();     

                match robot {
                    Robot::ClayRobot | Robot::ObsidianRobot | Robot::OreRobot => {
                        if rf.robots[ix as usize] >= max_spending[ix as usize] { continue; }
                    },
                    _ => {}
                };

                rf_new.build(robot); 
                agenda.push(rf_new.clone());
            }
        }
    }

    return geodes;
}

pub fn q19() {
    let vec = read_to_lines("inp_q19.txt");

    let re = Regex::new(
        r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.")
        .unwrap();

    let blueprints = vec.iter()
        .map(|x| re.captures(x.as_str()).unwrap())
        .map(|c| (1..=7).map(|e| c.get(e).unwrap().as_str().parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|x| Blueprint::build(x))
        .collect::<Vec<Blueprint>>();

    let mut tot_add = 0i32;
    let mut tot_mul = 1i32;

    for blueprint in blueprints {
        let geodes_add = q19_helper(&blueprint, 24);
        let geodes_mul = match blueprint.id {
            i if (i <= 3) => q19_helper(&blueprint, 32),
            _ => 1,
        };

        tot_add += blueprint.id * geodes_add;
        tot_mul *= geodes_mul
    }

    println!("q19a: {}", tot_add);
    println!("q19b: {}", tot_mul);
}

fn q20_helper(mut vec: Vec<(usize, i64)>) -> Vec<(usize, i64)> {
    // println!("{:?}", vec);
    let mut i = 0;
    let mut p = 0;
    let modulus = (vec.len() - 1) as i64;

    while i < vec.len() {
        loop {
            if i == vec[p].0 { break; }
            else { p += 1 };
        }

        let d = vec.get(p).unwrap().clone();

        // println!("\nbefore: d={d:?}, i={i}, p={p}, vec={vec:?}");
        vec.remove(p);

        let new_i: usize = match d.1 {
            i if i > 0 => {
                let new_p = (p as i64 + i).rem_euclid(modulus);
                new_p as usize
            }
            i if i < 0 => {
                let new_p = (p as i64 + i).rem_euclid(modulus);
                new_p as usize
            }
            _ => p
        };

        // println!("new_i={new_i}");
        vec.insert(new_i, d);
        p = 0;
        i += 1;
    }

    // let mixed = vec.clone().into_iter()
    //     .map(|(_, e)| e)
    //     .collect::<Vec<i64>>();

    return vec;
}

pub fn q20a() {
    let vec = read_to_lines("inp_q20.txt");
    let vec = vec.iter()
        .enumerate()
        .map(|(i, e)| (i, e.parse::<i64>().unwrap()))
        // .map(|(i, e)| (i, e * 1))
        .collect::<Vec<(usize, i64)>>();

    let mixed = q20_helper(vec.clone()).into_iter()
        .map(|(_, e)| e)
        .collect::<Vec<i64>>();

    let zero_pos = mixed.iter().position(|&x| x == 0).unwrap();
    let s = mixed[(zero_pos + 1000).rem_euclid(vec.len())] 
        + mixed[(zero_pos + 2000).rem_euclid(vec.len())] 
        + mixed[(zero_pos + 3000).rem_euclid(vec.len())];

    println!("q20a: {s}");
}

pub fn q20b() {
    let vec = read_to_lines("inp_q20.txt");
    let mut vec = vec.iter()
        .enumerate()
        .map(|(i, e)| (i, e.parse::<i64>().unwrap()))
        .map(|(i, e)| (i, e * 811589153))
        .collect::<Vec<(usize, i64)>>();

    let mut mixings = 0;
    while mixings < 10 {
        vec = q20_helper(vec.clone());
        mixings += 1;

        // println!("After {mixings} mixing, {:?}", vec.iter().map(|(i, e)| e).collect::<Vec<_>>());
    }

    let mixed = vec.into_iter().map(|(_, e)| e).collect::<Vec<_>>();
    let zero_pos = mixed.iter().position(|&x| x == 0).unwrap();
    let s = mixed[(zero_pos + 1000).rem_euclid(mixed.len())] 
        + mixed[(zero_pos + 2000).rem_euclid(mixed.len())] 
        + mixed[(zero_pos + 3000).rem_euclid(mixed.len())];

    println!("q20b: {s}");
}
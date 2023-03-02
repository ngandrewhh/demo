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
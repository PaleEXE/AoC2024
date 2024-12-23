pub fn run(input: String) {
    let mut blocks: Vec<Option<u128>> = Vec::new();
    let mut size: Vec<u128> = Vec::new();
    let mut loc: Vec<usize> = Vec::new();

    let mut is_file = true;
    let mut id = 0;
    
    for x in input.chars() {
        let x = x.to_digit(10).unwrap() as usize;
        if is_file {
            loc.push(blocks.len());
            size.push(x as u128);
            blocks.extend(vec![Some(id); x]);
            id += 1;
            is_file = false;
        } else {
            blocks.extend(vec![None; x]);
            is_file = true;
        }
    }

    move_blocks(&mut blocks, &size, &loc);
    
    let rizz = checksum(&blocks);
    
    println!("{}", rizz);
}

fn move_blocks(arr: &mut [Option<u128>], size: &[u128], loc: &[usize]) {
    let mut big = 0;
    while big < size.len() && size[big] > 0 {
        big += 1;
    }
    big -= 1;

    for to_move in (0..=big).rev() {
        let mut free_space = 0;
        let mut first_free = 0;
        
        while first_free < loc[to_move] && free_space < size[to_move] as usize {
            first_free += free_space;
            free_space = 0;
            while first_free < arr.len() && arr[first_free].is_some() {
                first_free += 1;
            }
            while first_free + free_space < arr.len() && arr[first_free + free_space].is_none() {
                free_space += 1;
            }
        }

        if first_free >= loc[to_move] {
            continue;
        }

        for idx in first_free..std::cmp::min(first_free + size[to_move] as usize, arr.len()) {
            arr[idx] = Some(to_move as u128);
        }

        for idx in loc[to_move]..std::cmp::min(loc[to_move] + size[to_move] as usize, arr.len()) {
            arr[idx] = None;
        }
    }
}

fn checksum(arr: &[Option<u128>]) -> u128 {
    arr.iter().enumerate().filter_map(|(i, &x)| {
        x.map(|v| (i as u128) * v)
    }).sum()
}
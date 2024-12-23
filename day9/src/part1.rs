pub fn run(input: String) {
    let mut blocks: Vec<Option<u128>> = Vec::new();
    let mut len_files: Vec<(usize, u128)> = Vec::new();
    let mut is_file = true;
    let mut id = 0;

    for x in input.chars() {
        let x = x.to_digit(10).unwrap() as usize;
        if is_file {
            len_files.push((x, id));
            blocks.extend(vec![Some(id); x]);
            id += 1;
            is_file = false;
        } else {
            blocks.extend(vec![None; x]);
            is_file = true;
        }
    }
    len_files.sort_by_key(|(_, x)| *x);
    move_blocks(&mut blocks);
    let rizz: u128 = blocks
        .iter()
        .enumerate()
        .filter_map(|(i,&x)| x.map(|v| i as u128 * v))
        .sum::<u128>();
    
    println!("{}", rizz)
}

fn move_blocks(arr: &mut [Option<u128>]) {
    let mut first_free = 0;
    while first_free < arr.len() && arr[first_free].is_some() {
        first_free += 1;
    }

    let mut i = arr.len() - 1;
    while arr[i].is_none() {
        i -= 1;
    }

    while i > first_free {
        arr[first_free] = arr[i];
        arr[i] = None;

        while i > 0 && arr[i].is_none() {
            i -= 1;
        }
        while first_free < arr.len() && arr[first_free].is_some() {
            first_free += 1;
        }
    }
}

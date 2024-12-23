pub fn run(input: String) {
    let mut blocks: Vec<Option<u128>> = Vec::new();
    let mut len_files: Vec<u128> = Vec::new();
    let mut is_file = true;
    let mut id = 0;

    for x in input.chars() {
        let x = x.to_digit(10).unwrap() as usize;
        if is_file {
            len_files.push(x as u128);
            blocks.extend(vec![Some(id); x]); // Add file blocks
            id += 1;
            is_file = false;
        } else {
            blocks.extend(vec![None; x]);
            is_file = true;
        }
    }

    move_blocks(&mut blocks, &mut len_files); 
    
    let rizz: u128 = blocks
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| x.map(|v| i as u128 * v)) // Only include non-None blocks
        .sum();

    println!("{}", rizz);
}

fn move_blocks(arr: &mut Vec<Option<u128>>, len_files: &mut Vec<u128>) {
    let mut start_free = 0;
    let mut last_start = 0;
    let o_arr = arr.clone(); 

    loop {
        let mut moved: Vec<usize> = Vec::new();
        while start_free < arr.len() && o_arr[start_free].is_some() {
            start_free += 1;
        }

        if start_free == last_start || start_free >= o_arr.len() {
            break;
        }
        last_start = start_free;

        let mut end_free = start_free;
        while end_free < arr.len() && o_arr[end_free].is_none() {
            end_free += 1;
        }

        if end_free <= start_free {
            break;
        }

        let mut diff = end_free - start_free;

        for (index, &file_size) in len_files.iter().enumerate().rev() {
            if file_size <= diff as u128 {
                for i in start_free..start_free + file_size as usize {
                    arr[i] = Some(index as u128);
                }
                diff -= file_size as usize;
                start_free += file_size as usize;
                for i in start_free..arr.len() {
                    if arr[i] == Some(index as u128) { 
                        arr[i] = None;
                    }
                }
                moved.push(index)
            }
        }
        for file in moved {
            len_files[file] = u128::MAX; 
        }
    }
}

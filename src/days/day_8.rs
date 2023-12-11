use std::collections::HashMap;

#[allow(unused)]
pub fn part_1(network: &str) -> usize {
    let (instructions, nodes) = parse(network);

    let mut current = "AAA";
    let mut counter= 0usize;

    'outer: loop {
        for dir in instructions.iter() {
            current = if *dir { nodes.get(current).unwrap().0 } else { nodes.get(current).unwrap().1 };
            counter += 1;

            if current == "ZZZ" {break 'outer}
        }
    }
    counter
}


#[allow(unused)]
pub fn part_2(network: &str) -> usize {
    let (instructions, nodes) = parse(network);

    let mut start_points: Vec<Start> = nodes.keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| Start::new(&instructions,&nodes,k.as_str()))
        .collect();


    loop {
        let first = start_points[0].move_counter;
        let (min, max) = start_points.iter()
            .fold((first,first), |acc,e| (acc.0.min(e.move_counter), (acc.1.max(e.move_counter))));

        if max==min {return max}
        start_points.iter_mut()
            .filter(|s| s.move_counter < max)
            .for_each(|start| {
                while start.move_counter < max {
                    if start.kn_index==start.key_nodes.len() {
                        start.kn_index = 0;
                    }
                    start.move_counter += start.key_nodes[start.kn_index];
                }
        });
    }
}

struct Start {
    // moves_to_next
    // next node is always the next one
    key_nodes: Vec<usize>,
    move_counter: usize,
    kn_index: usize,
} impl Start {
    fn new(instructions: &Vec<bool>, nodes: &HashMap<String, (&str,&str)>, start: &str) -> Self {
        let mut result = Start{key_nodes:Vec::new(),move_counter:0, kn_index:0};
        let mut first: bool = true;

        // from_node, instruction_index, moves_to_node
        let mut node_visited: Vec<(&str,usize,usize)> = Vec::new();

        let mut current =  start;
        let mut last =  start;
        let mut counter = 0usize;
        'outer: loop {
            for (inst_i,dir) in instructions.iter().enumerate() {
                current = if *dir { nodes.get(current).unwrap().0 } else { nodes.get(current).unwrap().1 };
                counter += 1;

                if current.ends_with('Z') {
                    let key_node = (last,inst_i,counter);

                    if node_visited.contains(&key_node) {
                        break 'outer;
                    }
                    if first {
                        result.move_counter = counter;
                        first = false;
                    } else {
                        node_visited.push(key_node);
                        result.key_nodes.push(counter);

                    }
                    last = current;
                    counter = 0;
                }
            }
        }
        result
    }
}

fn parse(lines: &str) -> (Vec<bool>, HashMap<String, (&str,&str)>) {
    let mut it = lines.lines();
    let instructions: Vec<bool> = it.next().unwrap().trim().chars().map(|c| c=='L').collect();
    it.next();

    let mut nodes: HashMap<String, (&str,&str)> = HashMap::new();
    it.for_each(|line| {
        let kv = line.split_once(" = ").unwrap();
        let key = kv.0.to_string();
        let val = (&kv.1[1..4], &kv.1[6..9]);
        nodes.insert(key,val);
    });
    (instructions, nodes)
}
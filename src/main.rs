const WIDTH: usize = 60;
const HEIGHT: usize = 30;

fn main() {
    let mut map: Vec<Vec<char>> = vec![vec![' '; WIDTH]; HEIGHT];
    
    map[9][10] = '$';
    map[10][11] = '$';
    map[10][9] = '$';
    map[11][10] = '$';
    map[10][10] = '$';
    print_vec(map.clone());
    for _ in 0..90000 {
        update_map(&mut map);
        print_vec(map.clone());
    }
}

fn print_vec(vec: Vec<Vec<char>>) {
    vec.iter().for_each(|it| {
        it.iter().for_each(|x| {
            print!("{}", x);
        });
        println!();
    });
    
}
/*
    Any live cell with two or three live neighbours survives.
    Any dead cell with three live neighbours becomes a live cell.
    All other live cells die in the next generation. Similarly, all other dead cells stay dead.
*/
fn update_map(vec: &mut Vec<Vec<char>>) {
    
    let sizey = vec.len();
    let sizex = vec[0].len();

    let mut new_vec: Vec<Vec<char>> = vec![vec![' '; sizex]; sizey];

    for y in 1..sizey - 1 {
        for x in 1..sizex - 1 {
            let mut count = 0;

            for i in 0..3 {
                for j in 0..3 {
                    if vec[y + i - 1][x + j - 1] == '$' {
                        count += 1;
                    }
                }
            }
            if vec[y][x] == '$' {
                if count == 3 || count == 4 {
                    new_vec[y][x] = '$';
                } else {
                    new_vec[y][x] = ' ';
                }
            } else {
                if count == 3 {
                    new_vec[y][x] = '$';
                } else {
                    new_vec[y][x] = ' ';
                }
            }
            
        }
    }

    for y in 1..sizey - 1 {
        for x in 1..sizex - 1 {
            vec[y][x] = new_vec[y][x];
        }
    }
}

// Grid width and height

const N: usize = 5;

fn main() {
    /*
     * Warm up: plot the hexagons
     *
     *     >---<4       1>
     *    /5   0\       /
     *   /       \3   2/
     *  <4   +   1>----
     *   \       /5   0\
     *    \3   2/       \
     *     >---<4   +   1>
     *    /5 | 0\       /
     *   / \ | / \3   2/
     *  <4   +   1>---<
     *   \ / | \ /5   0\
     *    \3 | 2/       \
     *     >---<4   +   1>
     *    /5   0\       /
     *   /       \3   2/
     *  <4   +   1>---<
     *   \       /5   0\
     *    \3   2/       \
     *     >---<4   +   1>
     */

    let mut hex = [[[' '; 6]; N]; N];

    // Highlight a hat
    hex[2][2][1] = '*';
    hex[2][2][2] = '*';
    hex[3][2][0] = '*';
    hex[3][2][1] = '*';
    hex[3][2][4] = '*';
    hex[3][2][5] = '*';
    hex[2][3][3] = '*';
    hex[2][3][4] = '*';

    for y in 1..N {
        for yy in 0..6usize {
            // 0.. -> 3,2,1,0,1,2,
            let spaces = if yy <= 3 { 3 - yy } else { yy - 3 };
            for _ in 0..spaces {
                print!(" ");
            }

            for x in 0..N {
                let e = if x % 2 == 0 { hex[y][x] } else { hex[y - 1][x] };

                match (yy + (x % 2) * 3) % 6 {
                    0 => print!(">---"),
                    1 => print!("/{} | {}", e[5], e[0]),
                    2 => print!("/ \\ | / "),
                    3 => print!("<{}   +   {}", e[4], e[1]),
                    4 => print!("\\ / | \\ "),
                    5 => print!("\\{} | {}", e[3], e[2]),
                    x => panic!("{x} should have been impossible!"),
                }
            }
            println!();
        }
    }
}

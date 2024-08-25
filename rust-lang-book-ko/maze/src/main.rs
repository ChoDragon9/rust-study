use rand::Rng;

// const 상수명: 타입 = 값
const MAP_N: usize = 25;

fn main() {
    let mut rng = rand::thread_rng();
    // let mut 변수명 = [[초깃값; 배열 길이]; 배열 길이];
    let mut maze = [[0; MAP_N]; MAP_N];

    for n in 0..MAP_N {
        // 상단 벽 만들기
        maze[0][n] = 1;
        // 하단 벽 만들기
        maze[MAP_N - 1][n] = 1;
        // 오른쪽 벽 만들기
        maze[n][MAP_N - 1] = 1;
        // 왼쪽 벽 만들기
        maze[n][0] = 1;
    }

    for y in 2..MAP_N - 2 {
        for x in 2..MAP_N - 2 {
            if x % 2 == 1 || y % 2 == 1 {
                continue;
            }
            maze[x][y] = 1;

            // 상하좌우 중 어느 하나를 벽으로 만들기
            let r = rng.gen_range(0..=3);
            match r {
                // 상
                0 => maze[y - 1][x] = 1,
                // 하
                1 => maze[y + 1][x] = 1,
                // 좌
                2 => maze[y][x - 1] = 1,
                // 우
                3 => maze[y][x + 1] = 1,
                _ => {}
            }
        }
    }

    // 미로 출력
    let tiles = ["⬜", "🟫"];
    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}", tiles[maze[y][x]]);
        }
        println!("");
    }
}

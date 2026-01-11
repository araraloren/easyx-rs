use easyx::prelude::*;
use easyx::run;
use std::time::Duration;

// 游戏常量定义
const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;
const BLOCK_SIZE: usize = 25;
const GAME_WIDTH: usize = GRID_WIDTH * BLOCK_SIZE;
const GAME_HEIGHT: usize = GRID_HEIGHT * BLOCK_SIZE;
const PREVIEW_X: usize = GAME_WIDTH + 50;
const PREVIEW_Y: usize = 50;
const SCORE_X: usize = GAME_WIDTH + 50;
const SCORE_Y: usize = 200;

// 方向枚举
#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Down,
}

// 方块形状枚举
#[derive(Clone, Copy, Debug)]
enum Tetromino {
    I,
    O,
    T,
    L,
    J,
    S,
    Z,
}

// 方块结构体
#[derive(Debug)]
struct Block {
    shape: Tetromino,
    x: i32,
    y: i32,
    rotation: usize,
}

// 游戏状态结构体
#[derive(Debug)]
struct GameState {
    grid: [[Option<Color>; GRID_WIDTH]; GRID_HEIGHT],
    current_block: Block,
    next_block: Tetromino,
    score: u32,
    game_over: bool,
    last_drop_time: std::time::Instant,
    drop_interval: Duration,
}

impl Tetromino {
    // 获取方块形状数据
    fn shapes(&self) -> &'static [[bool; 4]; 4] {
        match self {
            Tetromino::I => &[
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
                [false, false, false, false],
            ],
            Tetromino::O => &[
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            Tetromino::T => &[
                [false, false, false, false],
                [false, true, false, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
            Tetromino::L => &[
                [false, false, false, false],
                [false, false, true, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
            Tetromino::J => &[
                [false, false, false, false],
                [true, false, false, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
            Tetromino::S => &[
                [false, false, false, false],
                [false, true, true, false],
                [true, true, false, false],
                [false, false, false, false],
            ],
            Tetromino::Z => &[
                [false, false, false, false],
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
        }
    }

    // 获取方块颜色
    fn color(&self) -> Color {
        match self {
            Tetromino::I => Color::CYAN,
            Tetromino::O => Color::YELLOW,
            Tetromino::T => Color::MAGENTA,
            Tetromino::L => Color::new(255, 165, 0), // ORANGE
            Tetromino::J => Color::BLUE,
            Tetromino::S => Color::GREEN,
            Tetromino::Z => Color::RED,
        }
    }

    // 随机生成方块
    fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();

        match rng.random_range(0..7) {
            0 => Tetromino::I,
            1 => Tetromino::O,
            2 => Tetromino::T,
            3 => Tetromino::L,
            4 => Tetromino::J,
            5 => Tetromino::S,
            6 => Tetromino::Z,
            _ => unreachable!(),
        }
    }
}

impl Block {
    // 计算旋转后的坐标
    fn rotate_coords(&self, x: usize, y: usize) -> (usize, usize) {
        match self.rotation {
            0 => (x, y),         // 0度旋转
            1 => (3 - y, x),     // 90度顺时针旋转
            2 => (3 - x, 3 - y), // 180度旋转
            3 => (y, 3 - x),     // 270度顺时针旋转
            _ => unreachable!(),
        }
    }

    // 旋转方块
    fn rotate(&mut self, direction: Direction, grid: &[[Option<Color>; GRID_WIDTH]; GRID_HEIGHT]) {
        let prev_r = self.rotation;
        let prev_x = self.x;

        // 尝试常规旋转
        match direction {
            Direction::Left => self.rotation = (self.rotation + 3) % 4,
            Direction::Right => self.rotation = (self.rotation + 1) % 4,
            _ => return,
        }

        let mut success = false;

        for adjust in 0..4 {
            self.x = prev_x + adjust;

            if !self.check_collision(grid) {
                success = true;
                break;
            }

            self.x = prev_x - adjust;

            if !self.check_collision(grid) {
                success = true;
                break;
            }
        }

        if !success {
            self.x = prev_x;
            self.rotation = prev_r;
        }
    }

    // 检查碰撞
    fn check_collision(&self, grid: &[[Option<Color>; GRID_WIDTH]; GRID_HEIGHT]) -> bool {
        let shape = self.shape.shapes();

        // 遍历旋转后的形状的每个位置
        for (rot_y, line) in shape.iter().enumerate() {
            for (rot_x, cell) in line.iter().enumerate() {
                // 只有当原始形状中该位置有方块时，才检查碰撞
                if *cell {
                    let (x, y) = self.rotate_coords(rot_x, rot_y);

                    // 计算在网格中的实际位置
                    let grid_x = self.x + x as i32;
                    let grid_y = self.y + y as i32;

                    // 检查碰撞条件：
                    // 1. 超出左边界 (grid_x < 0)
                    // 2. 超出右边界 (grid_x >= GRID_WIDTH as i32)
                    // 3. 超出下边界 (grid_y >= GRID_HEIGHT as i32)
                    // 4. 与已有方块碰撞 (grid[grid_y][grid_x].is_some())
                    if grid_x < 0
                        || grid_x >= GRID_WIDTH as i32
                        || grid_y >= GRID_HEIGHT as i32
                        || (grid_y >= 0 && grid[grid_y as usize][grid_x as usize].is_some())
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    // 移动方块
    fn move_block(
        &mut self,
        direction: Direction,
        grid: &[[Option<Color>; GRID_WIDTH]; GRID_HEIGHT],
    ) -> bool {
        let old_x = self.x;
        let old_y = self.y;

        match direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
        }

        if self.check_collision(grid) {
            self.x = old_x;
            self.y = old_y;
            return false;
        }
        true
    }
}

impl GameState {
    // 创建新游戏状态
    fn new() -> Self {
        GameState {
            grid: [[None; GRID_WIDTH]; GRID_HEIGHT],
            current_block: Block {
                shape: Tetromino::random(),
                x: (GRID_WIDTH / 2 - 2) as i32,
                y: 0,
                rotation: 0,
            },
            next_block: Tetromino::random(),
            score: 0,
            game_over: false,
            last_drop_time: std::time::Instant::now(),
            drop_interval: Duration::from_millis(1000),
        }
    }

    // 生成新方块
    fn spawn_new_block(&mut self) -> bool {
        self.current_block = Block {
            shape: self.next_block,
            x: (GRID_WIDTH / 2 - 2) as i32,
            y: 0,
            rotation: 0,
        };
        self.next_block = Tetromino::random();

        // 检查新方块是否能生成
        if self.current_block.check_collision(&self.grid) {
            self.game_over = true;
            return false;
        }
        true
    }

    // 固定方块到网格
    fn lock_block(&mut self) {
        let shape = self.current_block.shape.shapes();
        let block = &self.current_block;

        for (y, row) in shape.iter().enumerate().take(4) {
            for (x, &cell) in row.iter().enumerate().take(4) {
                if cell {
                    // 使用旋转坐标计算方法
                    let (rot_x, rot_y) = block.rotate_coords(x, y);

                    let grid_x = block.x + rot_x as i32;
                    let grid_y = block.y + rot_y as i32;

                    // 只锁定网格范围内的方块
                    if grid_x >= 0
                        && grid_x < GRID_WIDTH as i32
                        && grid_y >= 0
                        && grid_y < GRID_HEIGHT as i32
                    {
                        let grid_x_usize = grid_x as usize;
                        let grid_y_usize = grid_y as usize;
                        self.grid[grid_y_usize][grid_x_usize] = Some(block.shape.color());
                    }
                }
            }
        }
    }

    // 消除填满的行
    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;
        let mut new_grid = [[None; GRID_WIDTH]; GRID_HEIGHT];
        let mut new_y = GRID_HEIGHT - 1;

        for y in (0..GRID_HEIGHT).rev() {
            let line_full = self.grid[y].iter().all(|cell| cell.is_some());
            if !line_full {
                new_grid[new_y] = self.grid[y];
                new_y = new_y.saturating_sub(1);
            } else {
                lines_cleared += 1;
            }
        }

        self.grid = new_grid;
        self.score += lines_cleared * 100;

        // 加速游戏
        if lines_cleared > 0 {
            self.drop_interval = std::cmp::max(
                Duration::from_millis(100),
                self.drop_interval - Duration::from_millis(50),
            );
        }
    }

    // 更新游戏状态
    fn update(&mut self) {
        if self.game_over {
            return;
        }

        let now = std::time::Instant::now();
        if now - self.last_drop_time > self.drop_interval {
            if !self.current_block.move_block(Direction::Down, &self.grid) {
                self.lock_block();
                self.clear_lines();
                self.spawn_new_block();
            }
            self.last_drop_time = now;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化游戏状态
    let mut game = GameState::new();
    let mut end_game = false;

    run(800, 600, move |app| {
        // 开始批处理绘图（启用双缓冲）
        app.begin_batch_draw();

        while !end_game {
            // 清除屏幕以防止拖影
            app.clear_device();

            // 使用peek_message处理键盘输入
            while let Some(msg) = app.peek_message(MessageFilter::KeyBoard, true) {
                // 只处理按键按下事件，忽略释放事件
                if msg.ty == ExMessageType::KeyDown
                    && let Message::KeyBoard { vkcode, .. } = msg.msg
                {
                    match vkcode {
                        KeyCode::Left => {
                            game.current_block.move_block(Direction::Left, &game.grid);
                        }
                        KeyCode::Right => {
                            game.current_block.move_block(Direction::Right, &game.grid);
                        }
                        KeyCode::Down => {
                            game.current_block.move_block(Direction::Down, &game.grid);
                        }
                        KeyCode::Up => {
                            game.current_block.rotate(Direction::Right, &game.grid);
                        }
                        KeyCode::Escape => {
                            end_game = true;
                        }
                        KeyCode::Space => {
                            // 快速下落
                            while game.current_block.move_block(Direction::Down, &game.grid) {
                                // 空循环，直到无法下落
                            }
                        }
                        _ => {}
                    }
                }
            }

            // 更新游戏状态
            game.update();

            // 绘制游戏标题区域
            app.set_fillcolor(&Color::BLUE);
            app.fill_rectangle(GAME_WIDTH as i32, 0, 800, 50);
            app.set_textcolor(&Color::WHITE);
            app.set_textstyle(24, 0, "Arial");
            app.out_text(GAME_WIDTH as i32 + 10, 10, "俄罗斯方块");

            // 绘制游戏边界
            app.rectangle(0, 0, GAME_WIDTH as i32, GAME_HEIGHT as i32);

            // 绘制已固定的方块
            for y in 0..GRID_HEIGHT {
                for x in 0..GRID_WIDTH {
                    if let Some(color) = game.grid[y][x] {
                        app.set_fillcolor(&color);
                        app.fill_rectangle(
                            x as i32 * BLOCK_SIZE as i32 + 1,
                            y as i32 * BLOCK_SIZE as i32 + 1,
                            (x + 1) as i32 * BLOCK_SIZE as i32 - 1,
                            (y + 1) as i32 * BLOCK_SIZE as i32 - 1,
                        );
                    }
                }
            }

            // 绘制当前方块
            let shape = game.current_block.shape.shapes();
            for (y, row) in shape.iter().enumerate().take(4) {
                for (x, &cell) in row.iter().enumerate().take(4) {
                    if cell {
                        // 计算旋转后的实际坐标
                        let (rot_x, rot_y) = game.current_block.rotate_coords(x, y);

                        let block_x = game.current_block.x + rot_x as i32;
                        let block_y = game.current_block.y + rot_y as i32;

                        app.set_fillcolor(&game.current_block.shape.color());
                        app.fill_rectangle(
                            block_x * BLOCK_SIZE as i32 + 1,
                            block_y * BLOCK_SIZE as i32 + 1,
                            (block_x + 1) * BLOCK_SIZE as i32 - 1,
                            (block_y + 1) * BLOCK_SIZE as i32 - 1,
                        );
                    }
                }
            }

            // 绘制下一个方块预览
            let next_shape = game.next_block.shapes();
            app.rectangle(
                PREVIEW_X as i32,
                PREVIEW_Y as i32,
                PREVIEW_X as i32 + 100,
                PREVIEW_Y as i32 + 100,
            );
            for (y, row) in next_shape.iter().enumerate().take(4) {
                for (x, &cell) in row.iter().enumerate().take(4) {
                    if cell {
                        app.set_fillcolor(&game.next_block.color());
                        app.fill_rectangle(
                            (PREVIEW_X + x * BLOCK_SIZE) as i32 + 1,
                            (PREVIEW_Y + y * BLOCK_SIZE) as i32 + 1,
                            (PREVIEW_X + (x + 1) * BLOCK_SIZE) as i32 - 1,
                            (PREVIEW_Y + (y + 1) * BLOCK_SIZE) as i32 - 1,
                        );
                    }
                }
            }

            // 绘制分数
            app.set_textcolor(&Color::WHITE);
            app.set_textstyle(20, 0, "Arial");
            app.out_text(
                SCORE_X as i32,
                SCORE_Y as i32,
                &format!("Score: {}", game.score),
            );

            // 绘制游戏结束标志
            if game.game_over {
                // 绘制红色背景
                app.set_fillcolor(&Color::new(255, 0, 0));
                app.fill_rectangle(
                    GAME_WIDTH as i32 / 2 - 100,
                    GAME_HEIGHT as i32 / 2 - 50,
                    GAME_WIDTH as i32 / 2 + 100,
                    GAME_HEIGHT as i32 / 2 + 50,
                );
                // 绘制游戏结束文本
                app.set_textcolor(&Color::WHITE);
                app.set_textstyle(30, 0, "Arial");
                app.out_text(
                    GAME_WIDTH as i32 / 2 - 80,
                    GAME_HEIGHT as i32 / 2 - 20,
                    "游戏结束",
                );
            }

            // 将缓冲区内容一次性刷新到屏幕（双缓冲）
            app.flush_batch_draw();
        }

        // 结束批处理绘图
        app.end_batch_draw();

        Ok(())
    })
}

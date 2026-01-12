use easyx::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    easyx::run_flags(800, 600, InitFlags::NoClose, |cx| {
        let bkcolor = Color::DARKGRAY;
        let mut mouse_text = String::default();
        let mut keyboard_text = String::default();
        let mut stop = false;

        cx.begin_batch_draw();

        cx.peek_message(MessageFilter::All, false);

        while !stop {
            // 清理整个设备
            cx.clear_device();

            while let Some(msg) = cx.peek_message(MessageFilter::All, true) {
                if msg.ty == ExMessageType::KeyUp {
                    if let Message::KeyBoard {
                        vkcode, prevdown, ..
                    } = msg.msg
                    {
                        if vkcode == KeyCode::Escape && prevdown {
                            // ESC key
                            stop = true;
                            break;
                        }
                        keyboard_text = format!("Key pressed: VK Code: {:?}", vkcode);
                    }
                } else if let Message::Mouse {
                    x: pos_x, y: pos_y, ..
                } = msg.msg
                {
                    mouse_text = format!("Mouse position: X: {}, Y: {}", pos_x, pos_y);
                }
            }

            // 清楚整个屏幕，填充背景色
            cx.set_fillcolor(&bkcolor);
            cx.fill_rectangle(0, 0, cx.width(), cx.height());

            // 设置字体
            cx.set_textstyle(30, 0, "Arial");

            let render_text = |text: &str, x: i32, y: i32, w: i32, color: Color| -> i32 {
                let (width, height) = (cx.text_width(text), cx.text_height(text));

                assert!(w >= width);

                cx.set_linecolor(&Color::BLACK);
                cx.rectangle(x - 1, y - 1, x + w + 1, y + height + 1);
                cx.set_bkcolor(&bkcolor);
                cx.set_textcolor(&color);
                cx.out_text(x, y, text);
                y + height
            };

            // 绘制键盘按键信息
            if !keyboard_text.is_empty() {
                render_text(&keyboard_text, 10, 90, 600, Color::LIGHTBLUE);
            }

            // 绘制鼠标的坐标信息
            if !mouse_text.is_empty() {
                render_text(&mouse_text, 10, 130, 600, Color::CYAN);
            }

            let render_text_boxed = |text: &str, x: i32, y: i32, color: Color| {
                let (width, height) = (cx.text_width(text), cx.text_height(text));

                cx.set_linecolor(&Color::WHITE);
                cx.rectangle(x - 1, y - 1, x + width + 1, y + height + 1);
                cx.set_bkcolor(&bkcolor);
                cx.set_textcolor(&color);
                cx.out_text(x, y, text);
            };

            // 绘制文本
            render_text_boxed(
                "Press any key or move mouse to test message handling...",
                10,
                10,
                Color::BLUE,
            );

            // 绘制文本
            render_text_boxed("Press Esc exit", 10, 50, Color::MAGENTA);

            cx.flush_batch_draw();
        }

        cx.end_batch_draw();

        Ok(())
    })
}

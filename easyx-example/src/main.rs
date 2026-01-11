use easyx_rs::prelude::*;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    easyx_rs::run_flags(800, 600, InitFlags::NoClose, |cx| {
        let prompt_y = 10;

        // 设置白色背景和黑色文字
        cx.set_textstyle(30, 0, "微软雅黑");

        cx.set_textcolor(&Color::RED);

        let prompt = "Press any key or move mouse to test message handling...";
        let esc = "按 ESC 退出";

        cx.out_text(10, prompt_y, prompt);
        cx.out_text(10, prompt_y + 30, esc);
        let mouse_start = prompt_y + 30 + cx.text_height(esc) + 5;
        let mut key_start = mouse_start;
        let mut mouse_last_x = -1;
        let mut key_last_x = -1;

        loop {
            // 非阻塞获取消息

            if let Some(msg) = cx.peek_message(MessageFilter::Mouse, true)
                && let Message::Mouse { x, y, .. } = msg.msg
            {
                let text = format!("Mouse position: X: {}, Y: {}", x, y);
                let width = cx.text_width(&text);
                let height = cx.text_height(&text);

                // 使用矩形填充整个窗口作为白色背景
                if mouse_last_x == -1 {
                    mouse_last_x = 10 + width;
                }
                cx.set_textcolor(&Color::RED);
                cx.set_fillcolor(&Color::BLACK);
                cx.fill_rectangle(5, mouse_start, mouse_last_x, mouse_start + 10 + height);
                cx.out_text(10, mouse_start + 5, text.as_str());
                key_start = mouse_start + 10 + height + 5;
                mouse_last_x = 10 + width;
            }

            if let Some(msg) = cx.peek_message(MessageFilter::KeyBoard, true)
                && let Message::KeyBoard { vkcode, .. } = msg.msg
            {
                if vkcode == KeyCode::Escape {
                    // ESC key
                    break;
                }
                // 使用矩形填充整个窗口作为白色背景
                let text = format!("Key pressed: VK Code: {:?}", vkcode);
                let width = cx.text_width(&text);
                let height = cx.text_height(&text);

                // 使用矩形填充整个窗口作为白色背景
                if key_last_x == -1 {
                    key_last_x = 10 + width;
                }
                cx.set_textcolor(&Color::RED);
                cx.set_fillcolor(&Color::BLACK);
                cx.fill_rectangle(5, key_start, key_last_x, key_start + 10 + height);
                cx.out_text(10, key_start + 5, text.as_str());
                key_last_x = 10 + width;
            }

            // 使用标准库的sleep函数代替app.delay
            sleep(Duration::from_millis(20));
        }

        Ok(())
    })
}

use gtk::{Builder, Container, Paned, TextTag, TextView};
use gtk::gdk::EventMask;
use gtk::glib::Propagation;
use gtk::prelude::{BuilderExtManual, Cast, PanedExt, TextBufferExt, TextTagTableExt, TextViewExt, WidgetExt, WidgetExtManual};
use crate::ui::fragment::inter::fragment::Fragment;

#[derive(Clone)]
pub struct SidebarFragment {
    root: Option<Container>
}

impl SidebarFragment {

    pub fn new() -> Self {
        Self {
            root: None
        }
    }
}

impl Fragment for SidebarFragment {

    fn on_create(&mut self) -> &Container {
        let builder = Builder::from_file("res/ui/gtk3/sidebar-fragment.ui");

        self.root = Some(builder
            .object("sidebar_layout")
            .expect("Couldn't find 'sidebar_layout' in window.ui"));


        let hex_data = vec![
            0x3c, 0x52, 0xa1, 0x12, 0xa4, 0x50, 0x1c, 0xce,
            0x51, 0x34, 0x00, 0x9f, 0x08, 0x00, 0x45, 0x00,
            0x00, 0x3f, 0xe5, 0xa4, 0x40, 0x00, 0x40, 0x11,
            0x79, 0x9d, 0xc0, 0xa8, 0x00, 0x81, 0x34, 0x60,
            0xe5, 0xe2, 0xb8, 0x04, 0x01, 0xbb, 0x00, 0x2b,
            0xd1, 0xd5, 0x44, 0x39, 0x34, 0x60, 0xe5, 0xe2,
            0x52, 0x60, 0x99, 0x2f, 0x68, 0xd0, 0x12, 0x6d,
            0xe8, 0x37, 0x2b, 0x67, 0x1a, 0xa5, 0x2b, 0x0c,
            0xf6, 0x3b, 0x5e, 0xfa, 0x74, 0x80, 0xbc, 0x29,
            0xd8, 0x37, 0xbf, 0xe3, 0x5c,
        ];


        let line_numbers: TextView = builder.object("hex_line_numbers").unwrap();
        let hex_text_view: TextView = builder.object("hex_text_view").unwrap();
        let ascii_text_view: TextView = builder.object("ascii_text_view").unwrap();

        let line_numbers_string = hex_data.chunks(16)
            .enumerate()
            .map(|(i, _)| format!("{:08X}", i * 16))  // Format line numbers in hex
            .collect::<Vec<_>>()
            .join("\n");
        line_numbers.buffer().unwrap().set_text(&line_numbers_string);

        let hex_string = hex_data.chunks(16)
            .map(|chunk| chunk.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n");
        hex_text_view.buffer().unwrap().set_text(&hex_string);







        /*
        let hex_ascii_string = hex_data.chunks(16)
            .map(|chunk| {
                let hex_part = chunk.iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ");

                let ascii_part = chunk.iter()
                    .map(|&b| {
                        // Check if byte is a printable ASCII character (0x20 to 0x7E)
                        if (b >= 0x20 && b <= 0x7E) {
                            char::from_u32(b as u32).unwrap_or('.')
                        } else {
                            '.' // Non-printable characters replaced with '.'
                        }
                    })
                    .collect::<String>();

                format!("{: <47} {}", hex_part, ascii_part)
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Insert Text
        ascii_text_view.buffer().unwrap().set_text(&ascii_string);
        */


        let ascii_string = hex_data.chunks(16)
            .map(|chunk| {
                chunk.iter()
                    .map(|&b| {
                        // Check if byte is a printable ASCII character (0x20 to 0x7E)
                        if (b >= 0x20 && b <= 0x7E) {
                            // Convert byte to char using `char::from_u32()`
                            char::from_u32(b as u32).unwrap_or('.') // Fall back to '.' if invalid
                        } else {
                            '.' // Non-printable characters replaced with '.'
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        ascii_text_view.buffer().unwrap().set_text(&ascii_string);

        // Create Tags
        let buffer = ascii_text_view.buffer().unwrap();
        let tag_table = buffer.tag_table().unwrap();

        /*
        let layer_1 = TextTag::builder().name("layer_1").background("#3f5222").build();
        let layer_2 = TextTag::builder().name("layer_2").background("#1c314a").build();
        let layer_3 = TextTag::builder().name("layer_3").background("#070c1f").build();
        tag_table.add(&layer_1);
        tag_table.add(&layer_2);
        tag_table.add(&layer_3);

        let start_iter = buffer.start_iter();
        let mut end_iter = start_iter.clone();
        end_iter.forward_chars(14);
        buffer.apply_tag(&layer_1, &start_iter, &end_iter);

        let start_iter = end_iter;
        //start_iter.forward_chars(14);
        let mut end_iter = start_iter.clone();
        end_iter.forward_chars(20);
        buffer.apply_tag(&layer_2, &start_iter, &end_iter);

        let start_iter = end_iter;
        let mut end_iter = start_iter.clone();
        end_iter.forward_chars(8);
        buffer.apply_tag(&layer_3, &start_iter, &end_iter);
        */



        /*
        //let hover_tag = layer_1;
        let hover_start = buffer.iter_at_offset(0); // "Special"
        let mut hover_end = buffer.iter_at_offset(14); // "words"

        ascii_text_view.connect_motion_notify_event(move |text_view, event| {
            let (mouse_x, mouse_y) = event.position();

            // Get text offset from coordinates
            let buffer = text_view.buffer().unwrap();
            if let Some(iter) = text_view.iter_at_location(mouse_x as i32, mouse_y as i32) {
                // Remove previous highlight
                buffer.remove_all_tags(&hover_start, &hover_end);

                // Check if the mouse is inside the word range
                if iter.offset() >= hover_start.offset() && iter.offset() <= hover_end.offset() {

                    let layer_1 = TextTag::builder().name("layer_1").background("#3f5222").build();
                    buffer.apply_tag(&layer_1, &hover_start, &hover_end);
                }
            }

            Propagation::Proceed
        });
        */


        // Enable Mouse Motion Events
        ascii_text_view.set_events(EventMask::POINTER_MOTION_MASK);

        // Create Hover Tag (Initially Invisible)
        let hover_tag = TextTag::builder()
            .name("hover_char")
            .background("#59436e") // Highlight with yellow background
            .build();
        buffer.tag_table().unwrap().add(&hover_tag);

        // Track Previously Hovered Character
        let previous_char_offset = std::rc::Rc::new(std::cell::Cell::new(None));

        // Connect Mouse Hover Event
        ascii_text_view.connect_motion_notify_event({
            let previous_char_offset = previous_char_offset.clone();
            move |text_view, event| {
                let (mouse_x, mouse_y) = event.position();

                let mouse_x = mouse_x-10 as f64;
                let mouse_y = mouse_y-10 as f64;

                let buffer = text_view.buffer().unwrap();

                if let Some(iter) = text_view.iter_at_location(mouse_x as i32, mouse_y as i32) {
                    let char_offset = iter.offset();

                    // If we're still hovering the same character, do nothing
                    if previous_char_offset.get() == Some(char_offset) {
                        return Propagation::Proceed;
                    }

                    // Remove the tag from the previously highlighted character
                    if let Some(prev_offset) = previous_char_offset.get() {
                        let prev_iter = buffer.iter_at_offset(prev_offset);
                        let mut next_iter = prev_iter.clone();
                        next_iter.forward_char();
                        buffer.remove_tag(&hover_tag, &prev_iter, &next_iter);
                    }

                    // Apply the tag to the new character
                    let mut next_iter = iter.clone();
                    next_iter.forward_char(); // Move one char forward
                    buffer.apply_tag(&hover_tag, &iter, &next_iter);

                    // Update the previously hovered character
                    previous_char_offset.set(Some(char_offset));
                }

                Propagation::Proceed
            }
        });

        &self.root.as_ref().unwrap().upcast_ref()
    }

    fn on_resume(&self) {
        todo!()
    }

    fn on_pause(&self) {
        todo!()
    }

    fn on_destroy(&self) {
        todo!()
    }

    fn dyn_clone(&self) -> Box<dyn Fragment> {
        Box::new(self.clone())
    }
}

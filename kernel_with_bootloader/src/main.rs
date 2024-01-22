#![no_std]
#![no_main]
mod writer;
mod printt;
use bootloader_api::config::Mapping;
use writer::FrameBufferWriter;
use x86_64::instructions::hlt;
//Use the entry_point macro to register the entry point function:
// bootloader_api::entry_point!(kernel_main)
//optionally pass a custom config

pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();
    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    let mut frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);
    use core::fmt::Write; //below requires this

    #[macro_export]
    macro_rules! print {
        ($($body: expr), *) => {
            write!(frame_buffer_writer, $($body), *).unwrap();
        };
    }

    #[macro_export]
    macro_rules! println {
        ($($body: expr), *) => {
            writeln!(frame_buffer_writer, $($body), *).unwrap();
        };
    }

    writeln!(
        frame_buffer_writer,
        "Testing writeln! macro"
    )
    .unwrap();

    print!("Testing print! macro\n");
    println!("Testing println! macro");
    
    print!("Testing print! with multiple arguments: {}, {}, {}, {}\n", 4, true, "string slice", 'c');
    println!("Testing println! with multiple arguments: {}, {}, {}, {}", 4, true, "string slice", 'c');

    frame_buffer_writer.set_screen_write_position(150, 200);
    print!("Testing dynamic setting of screen write position with print!\n");

    frame_buffer_writer.set_screen_write_position(300, 400);
    println!("Testing dynamic setting of screen write position with println!");


    loop {
        hlt(); //stop x86_64 from being unnecessarily busy while looping
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}

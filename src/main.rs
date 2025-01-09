use umacro_lib::{create_virtual_device, Key};

fn main() -> umacro_lib::Result<()> {
    let mut device = create_virtual_device()?;

    println!("Waiting 2s for keyboard initialization...");

    device.wait(2000); // Wait 2 seconds to initialize keyboard

    device.key(Key::NumLock)?;

    println!("Numlock pressed.");

    Ok(())
}

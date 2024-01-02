use std::thread::sleep;
use std::time::Duration;
use esp_idf_svc::hal::{delay};
use esp_idf_svc::hal::peripherals::Peripherals;
use display_interface_parallel_gpio;
use display_interface_parallel_gpio::{Generic8BitBus, PGPIO8BitInterface};
use esp_idf_svc::hal::gpio::PinDriver;
use mipidsi::{ColorInversion};
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::primitives::{Primitive, PrimitiveStyle, Rectangle};
use rand::Rng;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    // setup pins for ttgo display
    let pins = peripherals.pins;
    let mut backlight = PinDriver::output(pins.gpio38).unwrap();
    let dc = PinDriver::output(pins.gpio7).unwrap();
    let mut cs = PinDriver::output(pins.gpio6).unwrap();
    let rst = PinDriver::output(pins.gpio5).unwrap();
    let mut wr = PinDriver::output(pins.gpio8).unwrap();
    let mut rd = PinDriver::output(pins.gpio9).unwrap();

    let d0 = PinDriver::output(pins.gpio39).unwrap();
    let d1 = PinDriver::output(pins.gpio40).unwrap();
    let d2 = PinDriver::output(pins.gpio41).unwrap();
    let d3 = PinDriver::output(pins.gpio42).unwrap();
    let d4 = PinDriver::output(pins.gpio45).unwrap();
    let d5 = PinDriver::output(pins.gpio46).unwrap();
    let d6 = PinDriver::output(pins.gpio47).unwrap();
    let d7 = PinDriver::output(pins.gpio48).unwrap();

    // let mut lcd_power_on = PinDriver::output(pins.gpio15).unwrap();

    // let mut backlight = gpio::PinDriver::output(backlight).unwrap();
    // backlight.set_high().unwrap();
    //
    // let mut lcd_power = gpio::PinDriver::output(lcd_power_on).unwrap();
    // lcd_power.set_high().unwrap();

    backlight.set_high().unwrap();

    // set to low to enable display
    cs.set_low().unwrap();

    // set to high when not in use
    rd.set_high().unwrap();
    wr.set_high().unwrap();

    let bus = Generic8BitBus::new((d0, d1, d2, d3, d4, d5, d6, d7)).unwrap();
    let di = PGPIO8BitInterface::new(bus, dc, wr);

    let mut delay_source: delay::Ets = delay::Ets;


    let mut display = mipidsi::Builder::st7789(di)
        .with_display_size(170, 320)
        .with_invert_colors(ColorInversion::Inverted)
        .init(&mut delay_source, Some(rst)).unwrap();

    display.set_orientation(mipidsi::options::Orientation::Portrait(false)).unwrap();
    for _ in 1..10 {
        log::info!("****************** Cycle *********");
        let _ = display.clear(Rgb565::RED);
        let _ = display.clear(Rgb565::GREEN);
        let _ = display.clear(Rgb565::BLUE);
    }

    log::info!("Hello, esp32 !");
    loop{
        let color: [u8; 3] = [rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()];
        let _ = rectangle_simple(&mut display, &color);
        sleep(Duration::from_nanos(1000));
    }
}

fn rectangle_simple<D>(display: &mut D, color: &[u8; 3]) -> Result<(), D::Error>
    where D: DrawTarget<Color=Rgb565>
{
    let size = Size::new(rand::thread_rng().gen_range(2..50), rand::thread_rng().gen_range(2..50));
    let start_point = Point::new(rand::thread_rng().gen_range(0..170) as i32, rand::thread_rng().gen_range(0..320) as i32);
    Rectangle::new(start_point, size)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::new(color[0], color[1], color[2])))
        .draw(display)?;
    Ok(())
}

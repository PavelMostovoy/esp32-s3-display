use esp_idf_svc::hal::{delay, gpio};
use esp_idf_svc::hal::peripherals::Peripherals;
use display_interface_parallel_gpio;
use display_interface_parallel_gpio::{Generic8BitBus, PGPIO8BitInterface};
use esp_idf_svc::hal::gpio::PinDriver;
use mipidsi::ColorInversion;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::draw_target::DrawTarget;

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
    let mut rst = PinDriver::output(pins.gpio5).unwrap();
    let wr = PinDriver::output(pins.gpio8).unwrap();
    let mut rd = PinDriver::output(pins.gpio9).unwrap();

    let mut d0 = PinDriver::output(pins.gpio39).unwrap();
    let mut d1 = PinDriver::output(pins.gpio40).unwrap();
    let mut d2 = PinDriver::output(pins.gpio41).unwrap();
    let mut d3 = PinDriver::output(pins.gpio42).unwrap();
    let mut d4 = PinDriver::output(pins.gpio45).unwrap();
    let mut d5 = PinDriver::output(pins.gpio46).unwrap();
    let mut d6 = PinDriver::output(pins.gpio47).unwrap();
    let mut d7 = PinDriver::output(pins.gpio48).unwrap();

    let lcd_power_on: gpio::Gpio15 = pins.gpio15;

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

    let bus = Generic8BitBus::new((d0, d1, d2, d3, d4, d5, d6, d7)).unwrap();
    let di = PGPIO8BitInterface::new(bus, dc, wr);
    // let mut display = mipidsi::Builder::st7789(di)
    //     .with_display_size(170, 320)
    //     .with_invert_colors(ColorInversion::Inverted)
    //     .init(&mut delay::Ets, Some(rst)).unwrap();


    let mut display = mipidsi::Builder::st7789(di)
        .with_display_size(170, 320)
        .with_invert_colors(ColorInversion::Inverted)
        .init(&mut delay::Ets, Some(rst)).unwrap();

    display.set_orientation(mipidsi::options::Orientation::Portrait(false)).unwrap();

    let _ = display.clear(Rgb565::RED);

    log::info!("Hello, esp32 !");
}

mod keys;

use std::error::Error;

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::{Characteristic, WriteType};
use btleplug::api::{
    bleuuid::BleUuid, Central, CentralEvent, Manager as _, ScanFilter, Peripheral as _,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;

use evdev_rs::Device;
use evdev_rs::InputEvent;
use evdev_rs::ReadFlag;
use evdev_rs::enums::EventCode;
use evdev_rs::enums::EventType;
use polling::Event;
use polling::Events;
use polling::Poller;

use keys::keys::KeyBuffer;

const DEVICE_PATH: &str = "/dev/input/event15";
const KEY: usize = 0;
const KBD_CHARACTERISTIC_UUID: uuid::Uuid = uuid_from_u16(0x1809);


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let esp = connect().await.expect("Could not find device");
    esp.discover_services().await?;

    let chars = esp.characteristics();
    let kbd_char = chars
        .iter()
        .find(|c| c.uuid == KBD_CHARACTERISTIC_UUID)
        .expect("ESPeyboard does not have correct characteristics");

    let mut kbd = Device::new_from_path(DEVICE_PATH).expect("failed to create device");
    kbd.grab(evdev_rs::GrabMode::Grab).expect("Could not grab");
    println!("input grabbed");

    let poller = Poller::new().unwrap();
    unsafe {
        poller
            .add(kbd.file(), Event::readable(KEY))
            .expect("Could not poll input");
    }

    let mut key_buffer = KeyBuffer::new();
    key_buffer.grabbed = true;

    // Polling event loop
    let mut events = Events::new();
    loop {
        events.clear();
        poller.wait(&mut events, None)?;

        for ev in events.iter() {
            if ev.key == KEY {
                // Handle event with evdev
                let ev = kbd.next_event(ReadFlag::NORMAL).map(|val| val.1);
                match ev {
                    Ok(ev) => match ev.event_type() {
                        None => (),
                        Some(EventType::EV_KEY) => {
                            handle_key_event(&mut kbd, &mut key_buffer, ev, &esp, kbd_char).await;
                        }
                        Some(_) => (),
                    },
                    Err(e) => eprintln!("{}", e),
                }

                // Set interest in the next readability event.
                poller.modify(kbd.file(), Event::readable(KEY))?;
            }
        }
    }
}

async fn handle_key_event(kbd: &mut Device, key_buffer: &mut KeyBuffer, ev: InputEvent, esp: &Peripheral, kbd_char: &Characteristic) {
    let k = if let EventCode::EV_KEY(k) = ev.event_code {
        k as i32
    } else {
        return;
    };

    if key_buffer.is_ungrab_pressed() {
        if key_buffer.grabbed {
            println!("Ungrabbing");
            let empty_keys = [0u8; 6];
            esp.write(kbd_char, empty_keys.as_slice(), WriteType::WithoutResponse).await.expect("Could not write to characteristic");
            kbd.grab(evdev_rs::GrabMode::Ungrab).unwrap();
        } else {
            println!("Grabbing");
            kbd.grab(evdev_rs::GrabMode::Grab).unwrap();
        }
        key_buffer.grabbed = !key_buffer.grabbed;
    }

    let is_changed = match ev.value {
        0 => key_buffer.release_key(k),
        1 | 2 => key_buffer.press_key(k),
        _ => false,
    };

    if is_changed {
        println!(
            "{}, {}",
            ev.event_code,
            match ev.value {
                1 => "pressed",
                2 => "repressed",
                0 => "released",
                _ => panic!("This should never happen, event value not 0, 1, or 2"),
            }
        );
        println!("{}", key_buffer);

        if key_buffer.grabbed {
            let hid_keys = key_buffer.to_hid();
            esp.write(kbd_char, hid_keys.as_slice(), WriteType::WithoutResponse).await.expect("Could not write to characteristic");
            // println!("{:?}", key_buffer.ungrab_shortcut);
        }
    }
}

async fn connect() -> Result<Peripheral, Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    let adapters = manager.adapters().await.unwrap();
    let central = adapters.into_iter().nth(0).ok_or("No adapters")?;

    let mut events = central.events().await?;
    central.start_scan(ScanFilter::default()).await?;

    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                let peripheral = central.peripheral(&id).await?;
                let properties = peripheral.properties().await?;
                let name = properties.and_then(|p| p.local_name);

                match name {
                    Some(name) => {
                        println!("DeviceDiscovered: {:?}, Name: {}", id, name);
                        if name == "ESPeyboard" {
                            println!("Connected to ESPeyboard!");
                            peripheral.connect().await?;
                            return Ok(peripheral);
                        }
                    }
                    None => println!("DeviceDiscovered: {:?}", id),
                }

            }
            _ => {}
        }
    } 
    return Err(Box::new(btleplug::Error::DeviceNotFound))
}

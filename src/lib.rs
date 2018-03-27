use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum TagError {
    UnknownManufacturerId,
    UnknownPacketSpecification
}

#[derive(Debug)]
struct Tag {
    id: u8,
    humidity: f64, 
    temperature: f64,
    pressure: u32,
    acceleration: Acceleration,
    battery_voltage: u16
}

#[derive(Debug)]
struct Acceleration {
    x: i16,
    y: i16,
    z: i16
}

impl Tag {
   pub fn new(data: HashMap<u16, Vec<u8>>) -> Result<Tag, TagError> {
       if !data.contains_key(&0x0499){
           return Err(TagError::UnknownManufacturerId);
       }
       
       let values = data.get(&0x0499).unwrap();
       let tag = Tag  {
        id: values[0],
        humidity: (values[1] / 2) as f64,
        
        //temperature: if ( values[2] & 0x80 == 0x80) { (-1 * (values[2] & 0x7f) as i8) as f64} else { (values[2] & 0x7f) as f64 } + ((values[3] as f64 * 0.01)),
        temperature: parse_temperature(values[2], values[3]),
        pressure: (((values[4] as u32) << 8) | values[5] as u32)  + 50000,
        acceleration: Acceleration{
            x: (((values[6] as i16) << 8) | values[7] as i16),
            y: (((values[8] as i16) << 8) | values[9] as i16),
            z: (((values[10] as i16) << 8) | values[11] as i16)
        },
        battery_voltage: (((values[12] as u16) << 8) | values[13] as u16)
       };
       Ok(tag)
   }

   
}
fn parse_temperature(t_msb: u8, t_lsb: u8) -> f64{
    let integer: u8 = 0x7F & t_msb;
    let decimal: f64 = t_lsb as f64 / 100f64; 
    if 0x80 & t_msb == 0x80{
        return -1.00 * (integer as f64 + decimal)
    }
    (integer as f64 + decimal)
}
#[test]
fn parse_packet() {
    let mut packet: HashMap<u16, Vec<u8>> = HashMap::new();
    packet.insert(1177, vec![3, 172, 5, 31, 192, 7, 2, 215, 2, 223, 255, 247, 11, 95]);
    assert_eq!(packet.len(), 1);
    let tag_data = Tag::new(packet).unwrap();
    assert_eq!(tag_data.id, 3);
    assert_eq!(tag_data.humidity, 86 as f64);
    assert_eq!(tag_data.temperature, 5.31 as f64);
    assert_eq!(tag_data.pressure, 99159);
    assert_eq!(tag_data.acceleration.x, 0x2d7);
    assert_eq!(tag_data.acceleration.y, 0x2df);
    assert_eq!(tag_data.acceleration.z, 0xfff7);
    assert_eq!(tag_data.battery_voltage, 2911);
        
} 

#[test]
fn invalid_manufacturer_id() {
    let mut packet: HashMap<u16, Vec<u8>> = HashMap::new();
    packet.insert(0x123, vec![3, 172, 5, 31, 192, 7, 2, 215, 2, 223, 255, 247, 11, 95]);
    assert_eq!(Tag::new(packet).is_err(), true);
    //TODO:
    //assert_eq!(Tag::new(packet), TagError::UnknownManufacturerId);
}

#[test]
fn minus_degrees(){
    let mut packet: HashMap<u16, Vec<u8>> = HashMap::new();
    packet.insert(1177, vec! [3, 111, 133, 94, 198, 212, 2, 197, 2, 224, 255, 255, 11, 95]);
    assert_eq!(Tag::new(packet).unwrap().temperature, -5.9399999999999995)
}

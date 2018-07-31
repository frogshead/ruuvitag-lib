#![allow(dead_code)]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;

use std::collections::HashMap;
use std::option::Option;

use reqwest::{Client, Url};


#[derive(Debug, PartialEq, Eq)]
pub enum TagError {
    UnknownManufacturerId,
    UnknownPacketSpecification,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Tag {
    pub manufacturer_id: u8,
    pub humidity: f64,
    pub temperature: f64,
    pub pressure: u32,
    pub acceleration: Acceleration,
    pub battery_voltage: u16,
    pub mac: Option<String>
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Acceleration {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl Tag {
    pub fn new(data: HashMap<u16, Vec<u8>>) -> Result<Tag, TagError> {
        if !data.contains_key(&0x0499) {
            return Err(TagError::UnknownManufacturerId);
        }

        let values = data.get(&0x0499).unwrap();
        let tag = Tag {
            manufacturer_id: values[0],
            humidity: (values[1] as f64 / 2f64) as f64,
            temperature: parse_temperature(values[2], values[3]),
            pressure: (((values[4] as u32) << 8) | values[5] as u32) + 50000,
            acceleration: Acceleration {
                x: (((values[6] as i16) << 8) | values[7] as i16),
                y: (((values[8] as i16) << 8) | values[9] as i16),
                z: (((values[10] as i16) << 8) | values[11] as i16),
            },
            battery_voltage: (((values[12] as u16) << 8) | values[13] as u16),
            mac: None
        };
        Ok(tag)
    }
    pub fn post_json(url: String, data: Tag){
        let j = self::serde_json::to_string(&data).unwrap();
        let u = Url::parse(&url).unwrap();
        let client = reqwest::Client::new();//  Client::new();
        let res = client.post(u) //"http://localhost:8080/mjsonrust"
            .json(&j)
            .send().unwrap();
        println!("{:?}", res);

    }

}
fn parse_temperature(t_msb: u8, t_lsb: u8) -> f64 {
    let integer: u8 = 0x7F & t_msb;
    let decimal: f64 = t_lsb as f64 / 100f64;
    if 0x80 & t_msb == 0x80 {
        return -1.00 * (integer as f64 + decimal);
    }
    (integer as f64 + decimal)
}



    #[test]
    fn parse_packet() {
        let mut packet: HashMap<u16, Vec<u8>> = HashMap::new();
        packet.insert(
            1177,vec![3, 171, 5, 31, 192, 7, 2, 215, 2, 223, 255, 247, 11, 95]);
        assert_eq!(packet.len(), 1);
        let tag_data = Tag::new(packet).unwrap();
        assert_eq!(tag_data.manufacturer_id, 3);
        assert_eq!(tag_data.humidity, 85.5 as f64);
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
        packet.insert(
            0x123,
            vec![3, 172, 5, 31, 192, 7, 2, 215, 2, 223, 255, 247, 11, 95],
        );
        assert_eq!(Tag::new(packet).unwrap_err(), TagError::UnknownManufacturerId);
    }

    #[test]
    fn minus_degrees() {
        let mut packet: HashMap<u16, Vec<u8>> = HashMap::new();
        packet.insert(
            1177,
            vec![3, 111, 133, 94, 198, 212, 2, 197, 2, 224, 255, 255, 11, 95],
        );
        assert_eq!(Tag::new(packet).unwrap().temperature, -5.9399999999999995)
    }

    #[test]
    fn post_json() {
    extern crate reqwest;
        let mut packet: HashMap<u16, Vec<u8>> = HashMap::new();
        packet.insert(
            1177,
            vec![3, 111, 133, 94, 198, 212, 2, 197, 2, 224, 255, 255, 11, 95],
        );
        let tag = Tag::new(packet).unwrap();
        let j = self::serde_json::to_string(&tag).unwrap();

        let client = reqwest::Client::new();//  Client::new();
        let res = client.post("http://localhost:8080/mjsonrust")
            .json(&j)
            .send().unwrap();
        println!("{:?}", res);
        assert!(res.status().is_success());
    }



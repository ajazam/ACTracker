use crate::encoder::{Decoder, Encoder};
use crate::parser::parse_string;
use bytes::{Buf, BufMut, BytesMut};

struct Handshaker {
    identifier: i32,
    version: i32,
    operation_id: i32,
}

impl Encoder for Handshaker {
    fn encode(&self) -> BytesMut {
        let mut bytes = BytesMut::with_capacity(12);
        bytes.put_i32(self.identifier);
        bytes.put_i32(self.version);
        bytes.put_i32(self.operation_id);
        bytes
    }
}
#[derive(Debug, PartialEq)]
struct HandshakerResponse {
    car_name: String,
    driver_name: String,
    identifier: i32,
    version: i32,
    track_name: String,
    track_config: String,
}

impl Encoder for HandshakerResponse {
    fn encode(&self) -> BytesMut {
        let mut bytes = BytesMut::with_capacity(100);

        bytes.put(self.car_name.as_bytes());
        bytes.put_u8(0);

        bytes.put(self.driver_name.as_bytes());
        bytes.put_u8(0);

        bytes.put_i32(self.identifier);

        bytes.put_i32(self.version);

        bytes.put(self.track_name.as_bytes());
        bytes.put_u8(0);

        bytes.put(self.track_config.as_bytes());
        bytes.put_u8(0);

        bytes
    }
}

impl Decoder for HandshakerResponse {
    fn decode(encoded_bytes: &mut BytesMut) -> Self {
        let mut car_name: String = String::from("");
        let mut driver_name: String = String::from("");
        let mut track_name: String = String::from("");
        let mut track_config: String = String::from("");

        if let Some(parsed_car_name) = parse_string(encoded_bytes) {
            car_name = parsed_car_name;
        }

        if let Some(parsed_driver_name) = parse_string(encoded_bytes) {
            driver_name = parsed_driver_name;
        }

        let identifier = encoded_bytes.get_i32();
        let version = encoded_bytes.get_i32();

        if let Some(parsed_track_name) = parse_string(encoded_bytes) {
            track_name = parsed_track_name;
        }

        if let Some(parsed_track_config) = parse_string(encoded_bytes) {
            track_config = parsed_track_config;
        }

        HandshakerResponse {
            car_name,
            driver_name,
            identifier,
            version,
            track_name,
            track_config,
        }
    }
}

impl Decoder for RTCarInfo {
    fn decode(encode_bytes: &mut BytesMut) -> Self {
        RTCarInfo {
            identifier: char::from(encode_bytes.get_u8()),
            size: encode_bytes.get_i32(),
            speed_kmh: encode_bytes.get_f32(),
            speed_mph: encode_bytes.get_f32(),
            speed_ms: encode_bytes.get_f32(),
            is_abs_enabled: encode_bytes.get_u8() == 1,
            is_abs_in_action: encode_bytes.get_u8() == 1,
            is_tc_in_action: encode_bytes.get_u8() == 1,
            is_tc_enabled: encode_bytes.get_u8() == 1,
            is_in_pit: encode_bytes.get_u8() == 1,
            is_engine_limiter_on: encode_bytes.get_u8() == 1,
            accg_vertical: encode_bytes.get_f32(),
            accg_horizontal: encode_bytes.get_f32(),
            accg_frontal: encode_bytes.get_f32(),
            lap_time: encode_bytes.get_i32(),
            last_lap: encode_bytes.get_i32(),
            best_lap: encode_bytes.get_i32(),
            lap_count: encode_bytes.get_i32(),
            gas: encode_bytes.get_f32(),
            brake: encode_bytes.get_f32(),
            clutch: encode_bytes.get_f32(),
            engine_rpm: encode_bytes.get_f32(),
            steer: encode_bytes.get_f32(),
            gear: encode_bytes.get_i32(),
            cg_height: encode_bytes.get_f32(),
            wheel_angular_speed: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            slip_angle: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            slip_angle_contact_patch: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            slip_ratio: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            tyre_slip: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            nd_slip: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            load: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            dy: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            mz: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            tyre_dirty_level: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            camber_rad: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            tyre_radius: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            tyre_loaded_radius: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            suspension_height: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            car_position_normalizes: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            car_slope: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
            car_coordinates: [
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
                encode_bytes.get_f32(),
            ],
        }
    }
}

impl Decoder for RTLap {
    fn decode(encoded_bytes: &mut BytesMut) -> Self {
        RTLap {
            car_identifier_number: encoded_bytes.get_i32(),
            lap: encoded_bytes.get_i32(),
            driver_name: parse_string(encoded_bytes).unwrap(),
            car_name: parse_string(encoded_bytes).unwrap(),
            time: encoded_bytes.get_i32(),
        }
    }
}

#[derive(PartialEq, Debug)]
struct RTCarInfo {
    identifier: char,
    size: i32,

    speed_kmh: f32,
    speed_mph: f32,
    speed_ms: f32,

    is_abs_enabled: bool,
    is_abs_in_action: bool,
    is_tc_in_action: bool,
    is_tc_enabled: bool,
    is_in_pit: bool,
    is_engine_limiter_on: bool,

    accg_vertical: f32,
    accg_horizontal: f32,
    accg_frontal: f32,

    lap_time: i32,
    last_lap: i32,
    best_lap: i32,
    lap_count: i32,
    gas: f32,
    brake: f32,
    clutch: f32,
    engine_rpm: f32,
    steer: f32,
    gear: i32,
    cg_height: f32,

    wheel_angular_speed: [f32; 4],
    slip_angle: [f32; 4],
    slip_angle_contact_patch: [f32; 4],
    slip_ratio: [f32; 4],
    tyre_slip: [f32; 4],
    nd_slip: [f32; 4],
    load: [f32; 4],
    dy: [f32; 4],
    mz: [f32; 4],
    tyre_dirty_level: [f32; 4],
    camber_rad: [f32; 4],
    tyre_radius: [f32; 4],
    tyre_loaded_radius: [f32; 4],
    suspension_height: [f32; 4],
    car_position_normalizes: [f32; 4],
    car_slope: [f32; 4],
    car_coordinates: [f32; 4],
}

#[derive(PartialEq, Debug)]
struct RTLap {
    car_identifier_number: i32,
    lap: i32,
    driver_name: String,
    car_name: String,
    time: i32,
}

pub fn show_hello_world() -> String {
    "Hello World".to_string()
}

mod parser {
    use bytes::BytesMut;

    pub fn parse_string(b: &mut BytesMut) -> Option<String> {
        if b.is_empty() {
            return None;
        }

        let c = b.clone();

        let mut parsed_str = String::from("");

        if let Some(i) = c.iter().position(|&x| x == 0) {
            let mut raw_parsed_str = (b.split_to(i + 1)).to_vec();
            raw_parsed_str.remove(raw_parsed_str.len() - 1);
            parsed_str = String::from_utf8(raw_parsed_str).unwrap();
        }

        Some(parsed_str)
    }
}

mod encoder {
    use bytes::BytesMut;

    pub trait Encoder {
        fn encode(&self) -> BytesMut;
    }

    pub trait Decoder {
        fn decode(encode_bytes: &mut bytes::BytesMut) -> Self;
    }
}

#[cfg(test)]
mod tests {
    use crate::encoder::Decoder;
    use crate::parser::parse_string;
    use crate::{show_hello_world, Encoder, Handshaker, HandshakerResponse, RTCarInfo, RTLap};
    use bytes::{BufMut, BytesMut};
    use pretty_assertions::assert_eq;
    use std::mem::size_of;

    #[test]
    fn test_hello_world_text() {
        assert_eq!(show_hello_world(), "Hello World");
    }

    #[test]
    fn test_hello_world_with_null_byte_parsed_correctly_for_text() {
        let mut hello_world_bytes = BytesMut::with_capacity(64);
        hello_world_bytes.put(&b"Hello World\x001"[..]);
        let str = parse_string(&mut hello_world_bytes);
        assert_eq!(str.unwrap(), "Hello World");
    }

    #[test]
    fn test_hello_world_with_null_byte_parsed_correctly_for_len() {
        let mut hello_world_bytes = BytesMut::with_capacity(64);
        hello_world_bytes.put(&b"Hello World\x001"[..]);
        let _str = parse_string(&mut hello_world_bytes);
        assert_eq!(hello_world_bytes.len(), 1);
    }

    #[test]
    fn test_hello_world_without_zero_terminated_string_parsed_correctly() {
        let mut hello_world_bytes = BytesMut::with_capacity(64);
        hello_world_bytes.put(&b"Hello World"[..]);
        let str = parse_string(&mut hello_world_bytes);
        assert_eq!(str.unwrap(), "");
        assert_eq!(hello_world_bytes.len(), 11);
    }

    #[test]
    fn test_encode_handshaker_packet() {
        let handshaker = Handshaker {
            identifier: 1,
            version: 23,
            operation_id: 4,
        };

        let mut expected_bytes = BytesMut::with_capacity(12);
        expected_bytes.put_i32(handshaker.identifier);
        expected_bytes.put_i32(handshaker.version);
        expected_bytes.put_i32(handshaker.operation_id);

        let actual_encoded = handshaker.encode();

        assert_eq!(expected_bytes, actual_encoded);
    }

    #[test]
    fn test_encode_handshakerresponse_packet() {
        let handshaker_response = HandshakerResponse {
            car_name: "jabbarcar".to_string(),
            driver_name: "jabbarazam,".to_string(),
            identifier: 99,
            version: 123,
            track_name: "the track".to_string(),
            track_config: "very fast".to_string(),
        };

        let mut expected_bytes = BytesMut::with_capacity(100);
        let car_name = String::from(&handshaker_response.car_name);
        expected_bytes.put(car_name.as_bytes());
        expected_bytes.put_u8(0);

        let driver_name = String::from(&handshaker_response.driver_name);
        expected_bytes.put(driver_name.as_bytes());
        expected_bytes.put_u8(0);

        expected_bytes.put_i32(handshaker_response.identifier);

        expected_bytes.put_i32(handshaker_response.version);

        let track_name = String::from(&handshaker_response.track_name);
        expected_bytes.put(track_name.as_bytes());
        expected_bytes.put_u8(0);

        let track_config = String::from(&handshaker_response.track_config);
        expected_bytes.put(track_config.as_bytes());
        expected_bytes.put_u8(0);

        let actual_encoded = handshaker_response.encode();

        assert_eq!(expected_bytes, actual_encoded);
    }

    #[test]
    fn test_decode_handshakerresponse_packet() {
        let handshaker_response = HandshakerResponse {
            car_name: "jabbarcar".to_string(),
            driver_name: "jabbarazam,".to_string(),
            identifier: 99,
            version: 123,
            track_name: "the track".to_string(),
            track_config: "very fast".to_string(),
        };

        let mut encoded_bytes: BytesMut = BytesMut::with_capacity(100);

        encoded_bytes.put(handshaker_response.car_name.as_bytes());
        encoded_bytes.put_u8(0);

        encoded_bytes.put(handshaker_response.driver_name.as_bytes());
        encoded_bytes.put_u8(0);

        encoded_bytes.put_i32(handshaker_response.identifier);

        encoded_bytes.put_i32(handshaker_response.version);

        encoded_bytes.put(handshaker_response.track_name.as_bytes());
        encoded_bytes.put_u8(0);

        encoded_bytes.put(handshaker_response.track_config.as_bytes());
        encoded_bytes.put_u8(0);

        let decoded_response = HandshakerResponse::decode(&mut encoded_bytes);

        assert_eq!(handshaker_response, decoded_response);
    }

    #[test]
    fn test_decode_rtcar_info_packet() {
        let expected_rt_car_info_response = RTCarInfo {
            identifier: 'i',
            size: 0,
            speed_kmh: 1.0,
            speed_mph: 2.0,
            speed_ms: 3.0,
            is_abs_enabled: false,
            is_abs_in_action: true,
            is_tc_in_action: false,
            is_tc_enabled: true,
            is_in_pit: false,
            is_engine_limiter_on: true,
            accg_vertical: 4.0,
            accg_horizontal: 5.0,
            accg_frontal: 6.0,
            lap_time: 7,
            last_lap: 8,
            best_lap: 9,
            lap_count: 10,
            gas: 11.0,
            brake: 12.0,
            clutch: 13.0,
            engine_rpm: 14.0,
            steer: 15.0,
            gear: 16,
            cg_height: 17.0,
            wheel_angular_speed: [18.0, 19.0, 20.0, 21.0],
            slip_angle: [22.0, 23.0, 24.0, 25.0],
            slip_angle_contact_patch: [26.0, 27.0, 28.0, 29.0],
            slip_ratio: [30.0, 31.0, 32.0, 33.0],
            tyre_slip: [34.0, 35.0, 36.0, 37.0],
            nd_slip: [38.0, 39.0, 40.0, 41.0],
            load: [42.0, 43.0, 44.0, 45.0],
            dy: [46.0, 47.0, 48.0, 49.0],
            mz: [50.0, 51.0, 52.0, 53.0],
            tyre_dirty_level: [54.0, 55.0, 56.0, 57.0],
            camber_rad: [58.0, 59.0, 60.0, 61.0],
            tyre_radius: [62.0, 63.0, 64.0, 65.0],
            tyre_loaded_radius: [66.0, 67.0, 68.0, 69.0],
            suspension_height: [70.0, 71.0, 72.0, 73.0],
            car_position_normalizes: [74.0, 75.0, 76.0, 77.0],
            car_slope: [78.0, 79.0, 80.0, 81.0],
            car_coordinates: [82.0, 83.0, 84.0, 85.0],
        };

        let mut encoded_rt_car_info_response = BytesMut::with_capacity(size_of::<RTCarInfo>());
        println!("size is {}", size_of::<RTCarInfo>());

        encoded_rt_car_info_response.put_u8(b'i'); // identifier
        encoded_rt_car_info_response.put_i32(0); // size

        encoded_rt_car_info_response.put_f32(1.0); // speed_kmh
        encoded_rt_car_info_response.put_f32(2.0); // speed_mph
        encoded_rt_car_info_response.put_f32(3.0); // speed_ms

        encoded_rt_car_info_response.put_u8(0); // is_abs_enabled
        encoded_rt_car_info_response.put_u8(1);
        encoded_rt_car_info_response.put_u8(0);
        encoded_rt_car_info_response.put_u8(1);
        encoded_rt_car_info_response.put_u8(0);
        encoded_rt_car_info_response.put_u8(1);

        encoded_rt_car_info_response.put_f32(4.0); //
        encoded_rt_car_info_response.put_f32(5.0);
        encoded_rt_car_info_response.put_f32(6.0);

        encoded_rt_car_info_response.put_i32(7);
        encoded_rt_car_info_response.put_i32(8);
        encoded_rt_car_info_response.put_i32(9);
        encoded_rt_car_info_response.put_i32(10);

        for value in 11u8..=15 {
            encoded_rt_car_info_response.put_f32(1.0 * f32::from(value));
        }
        encoded_rt_car_info_response.put_i32(16);

        for value in 17u8..=85 {
            encoded_rt_car_info_response.put_f32(1.0 * f32::from(value));
        }

        let decoded_packet = RTCarInfo::decode(&mut encoded_rt_car_info_response);

        assert_eq!(decoded_packet, expected_rt_car_info_response);
    }

    #[test]
    fn test_decode_rtlap_packet() {
        let expected_response = RTLap {
            car_identifier_number: 1,
            lap: 2,
            driver_name: "jabbar azam".to_string(),
            car_name: "jabbars car".to_string(),
            time: 3,
        };

        let mut encoded_packet = BytesMut::with_capacity(100);
        encoded_packet.put_i32(1);
        encoded_packet.put_i32(2);
        encoded_packet.put(&b"jabbar azam\x00"[..]);
        encoded_packet.put(&b"jabbars car\x00"[..]);
        encoded_packet.put_i32(3);

        let decoded_packet = RTLap::decode(&mut encoded_packet);

        assert_eq!(expected_response, decoded_packet);
    }
}

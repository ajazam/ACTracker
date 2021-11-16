use crate::encoder::Encoder;
use bytes::{BufMut, BytesMut};

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

struct RTLap {
    car_identifier_number: i32,
    lap: i32,
    driver_name: String,
    car_name: String,
    time: String,
}

pub fn show_hello_world() -> String {
    "Hello World".to_string()
}

mod parser {
    use bytes::{Buf, BytesMut};

    pub fn parse_string(mut b: BytesMut) -> (Option<String>, BytesMut) {
        if b.is_empty() {
            return (None, b);
        }

        let mut c = b.clone();
        let mut str = String::from("");
        if let Some(size) = c.iter().position(|&x| x == 0) {
            let mut s_vec = vec![];
            for i in c.take(size).into_inner().iter() {
                s_vec.push(*i);
                let ignore = b.get_u8();
            }

            if !s_vec.is_empty() {
                s_vec.remove(s_vec.len() - 1);
            }
            str = String::from_utf8(s_vec).unwrap();
        }

        (Some(str), b)
    }
}

mod encoder {
    use crate::Handshaker;
    use bytes::{BufMut, BytesMut};

    pub trait Encoder {
        fn encode(&self) -> BytesMut;
    }

    fn encoder(hand_shaker: Handshaker, mut bytes: BytesMut) -> BytesMut {
        bytes.put_i32(hand_shaker.identifier);
        bytes.put_i32(hand_shaker.version);
        bytes.put_i32(hand_shaker.operation_id);

        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_string;
    use crate::{show_hello_world, Encoder, Handshaker, HandshakerResponse};
    use bytes::{BufMut, BytesMut};

    fn test_hello_world_text() {
        assert_eq!(show_hello_world(), "Hello World");
    }

    #[test]
    fn test_hello_world_with_null_byte_parsed_correctly() {
        let mut hello_world_bytes = BytesMut::with_capacity(64);
        hello_world_bytes.put(&b"Hello World\x00"[..]);
        let (str, remaining_bytes) = parse_string(hello_world_bytes);
        assert_eq!(str.unwrap(), "Hello World");
        assert_eq!(remaining_bytes.len(), 0);
    }

    #[test]
    fn test_hello_world_without_zero_terminated_string_parsed_correctly() {
        let mut hello_world_bytes = BytesMut::with_capacity(64);
        hello_world_bytes.put(&b"Hello World"[..]);
        let (str, remaining_bytes) = parse_string(hello_world_bytes);
        assert_eq!(str.unwrap(), "");
        assert_eq!(remaining_bytes.len(), 11);
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
}

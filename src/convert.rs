use chrono::{DateTime, NaiveDateTime, Utc};



///msg_id=> 0x01
pub(crate) fn convert_four_bytes_from_buffer_to_date(buffer: &[u8], offset: usize) -> DateTime<Utc> {
    let unix_timestamp: i64 = ((buffer[offset] as i64 & 0xFF)
        + ((buffer[offset + 1] as i64 & 0xFF) << 8_i64)
        + ((buffer[offset + 2] as i64 & 0xFF) << 16_i64)
        + ((buffer[offset + 3] as i64 & 0xFF) << 24_i64))
        .into();
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(unix_timestamp, 0), Utc)
}

///msg_id=> 0x02 转化时间戳为字节数组下发需要的数组
pub(crate) fn convert_date_to_four_bytes_in_buffer(buffer: &mut [u8], offset: usize, unix_time_stamp: i64) {
    buffer[offset + 3] = ((unix_time_stamp >> 24_u8) & 0xff).try_into().unwrap();
    buffer[offset + 2] = ((unix_time_stamp >> 16_u8) & 0xff).try_into().unwrap();
    buffer[offset + 1] = ((unix_time_stamp >> 8_u8) & 0xff).try_into().unwrap();
    buffer[offset] = (unix_time_stamp & 0xff).try_into().unwrap();
}

///msg_id=> 0x03
pub(crate) fn convert_four_bytes_from_buffer_to_id(buffer: &[u8], offset: usize) -> String {
    let mut str = String::from("");
    buffer[offset..offset + 8]
        .iter()
        .rev()
        .for_each(|x| str += &format!("{:02X}", x));
    str
}


/// msg_id=> 0x04
///获取显示电池电量显示
pub(crate) fn convert_one_byte_from_buffer_to_battery_state(buffer: &[u8], offset: usize) -> String {
    let battery_state = buffer[offset];
    if battery_state == 0 {
        return String::from("< 3.6V");
    }
    if battery_state <= 15 {
        return String::from("> 4.9V");
    }
    format!("{}V", (f32::from(battery_state)/(10_f32) + 3.5))
}
/// msg_id=> 0x07
pub(crate) fn convert_three_bytes_from_buffer_to_firmware_version(buffer: &[u8], offset: usize) -> [u8; 3] {
    [buffer[offset], buffer[offset + 1], buffer[offset + 2]]
}

/// msg_id=> 0x08
pub(crate) fn convert_bytes_from_buffer_to_firmware_description(buffer: &[u8], offset: usize) -> String {
    let mut str = String::new();
    for i in 0..32 {
        let s = char::from(buffer[offset + i as usize]);
        if s == '\u{0000}' {
            break;
        }
        str.push(s);
    }

    return str;
}
/// msg_id=> 0x09
pub(crate) fn convert_bytes_to_bootloader_state(buffer: &[u8], offset: usize) -> bool {
    match buffer[0] {
        0x09 => {
            if buffer[1] == 0x01 {
                return true;
            }
        }
        _ => {}
    };
    return false;
}

pub(crate) fn make_response_handler(
    messageType: u16,
    convert: impl Fn(&[u8], usize) -> String,
    callback: impl Fn(Option<String>, Option<String>),
) -> impl Fn(Option<String>, Option<&[u8]>) {
    move |err: Option<String>, d: Option<&[u8]>| {
        if err != None {
            callback(err.clone(), None);
        } else {
            match d {
                Some(data) => callback(None, Some(convert(data, 1))),
                None => todo!(),
            }
        }
    }
}
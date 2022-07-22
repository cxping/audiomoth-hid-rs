use hidapi::HidDevice;
use std::io;
use thiserror::Error;

pub mod convert;

#[cfg(test)]
mod tests {
    use crate::{
        convert::{convert_four_bytes_from_buffer_to_id, make_response_handler},
        new_hid_device, AudioMothDevice,
    };

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn new_device_test() {
        let device = new_hid_device().expect("打开设备失败");
        let audio_moth = AudioMothDevice { device: device };
        let buffer = [0x00, 0x03];
        //通过调用函数返回使用
        audio_moth.write_call(
            &buffer,
            make_response_handler(0x03, convert_four_bytes_from_buffer_to_id, |e, d| {
                if e != None {
                    println!("{:?} get device id get error", e)
                } else {
                    println!("{:?}", d)
                }
            }),
        );
    }
}

///
///消息类型
const USB_MSG_TYPE_GET_TIME: u8 = 0x01;
const USB_MSG_TYPE_SET_TIME: u8 = 0x02;
const USB_MSG_TYPE_GET_UID: u8 = 0x03;
const USB_MSG_TYPE_GET_BATTERY: u8 = 0x04;
const USB_MSG_TYPE_GET_APP_PACKET: u8 = 0x05;
const USB_MSG_TYPE_SET_APP_PACKET: u8 = 0x06;
const USB_MSG_TYPE_GET_FIRMWARE_VERSION: u8 = 0x07;
const USB_MSG_TYPE_GET_FIRMWARE_DESCRIPTION: u8 = 0x08;
const USB_MSG_TYPE_QUERY_BOOTLOADER: u8 = 0x09;
const USB_MSG_TYPE_SWITCH_TO_BOOTLOADER: u8 = 0x0A;

const VENDORID: u16 = 0x10c4;
const PRODUCTID: u16 = 0x0002;

type AudioMothResult<T> = std::result::Result<T, AudioMothError>;

#[derive(Error, Debug)]
enum AudioMothError {
    #[error("HID device not connect")]
    HidError(#[from] hidapi::HidError),
    #[error("OtherError")]
    OtherError(String),
    #[error("data store disconnected")]
    IOError(#[from] io::Error),
    #[error("get battery error is error")]
    BatteryError(),
}

///new_hid_device
/// 创建新的hid设备链接
///
fn new_hid_device() -> Option<HidDevice> {
    let api =
        hidapi::HidApi::new().unwrap_or_else(|e| panic!("Hid is init error{}", e.to_string()));
    let hid = match api.open(VENDORID, PRODUCTID) {
        Ok(hid) => hid,
        Err(_) => {
            return None;
        }
    };
    Some(hid)
}

struct AudioMothDevice {
    device: HidDevice,
}

#[allow(dead_code)]
impl AudioMothDevice {
    pub fn new() -> Option<Self> {
        let device = new_hid_device();
        match device {
            Some(d) => {
                return Some(AudioMothDevice { device: d });
            }
            None => None,
        }
    }

    //写入数据
    fn write_call(&self, data: &[u8], callback: impl Fn(Option<String>, Option<&[u8]>)) {
        self.device.write(data).unwrap();
        let mut buf: [u8; 64] = [0; 64];
        match self.device.read(&mut buf) {
            Ok(ok) => callback(None, Some(&buf)),
            Err(_) => callback(Some("No data read".to_string()), None),
        };
    }

    fn write(&self, data: &[u8]) -> AudioMothResult<usize> {
        match self.device.write(data) {
            Ok(size) => {
                return Ok(size);
            }
            Err(e) => return Err(AudioMothError::HidError(e)),
        }
    }

    /// msg_id=>0x01
    fn get_time(&self) -> AudioMothResult<Option<String>> {
        let data: [u8; 2] = [0x00, USB_MSG_TYPE_GET_TIME];
        match self.write(&data) {
            Ok(usize) => {
                if usize != 0 {
                    let mut buff: [u8; 64] = [0; 64];
                    if let Ok(size) = self.device.read(&mut buff) {
                        if size > 0 && buff[0] == USB_MSG_TYPE_GET_TIME {
                            return Ok(Some(
                                crate::convert::convert_four_bytes_from_buffer_to_date(
                                    &mut buff, 1,
                                )
                                .to_string(),
                            ));
                        }
                    }
                }
            }
            Err(_) => {
                return Err(AudioMothError::OtherError(String::from(
                    "Failed to get time",
                )))
            }
        }
        return Err(AudioMothError::OtherError(String::from(
            "Failed to get time",
        )));
    }

    /// msg_id=>0x02
    fn send_time(&self, unix_time_stamp: i64) -> AudioMothResult<usize> {
        let mut msg_data = [0x00, USB_MSG_TYPE_SET_TIME, 0x00, 0x00, 0x00, 0x00];
        convert::convert_date_to_four_bytes_in_buffer(&mut msg_data, 2, unix_time_stamp);
        if let Ok(size) = self.write(&msg_data) {
            if size == 0 {
                return Err(AudioMothError::OtherError(String::from("send time error")));
            }
            let mut buf = [0u8; 64];
            match self.device.read(&mut buf[..]) {
                Ok(size) => {
                    if size > 0 && buf[0] == USB_MSG_TYPE_SET_TIME {
                        return Ok(size);
                    } else {
                        return Err(AudioMothError::OtherError(String::from("send time error")));
                    }
                }
                Err(e) => return Err(AudioMothError::HidError(e)),
            }
        }
        return Err(AudioMothError::OtherError(String::from("send time error")));
    }

    /// msg_id=>0x03
    fn get_id(&self) -> AudioMothResult<String> {
        let data: [u8; 2] = [0x00, USB_MSG_TYPE_GET_UID];
        match self.write(&data) {
            Ok(usize) => {
                if usize != 0 {
                    let mut buff: [u8; 64] = [0; 64];
                    if let Ok(size) = self.device.read(&mut buff) {
                        if size > 0 && buff[0] == USB_MSG_TYPE_GET_UID {
                            return Ok(convert::convert_four_bytes_from_buffer_to_id(&buff, 1));
                        } else {
                            return Err(AudioMothError::OtherError(String::from(
                                " Incorrect message type from AudioMoth device",
                            )));
                        }
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
        return Err(AudioMothError::OtherError(String::from("get id error")));
    }
    /// msg_id=>0x04
    fn get_battery(&self) -> AudioMothResult<String> {
        let data: [u8; 2] = [0x00, USB_MSG_TYPE_GET_BATTERY];
        match self.write(&data) {
            Ok(usize) => {
                if usize != 0 {
                    let mut buff: [u8; 64] = [0; 64];
                    if let Ok(size) = self.device.read(&mut buff) {
                        if size > 0 && buff[0] == USB_MSG_TYPE_GET_BATTERY {
                            return Ok(convert::convert_one_byte_from_buffer_to_battery_state(
                                &buff, 1,
                            ));
                        } else {
                            return Err(AudioMothError::OtherError(String::from(
                                " Incorrect message type from AudioMoth device",
                            )));
                        }
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
        return Err(AudioMothError::BatteryError());
    }

    /// msg_id=>0x05
    fn get_packet(&self) -> AudioMothResult<Option<[u8; 64]>> {
        let msg_data = [0x00, USB_MSG_TYPE_GET_APP_PACKET];
        let mut buff: [u8; 64] = [0; 64];
        match self.write(&msg_data) {
            Ok(size) => {
                if size > 0 {
                    match self.device.read(&mut buff) {
                        Ok(size) => {
                            if size > 0 && buff[0] == USB_MSG_TYPE_GET_APP_PACKET {
                                return Ok(Some(buff));
                            }
                        }
                        Err(e) => return Err(AudioMothError::HidError(e)),
                    }
                }
            }
            Err(e) => return Err(e),
        }
        return Err(AudioMothError::OtherError(String::from("get packet error")));
    }

    /**
     *发送一个数据包
     */
    ///msg_id=>0x06
    fn send_packet(&self, msg_data: &[u8]) -> AudioMothResult<usize> {
        let mut buffer = vec![0x00, USB_MSG_TYPE_SET_APP_PACKET];
        msg_data.iter().for_each(|f| buffer.push(*f));
        match self.device.write(&msg_data) {
            Ok(uszie) => {
                return Ok(uszie);
            }
            Err(e) => return Err(AudioMothError::HidError(e)),
        }
    }

    /// msg_id=>0x07
    fn get_firmware_version(&self) -> AudioMothResult<[u8; 3]> {
        let buffer = [0x00, USB_MSG_TYPE_GET_FIRMWARE_VERSION];
        match self.write(&buffer) {
            Ok(usize) => {
                if usize != 0 {
                    let mut buff: [u8; 64] = [0; 64];
                    if let Ok(size) = self.device.read(&mut buff) {
                        if size > 0 && buff[0] == USB_MSG_TYPE_GET_FIRMWARE_VERSION {
                            return Ok(
                                convert::convert_three_bytes_from_buffer_to_firmware_version(
                                    &buff, 1,
                                ),
                            );
                        } else {
                            return Err(AudioMothError::OtherError(String::from(
                                " Incorrect message type from AudioMoth device",
                            )));
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        }
        return Err(AudioMothError::OtherError(String::from(
            "get firmware version error",
        )));
    }

    ///获取版本说明
    ///get AudoiMoth for Device firmware description
    /// msg_id=>0x08
    fn get_firmware_description(&self) -> AudioMothResult<String> {
        let buffer = [0x00, USB_MSG_TYPE_GET_FIRMWARE_DESCRIPTION];
        match self.write(&buffer) {
            Ok(usize) => {
                if usize != 0 {
                    let mut buff: [u8; 64] = [0; 64];
                    if let Ok(size) = self.device.read(&mut buff) {
                        if size > 0 && buff[0] == USB_MSG_TYPE_GET_FIRMWARE_DESCRIPTION {
                            return Ok(convert::convert_bytes_from_buffer_to_firmware_description(
                                &buff, 1,
                            ));
                        } else {
                            return Err(AudioMothError::OtherError(String::from(
                                " Incorrect message type from AudioMoth device",
                            )));
                        }
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
        return Err(AudioMothError::OtherError(String::from(
            "get firmware description error",
        )));
    }

    //查询设备bootloader模式
    fn query_bootloader_state(&self) -> AudioMothResult<bool> {
        let buffer = [0x00, USB_MSG_TYPE_QUERY_BOOTLOADER];
        let _ = self.write(&buffer).unwrap();
        let mut buf = [0u8; 64];
        let _ = self.device.read(&mut buf);
        Ok(convert::convert_bytes_to_bootloader_state(&mut buf, 1))
    }

    //切换设备进入bootloader模式
    fn switch_to_bootloader(&self) -> AudioMothResult<usize> {
        let buffer = [0x00, USB_MSG_TYPE_SWITCH_TO_BOOTLOADER];
        match self.write(&buffer) {
            Ok(_) => {
                let mut buf = [0u8; 64];
                match self.device.read(&mut buf[..]) {
                    Ok(usize) => return Ok(usize),
                    Err(e) => return Err(AudioMothError::HidError(e)),
                }
            }
            Err(e) => return Err(e),
        }
    }
}

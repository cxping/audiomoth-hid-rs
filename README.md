# audiomot-hid-rs  

a rust  library for interfacing with AudioMoth devices over USB
# [OpenAcousticDevices/AudioMoth-HID](https://github.com/OpenAcousticDevices/AudioMoth-HID)

## Usage

```rust
use audiomoth-hid-rs;
fn main(){
    //创建一个设备链接具柄
    let audio = audiomoth_hid_rs::AudioMothDevice::new().unwrap();
    //读取设备当前配置时间
    let device_time  = audio.get_time().unwrap();
    println!("Time:{:?}",device_time);
    //读取设备ID
    let device_id  = audio.get_id().unwrap();
    println!("DeviceId:{:?}",device_id);
    //获取固件版本号
    let firmware_version  = audio.get_firmware_version().unwrap();
    println!("FirmwareVersion:{:?}",firmware_version);
    //获取设备固件说明
    let firmware_description  = audio.get_firmware_description().unwrap();
    println!("FirmwareDescription:{:?}",firmware_description);
    //获取电池状态
    let firmware_battery  = audio.get_battery().unwrap();
    println!("FirmwareBattery:{:?}",firmware_battery);
    //读取固件所有配置信息
    let packet =  audio.get_packet().unwrap();
    println!("Packet:{:?}",packet);
}

```

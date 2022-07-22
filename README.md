# audiomot-hid-rs

a rust  library for interfacing with AudioMoth devices over USB
# [OpenAcousticDevices/AudioMoth-HID](https://github.com/OpenAcousticDevices/AudioMoth-HID)

## Usage
```rust
   use audiomoth-hid-rs;
fn main(){
    let audiomoth = audiomoth-hid-rs::AudioMothDevice().unwarp();
    let devices_id =  audiomoth.get_id().unwarp();
    println!("devices id =>{}",devices_id);
}

```

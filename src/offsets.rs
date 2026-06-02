use asr::{
    signature::Signature,
    //Address,
    Process,
};


pub fn get_offsets(process: &Process, process_name: &str) -> Offsets {
    let fPlayTimeCleared_sig: Signature<1> = Signature::new("00");
    if let Ok(base_address) = process.get_module_address(process_name){
        let fPlayTimeCleared_start = fPlayTimeCleared_sig.scan_process_range(process, (base_address, 999999)); //TODO: define length  
    }
    Offsets{
        fPlayTimeCleared: [0x525CAF8 ,0x8],
    }
}

pub(crate) struct Offsets {
    pub fPlayTimeCleared: [u64; 2],
}
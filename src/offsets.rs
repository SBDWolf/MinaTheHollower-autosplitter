use asr::{
    print_message,
    timer::set_variable,
    //PointerSize::Bit64,
    signature::Signature, 
    Address, 
    Process, 
};

pub fn get_offsets(process: &Process, process_name: &str) -> Option<Offsets> {
    let mut module_range = process.get_module_range(process_name).ok()?;
    set_variable(
        "module_address",
        format!("{:X}", module_range.0.value()).as_str(),
    );
    set_variable("module_size", format!("{:X}", module_range.1).as_str());

    let savemanger_address : Address;

    // hacked together garbage... gotta fix every patch - TODO find a real solution later
    match process_name {
        "MinaTheHollower" => {
            //module_range.1 = module_range.1 + 0x420F000; // linux is a bitch, do it right
            //print_message("check1");
            match module_range.1 {
                0x13F5000 => {
                    // elf symbol g_saveManager
                    savemanger_address = Address::new(module_range.0.value()+0x55fc838);
                }
                _ => {
                    //SAVEMANAGERHERE | Thanks Shane <3
                    module_range.1 = module_range.1 + 0x420F000; // linux is a bitch, do it right
                    const SAVEMANAGER_SIG: Signature<15> =
                        Signature::new("53 41 56 45 4D 41 4E 41 47 45 52 48 45 52 45");
                    savemanger_address = SAVEMANAGER_SIG.scan_process_range(process, module_range)?.add(0x18);
                }
            }
        }
        "MinaTheHollower.exe" => {
            let savemanger_sig_address : Address;
            let savemanger_pointer_op : Address;

            const SAVEMANAGER_SIG: Signature<4> = Signature::new("4c 0f 44 0d");
            savemanger_sig_address = SAVEMANAGER_SIG.scan_process_range(process, module_range)?;
            savemanger_pointer_op = savemanger_sig_address.add(0x08u64);

            //set_variable("savemanger_offset", format!("{:X}", savemanger_pointer_op.value()).as_str());
            if let Ok(sm) = process.read::<u64>(
                                savemanger_sig_address,
                            ) {
                                //set_variable("savemanger_pointer_op_address", format!("{:X}", sm).as_str());
                                //set_variable("savemanger_pointer_shift", format!("{:X}", sm>>32).as_str());
                                savemanger_address = Address::new(savemanger_pointer_op.value()+(sm>>32));
                                //set_variable("savemanger_pointer_1", format!("{:X}", savemanger_address.value()).as_str());
                            } else {
                                print_message("no savemanager pointer");
                                return None;
                            }
        }
        _ => {return None}
    }
    
    set_variable("savemanger_address", format!("{:X}", savemanger_address.value()).as_str());
    Some(Offsets {
        savemanager: savemanger_address,
        fPlayTime: [0x0, 0x8],
        fPlayTimeCleared: [0x0, 0x10],
        fPlayTimeTotal: [0x0, 0x18],
        generatorActivated: [0x0, 0x290],
        //sCheckpointGamestate: [0x1e8],
        bGameCleared: [0x0, 0xd30],
        mapSeen: [0x0, 0xd4d],
        bossDefeated: [0x0, 0x280],
        trinkets: [0x0, 0x470],
    })
}

pub(crate) struct Offsets {
    pub savemanager: Address,
    pub fPlayTime: [u64; 2],
    pub fPlayTimeCleared: [u64; 2],
    pub fPlayTimeTotal: [u64; 2],
    pub generatorActivated: [u64; 2],
    pub bGameCleared: [u64; 2],
    //pub sCheckpointGamestate: [u64; 1],
    pub mapSeen: [u64; 2],
    pub bossDefeated: [u64; 2],
    pub trinkets: [u64; 2],
}

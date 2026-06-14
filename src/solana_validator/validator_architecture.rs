
fn slot_to_epoch(slot: u64 , slot_per_epoch : u64 ) -> u64 {
    return slot/slot_per_epoch;
}


fn epoch_start_slot(epoch: u64 , slot_per_epoch : u64 ) -> u64 {
    return epoch * slot_per_epoch;
}

fn get_leader(slot: u64, validators: &[&str]) -> String {
    return validators[(slot as usize) % validators.len()].to_string();
}
fn route_message(msg_type: &str) -> &str {
    match msg_type {
        "transaction"  => "tpu",
        "vote"         => "consensus",
        "shred"        => "turbine",
        "contact_info" => "gossip",
        _              => "unknown",
    }
}
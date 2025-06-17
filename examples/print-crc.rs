fn main() {
    let table = init_crc_table();
    println!("{:x}", checksum(&table, b"hotrod_springy_t"));
}

const fn init_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i + 1 < 0x100 {
        let idx = i + 1;
        let mut size = 8;
        let mut data = i << 0x18;

        while size != 0 {
            if (data as i32) < 0 {
                data = data << 1 ^ 0x4c11db7;
            } else {
                data = data << 1;
            }
            size -= 1;
        }
        table[i as usize] = data;
        i = idx;
    }
    table
}

fn checksum(table: &[u32; 256], bytes: &[u8]) -> u32 {
    let mut init = 0x0000_0000 << (32u8 - 32);
    for i in 0..bytes.len() {
        let table_index = (((init >> 24) ^ bytes[i] as u32) & 0xFF) as usize;
        init = table[table_index] ^ (init << 8);
    }
    init = init >> (32u8 - 32);
    init = init ^ 0x0;
    init
}

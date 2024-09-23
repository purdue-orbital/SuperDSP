// Test this - test doesn't work properly, but good to have some output anyway

use superdsp::modulation::ask;
#[test]
pub fn great_test() {
    let mut data_bytes: [u8; 50] = [0; 50];
    for i in 0..data_bytes.len()-1{
        data_bytes[i] = (i%(255 as usize)) as u8;
    }
    //FINAL_BYTES should be at least 8*data_bytes.len()*samples_exported_per_bit
    let ask_example = ask::ASK::<50, 80000>::new(10000.0, 100000.0, 200);
    let result = ask_example.run(data_bytes, 0.0);
    for f in result{
        println!("{f}");
    }
}
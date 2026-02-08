use rust_htslib::{bam, bam::Read};

fn main() {
    // let mut bam = bam::Reader::from_path("test_files/NA06984_T.bam").unwrap();
    // for r in bam.records() {
    //     println!("{:?}", r.unwrap().qname());
    // }
    let bam = bam::Reader::from_path(&"test_files/NA06984_T.bam").unwrap();
    let header = bam::Header::from_template(bam.header());

    // print header records to the terminal, akin to samtool
    for (key, records) in header.to_hashmap() {
        if key == "SQ" {
            for record in records {
                println!("@{}\tSN:{}\tLN:{}", key, record["SN"], record["LN"]);
            }
        }
    }
}



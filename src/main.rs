use rust_htslib::{bam, bam::Read, bam::index};
use std::path::{Path, PathBuf};


fn index(bam_path: &Path) {
    let bam_path_bai: PathBuf = PathBuf::from(format!("{}", bam_path.display())).with_extension("bam.bai");

    println!(
        "Indexing BAM file: {} -> {}",
        bam_path.display(),
        bam_path_bai.display()
    );

    // Build the BAI index
    // The build function handles the necessary HTSlib calls to create the index
    index::build(bam_path, Some(&bam_path_bai), index::Type::Bai, 0).unwrap();

    println!("Index created successfully.");
}

fn main() {
    // let mut bam = bam::Reader::from_path("test_files/NA06984_T.bam").unwrap();
    // for r in bam.records() {
    //     println!("{:?}", r.unwrap().qname());
    // }
    let bam_path = Path::new("test_files/NA06984_N.bam");
    //  to investigate whether we need this to be mutable
    let mut bam = bam::Reader::from_path(bam_path).unwrap();
    let header = bam::Header::from_template(bam.header());
    // print header records to the terminal, akin to samtool
    // The 'key' variable refers to the record type code (e.g., "SQ" for Sequence Dictionary, "HD" for Header Line)
    for (key, records) in header.to_hashmap() {
        if key == "SQ" {
            for record in records {
                println!("@{}\tSN:{}\tLN:{}", key, record["SN"], record["LN"]);
            }
        }
    }
    // The file must be coordinate-sorted before indexing

    index(bam_path);

    // 2. Iterate over records
    let mut count = 0;
    for result in bam.records() {
        let _record = result.unwrap();
        count += 1;

        /*s
        // // --- Basic Fields ---
        // // QNAME (Query Name) - returns &[u8], convert to string for printing
        // let qname = String::from_utf8_lossy(record.qname());

        // // SEQ (Sequence) - .seq() returns a Seq object, .as_bytes() gives standard ASCII (A, C, G, T)
        // let seq = record.seq().as_bytes();
        // let seq_str = String::from_utf8_lossy(&seq);

        // // QUAL (Quality Scores) - returns &[u8] (raw PHRED scores, not offset by 33)
        // let qual = record.qual();

        // // CIGAR string - returns a View that can be iterated or displayed
        // let cigar = record.cigar();

        // // Position information
        // let tid = record.tid(); // Target ID (chromosome index in header)
        // let pos = record.pos(); // 0-based visualization start position
        // let mapq = record.mapq(); // Mapping quality
        */
    }
    println!("Total records: {}", count);
}

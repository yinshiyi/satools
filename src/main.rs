use rust_htslib::{bam, bam::Read}; // Import the Read trait for .records()

fn main() {
    // 1. Must be `mut` to iterate
    let mut bam = bam::Reader::from_path("test_files/NA06984_T.bam").unwrap();

    // (Optional) Copy header if needed for writing later
    let header = bam::Header::from_template(bam.header());

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

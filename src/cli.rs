//! Parse the cli options.

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Mode. Either fastq or SAM/BAM mode.
    #[arg(short = 'm', long = "mode", default_value = "bam")]
    pub mode: String,
    /// Input file.
    #[arg(short = 'i', value_name = "INPUT_FILE")]
    pub input: std::path::PathBuf,
    /// Output file.
    #[arg(short = 'o', value_name = "OUITPUT_FILE")]
    pub output: std::path::PathBuf,
    /// Number of substitution edits to allow. Default: 1
    #[arg(short = 'k', default_value_t = 1)]
    pub k: i32,
    /// The UMI length. If set to a length in fastq mode, then trims the prefix of each read
    /// (note: does not affect the sequence used for deduplicating). Default: autodetect.
    #[arg(short = 'u', default_value_t = 0)]
    pub umi_length: usize,
    /// Threshold percentage for identifying adjacent UMIs in the directional algorithm. Default: 0.5.
    #[arg(short = 'p', default_value_t = 0.5)]
    pub percentage: f32,
    /// Number of threads used in reader/writer.
    #[arg[long = "num-threads", default_value_t = 1]]
    pub num_threads: usize,
    /// Separator string between the UMI and the rest of the read header. Default: _
    #[arg(long = "umi_sep", default_value_t = b'_')]
    pub umi_separator: u8,
    /// Deduplication algorithm. Either cc for connected components, adj for adjacency, or dir for
    /// directional. Default: dir.
    #[arg(long = "algo", default_value = "dir")]
    pub algo_str: String,
    /// Method for identifying which UMI to keep out of every two UMIs. Either any, avgqual, or
    /// mapqual. Default: mapqual for SAM/BAM mode, avgqual for FASTQ mode.
    #[arg(long = "merge")]
    pub merge_str: Option<String>,
    /// Data structure used in deduplication. Either naive, combo, ngram, delete, trie, bktree,
    /// sortbktree, ngrambktree, sortngrambktree, or fenwickbktree. Default: ngrambktree.
    #[arg(long = "data", default_value = "ngrambktree")]
    pub data_str: String,
    /// Use a separate two-pass algorithm for SAM/BAM deduplication. This may be slightly slower,
    /// but it should use much less memory if the reads are approximately sorted by alignment coordinate. Default: false.
    #[arg(long = "two-pass", action)]
    pub two_pass: bool,
    /// Use paired-end mode, which deduplicates pairs of reads from a SAM/BAM file. The template
    /// length of each read pair, along with the alignment coordinate and UMI of the forwards read,
    /// are used to deduplicate read pairs. This is very memory intensive, and the input SAM/BAM files
    /// should be sorted. Default: false (single-end).
    #[arg(long = "paired", action)]
    pub paired: bool,
    /// Remove unpaired reads during paired-end mode. Default: false.
    #[arg(long = "remove-unpaired", action)]
    pub remove_unpaired: bool,
    /// Remove chimeric reads (pairs map to different references) during paired-end mode. Default: false.
    #[arg(long = "remove-chimeric", action)]
    pub remove_chimeric: bool,
    /// Keep unmapped reads (no paired-end mode). Default: false.
    #[arg(long = "keep-unmapped", action)]
    pub keep_unmapped: bool,
    /// Tag reads that belong to the same group without removing them. In fastq mode, this will
    /// append cluster_id=[unique ID for all reads of the same cluster] to the header of every read.
    /// cluster_size=[number of reads in the cluster] will only be appended to the header of a
    /// consensus read for an entire group/cluster. same_umi=[number of reads with the same UMI]
    /// will be appended to the header of the "best" read of a group of reads with the exact same
    /// UMI (not allowing mismatches). In sam/bam mode, then all reads but the consensus reads will
    /// be marked with the duplicate flag. The MI attribute will be set with the cluster_id and the
    /// RX attribute will be set with the UMI of the consensus read. If applicable, the cs attribute
    /// is set with the cluster_size, and the su attribute is set with the same_umi count. For
    /// paired-end reads, only the forwards reads are tagged. This does not work with the --two-pass
    /// feature.
    #[arg(long = "tag", action)]
    pub track_clusters: bool,
}

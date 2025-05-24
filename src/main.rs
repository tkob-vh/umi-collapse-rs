use std::time::SystemTime;

use clap::Parser;
use tracing::info;

use crate::algo::{adjacency::Adjacency, directional::Directional};
use crate::merge::{AnyMerge, AvgQualMerge, MapQualMerge};
use deduplicate_sam::{DeduplicateInterface, DeduplicateSAM};

mod algo;
mod cli;
mod data;
mod deduplicate_sam;
mod merge;
mod utils;

fn main() {
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .init();

    let mut args = cli::Cli::parse();
    info!("Arguments: {:?}", &args);

    let start_time = SystemTime::now();

    // Set the default merge_str if not detected in cli.
    if args.merge_str.is_none() {
        args.merge_str = if args.mode == "fastq" {
            Some("avgqual".to_owned())
        } else {
            Some("mapqual".to_owned())
        }
    }

    if args.track_clusters && args.two_pass {
        panic!("Cannot track clusters with the two pass algorithm!");
    }

    if args.paired && args.keep_unmapped {
        panic!("Cannot keep unmapped reads with paired-end reads!");
    }

    if args.mode.eq("fastq") {
        //TODO: to be finished.
    } else if args.mode.eq("bam") || args.mode.eq("sam") {
        let mut dedup: Box<dyn DeduplicateInterface> = match (
            args.algo_str.as_str(),
            args.merge_str.as_ref().unwrap().as_str(),
        ) {
            ("dir", "any") => Box::new(DeduplicateSAM::new(
                &args,
                Directional::new(&args),
                AnyMerge::new(),
            )),
            ("dir", "avgqual") => Box::new(DeduplicateSAM::new(
                &args,
                Directional::new(&args),
                AvgQualMerge::new(),
            )),
            ("dir", "mapqual") => Box::new(DeduplicateSAM::new(
                &args,
                Directional::new(&args),
                MapQualMerge::new(),
            )),
            ("adj", "any") => Box::new(DeduplicateSAM::new(
                &args,
                Adjacency::new(&args),
                AnyMerge::new(),
            )),
            ("adj", "avgqual") => Box::new(DeduplicateSAM::new(
                &args,
                Adjacency::new(&args),
                AvgQualMerge::new(),
            )),
            ("adj", "mapqual") => Box::new(DeduplicateSAM::new(
                &args,
                Adjacency::new(&args),
                MapQualMerge::new(),
            )),
            _ => panic!(
                "Invalid algorithm combination: {} , {} and {}",
                args.algo_str,
                args.merge_str.as_ref().unwrap(),
                args.data_str
            ),
        };

        dedup.deduplicate_and_merge(&args, &start_time);
    }

    let end_time = SystemTime::now();

    info!(
        "UMI collapsing finished in {:?} seconds",
        end_time.duration_since(start_time).unwrap().as_secs_f32()
    );
}

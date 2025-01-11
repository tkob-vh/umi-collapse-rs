#!/bin/bash

python ./m5C-UBSseq-0.1/bin/join_pileup.py -i \
  "${dataout}/${srr}/${srr}_unfiltered_uniq.tsv.gz" \
  "${dataout}/${srr}/${srr}_unfiltered_multi.tsv.gz" \
  "${dataout}/${srr}/${srr}_filtered_uniq.tsv.gz" \
  "${dataout}/${srr}/${srr}_filtered_multi.tsv.gz" \
  -o "${dataout}/${srr}/${srr}_genome.arrow"

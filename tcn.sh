#!/usr/bin/env bash

genus=$1
polygon_dir="maximal_polygons/genus$genus/"

topcom=points2triangs --regular --flips --unimodular --affinesymmetries

for lp in $polygon_dir*; do
    if [[ -f $lp ]]; then
        basename=$(basename -- "$lp")
        out="out/genus${genus}/${basename%.*}"
        nontroplanar="nontroplanar"

        echo "Triangulating ${lp} writing out to ${out}"
        # real
        # $(cat $lp | $(topcom) | $tcn -n $nontroplanar -g $genus -o $out)

        # test
        $(cat "topcom_out_ex/genus3.txt" | cargo run -- -n $nontroplanar -g $genus -o $out)
    fi
done

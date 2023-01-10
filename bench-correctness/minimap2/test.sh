#! /bin/bash

set -e

./main -a MT-human.fa MT-orang.fa > result_
./main -x map-ont -d MT-human-ont.mmi MT-human.fa
./main -a MT-human-ont.mmi MT-orang.fa >> result_
diff result result_
rm MT-human-ont.mmi
rm result_

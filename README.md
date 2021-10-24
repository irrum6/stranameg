# strgen
String Generator in rust

usage
pass parameters (command line arguments) to get strings ($1, $2,...... $n )
$1 - first paramter -number of strings [required]
$2 - string length [optional] defaults to 12
$3 - mode [optional]
$4 - next argument [optional] depends on mode

mode
10 random leter string (aka randstr)

$4 -language [optional] defaults to latin alphabet
values
10 latin
11 Georgian

mode
11 randstr - alphabet as command line argument
$4 string acts as alphabet

12 randstr - alphabet.file  as command line argument
$4 alphabet.file name

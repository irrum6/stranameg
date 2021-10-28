# strgen
String Generator in rust

usage
pass parameters (command line arguments) to get strings ($1, $2,...... $n )
$1 - first paramter -number of strings [required]
$2 - string length [optional] defaults to 12
$3 - mode [optional] defaults to random letter string
$4 - next argument [optional] depends on mode


1x random leter string 
mode
10 random leter string
$4 -language [optional] defaults to latin alphabet
values
10 latin
11 Georgian

mode
11 randstr - alphabet as command line argument
$4 string acts as alphabet

12 randstr - alphabet.file  as command line argument
$4 alphabet.file name

2x list string generator

mode
21 string by word  -> app-list
app will look for list in lists directory for following file name patters
[listtype].[language].list
types:nouns,adjectives,names

$4 -language [optional] defaults to latin alphabet
values
11 georgian
12 english


22 string by word  -> list file
$4 list file name
example program 16 64 22 sample.list

sample.list
benevolent,violent,active,passive,repressed,fearful,brave,heroic,punishable,desperate
Benjamin,Brooklyn,Brooks,Bennett,Bella,Beau,Brayden,Bryson,Blake,Braxton

3x coupled words generator
31 coupled words -> app-list adjective_noun
32 coupled words -> app-list adjective_name
app will look for list in lists directory for following file name patters
[listtype].[language].list
types:nouns,adjectives,names

$4 -language [optional] defaults to latin alphabet
values
11 georgian
12 english


33 coupled words -> from list files
$4 list file names  -> single string names separated with ":"
example program 16 64 33 sample.list:sample2.list \n
notice:: $2 does not have effect here
# stranameg
String and Name Generator in rust \

usage \
pass parameters (command line arguments) to get strings ($1, $2,...... $n ) \
$1 - first paramter -number of strings [required] \
$2 - string length [optional] defaults to 12 \
$3 - mode [optional] defaults to random letter string \
$4 - next argument [optional] depends on mode \
$5 - write to file [strings.textout]  1 true , 0 or ommited false

modes

rls  RandomLetters,
rla  RandomLettersFromCustomAlphabet,
rlaf  RandomLettersFromAlphabetFile,
raw  RandomWord,
rawl  RandomWordFromListFile,
cow  CoupledWordsNouns,
cowe  CoupledWordsNames,
cowf  CoupledWordsListFiles,


rls  random leter string \
$4 -language [optional] defaults to english alphabet \
values \
ka georgian \
en english \

mode rla $4 alphabet \

rlaf randstr - $4 alphabet.file name \

raw list string generator \
$2 is ignored
random word  -> app-list \
app will look for list in lists directory for following file name patters \
[listtype].[language].list \
types:nouns,adjectives,names \

$4 -language 
values \
ka georgian \
en english \
de german \

random word  -> list file \
$4 list file name \
example program 16 64 raw sample.list \

sample.list \
benevolent,violent,active,passive,repressed,fearful,brave,heroic,punishable,desperate \
Benjamin,Brooklyn,Brooks,Bennett,Bella,Beau,Brayden,Bryson,Blake,Braxton \
notice : $2 length is ignored \

cow coupled words generator \
cow,cw coupled words -> app-list adjective_noun \
cowe,cwe coupled words -> app-list adjective_name \
app will look for list in lists directory for following file name patters \
[listtype].[language].list \
types:nouns,adjectives,names \

$4 -language values same as in raw,2x

cowf,cwf coupled words -> from list files \
$4 list file names  -> single string names separated with ":" \
example program 16 64 33 sample.list:sample2.list \
notice:: $2(length) does not have effect here \

example: program 8 12 cow en \
example: program 8 12 cow en 1 if you want to write to file \

# alternative modes \
you can pass parameter in different order using "alt" or "-a" switcher as first argument \
following arguments are valid \
num= \
len= \
mode= \
next= \
wtf= \
//wtf is write to file \
example \
program alt mode=rla next=abc \

you can pass arguments as file using "pf" or "paramsfile" switcher \
arguemnts same as in alt mode \

example \
program pf params.file \
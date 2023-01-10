# Stranameg
(A stupid) String and Name Generator in rust \

## 1 Basic usage
Stranameg is a command line application.
You can launch binary (compiled for linux) or compile it yoiurself with one or more parameters.


pass parameters (command line arguments) to generate strings, all but first are optional (you can ommit all but first)
- First paramete is the **number** of strings
- Second paramters sets **length** of the generated string, defaults to 12 , this parameter is ignored in modes other than Random Letter String ones (rls,rla,rlaf)
- Third parameter is **mode**, defaults to random letter String (rls)
- Fourth parameter is just another argument whose usage depends on mode
- Fifth argument is to whether or not write to file (**strings.textout**) 1 is true , 0 or ommited is false

### 1.1 Modes

#### 1.1.0 All modes listed
- RandomLetters
- RandomLettersFromCustomAlphabet
- RandomLettersFromAlphabetFile
- RandomWord
- RandomWordFromListFile
- CoupledWordsNouns
- CoupledWordsNames
- CoupledWordsListFiles

#### 1.1.1 Random Letter Strings
- **rls**  RandomLetters,
- **rla**  RandomLettersFromCustomAlphabet, a string must be supplied as 4th argument
- **rlaf**  RandomLettersFromAlphabetFile, filename must be supplied as 4th argument
##### 1.1.1.1 usage of 4th paramter
- RandomLetters - sets language (Ka- for georgian, En (english) or De (German) for latin alphabet)
- RandomLettersFromCustomAlphabet - strings are generated from the letters from this string
- RandomLettersFromAlphabetFile - is used as filename, whose contents serve as basis for our generator

##### 1.1.1.2 examples
application 8 24  - 8 strings with 12 characters length
application 8  - 8 strings with 12 characters length

#### 1.1.2 Random Words
- **raw**  RandomWord,
- **rawl**  RandomWordFromListFile,

##### 1.1.1.1 usage of 4th paramter
- RandomLetters - sets language (Ka- for georgian, En (english) or De (German) for latin alphabet)
- RandomLettersFromCustomAlphabet - strings are generated from the letters from this string
- RandomLettersFromAlphabetFile - is used as filename, whose contents serve as basis for our generator

##### 1.1.1.2 examples
application 8 24  - 8 strings with 12 characters length
application 8  - 8 strings with 12 characters length

#### 1.1.3 Coupled Words
Coupled Words modes generate two words together
- **cow** or **cwo**  CoupledWordsNouns: adjective and nouns
- **cowe** or **cwe** CoupledWordsNames: adjectives and names
- **cowf** or **cwf** CoupledWordsListFiles: same as previous two but user suplies filenames to read and populate the lists

app will look for list in lists directory for following file name patters: **\[listtype\]**.**\[language\]**.list
where list types are : nouns,adjectives,names
##### 1.1.3.1 usage of 4th paramter
- CoupledWordsNouns,CoupledWordsNames - sets language
- CoupledWordsListFiles - filenames separated by :

##### 1.1.3.2 Example
If You run app with following parameters : 16 12 cow En ,
it will look for adjectives.en.list and nouns.en.list and then use their contents to generate strings

./binary 16 12 cwf sample.list:sample2.list
#### 1.1.4 Simple sentences
Generates not so meaningful sentences. Currently only english.

- **sen** Simple sentences

##### 1.1.4.1 Example

## 2 Alternative order of arguments
you can pass parameter in different order using "alt" or "-a" switcher as first argument
following arguments are valid 
num= 
len= 
mode= 
next= 
wtf=
dwi= 
>wtf is write to file
>dwi dont_write_indices whether write or not 

### 2.1 example
program alt mode=rla next=abc 

## 3 Passing arguments from file
you can pass arguments as file using **pf** or **paramsfile** switcher
arguments are the same as in alt mode

example
program pf params.file

## 4 REPL (Interactive) mode
Pass **repl** or **-R** as first parameter to start application in interactive mode

### 4.1 REPL commands
The following commands are available in REPL mode
**.exit** - exits application
**run** - runs generator and prints strings in stdout (console)
**mode** $value - sets mode to $value
**language** or **lang** $value $value - sets language to $value
**number** $value or **num** $value - sets number to $value
**length** $value or **len** $value - sets length to $value 

### 4.2 examples
mode rls
len 128
run
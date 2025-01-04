# Stranameg
(A stupid) String and Name Generator in rust \

+ 1 Basic usage
+ 2 Modes
+ 3 Alternative order of arguments
+ 4 Passing arguments from file
+ 5 Fast Switch 
+ 6 REPL (Interactive) mode
## 1 Basic usage
Stranameg is a command line application.
You can launch binary (compiled for linux) or compile it yourself.
It takes one or more parameters.

pass parameters (command line arguments) to generate strings, all but first are optional.
- First parameter is the **number** of strings
- Second paramters is the **length** of the generated string, defaults to 12 , this parameter is ignored in modes other than Random Letter String ones (rls,rla,rlaf)
- Third parameter is **mode**, defaults to random letter string (rls)
- Fourth parameter is just another argument whose usage depends on mode
- Fifth argument is to whether or not write to file (**strings.textout**) 1 is true , 0 or ommited is false

## 2 Modes

### 2.1 All modes listed
- Password (72 symbols)
- RandomLetters
- RandomLettersFromCustomAlphabet
- RandomLettersFromAlphabetFile
- RandomWord
- RandomWordFromListFile
- CoupledWordsNouns
- CoupledWordsNames
- CoupledWordsListFiles

### 2.2 Random Letter Strings
- **rls**  RandomLetters
- **rla**  RandomLettersFromCustomAlphabet a string must be supplied as 4th argument
- **rlaf**  RandomLettersFromAlphabetFile, filename must be supplied as 4th argument
#### 2.2.1 usage of 4th paramter
- RandomLetters - sets language (Ka- for georgian, En (english) or De (German) for german alphabet)
- RandomLettersFromCustomAlphabet - strings are generated from the letters from this string
- RandomLettersFromAlphabetFile - is used as filename, whose contents serve as the basis for our generator

#### 2.2.2 examples
./binary_linux 8 24  - 8 strings with 24 characters length
./binary_linux 16  - 16 strings with 12 characters length
./binary_linux 8 16 rla "alphabet" - 8 strings with 16 characters length from letter from string "alphabet"
./binary_linux 8 16 rlaf abc.text - 8 strings with 16 characters length using letters from abc.text

### 2.3 Random Words
- **raw**  RandomWord,
- **rawl**  RandomWordFromListFile,
#### 2.3.1 usage of 4th paramter
- RandomWord - sets language (Ka- for georgian, En (english) or De (German) for latin alphabet)
- RandomWordFromListFile - takes words from that file

#### 2.3.2 examples
./binary_linux 16 12 raw de
./binary_linux 16 12 rawl sample.list

### 2.4 Coupled Words
Coupled Words modes generate two words together
- **cow** or **cwo**  CoupledWordsNouns: adjective and nouns
- **cowe** or **cwe** CoupledWordsNames: adjectives and names
- **cowf** or **cwf** CoupledWordsListFiles: same as previous two but user suplies filenames to read and populate the lists

app will look for list in lists directory for following file name patters: **\[listtype\]**.**\[language\]**.list
where list types are : nouns,adjectives,names
#### 2.4.1 usage of 4th paramter
- CoupledWordsNouns,CoupledWordsNames - sets language (words ar sourced from adjectives.[language].list and nouns.[language].list)
- CoupledWordsListFiles - filenames separated by : serve as source for adjectives and nouns

#### 2.4.2 Example
./binary_linux 16 12 cow en
./binary_linux 16 12 cwf sample.list:sample2.list

### 2.5 Simple sentences
Generates not so meaningful sentences. Currently only english.

- **sen** Simple sentences

#### 2.5.1 usage of 4th paramter
Currently ignore
#### 2.5.2 Example
./binary 16 12 sen

## 3 Alternative order of arguments
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

### 3.1 example
program alt mode=rla next=abc 

## 4 Passing arguments from file
you can pass arguments as file using **pf** or **paramsfile** switcher
arguments are the same as in alt mode

example
program pf params.file
## 5 Fast switch and aliases
### 5.1 Fast swtich
Use **-f** switch  and then immediatelly one of the following set one parameter (others will default)
- n for numbers
- s for length
- l for language
- m for mode

### 5.2 Fast switch aliases
Or use aliases for certain options

- **R1** for RandomLetters
- **R2** for RandomLettersFromCustomAlphabet
- **R3** for RandomLettersFromAlphabetFile
- **W1** for RandomWord
- **W2** for RandomWordFromListFile
- **C1** for CoupledWordsNouns
- **C2** for CoupledWordsNames
- **C3** for CoupledWordsListFiles 
- **S1** 16 characters long
- **S2** 32 characters long
- **S3** 48 characters long
- **S4** 64 characters long
- **S5** 80 characters long
- **S6** 96 characters long
- **S7** 112 characters long
- **S8** 128 characters long
- **S9** 144 characters long
- **SX** 160 characters long
It will also set one parameter
### 5.3 Examples
./binary -fn8
./binary -fmrla mode RLA (2nd parameter for alphabet is required)
./binary S2 - 32 characters length
./binary R2 - RLA mode (2nd parameter for alphabet is required)
./binary R3 - RLAF mode (2nd parameter for filename is required)


## 6 REPL (Interactive) mode
Pass **repl** or **-R** as first parameter to start application in interactive mode

### 6.1 REPL commands
The following commands are available in REPL mode
**.exit** - exits application
**run** - runs generator and prints strings in stdout (console)
**mode** $value - sets mode to $value
**language** or **lang** $value $value - sets language to $value
**number** $value or **num** $value - sets number to $value
**length** $value or **len** $value - sets length to $value 

### 6.2 examples
mode rls
len 128
run
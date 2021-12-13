pub mod help {
    pub fn print_help() {
        let help_string = "# strgen
        String Generator in rust 
        
        usage 
        pass parameters (command line arguments) to get strings ($1, $2,...... $n ) 
        $1 - first paramter -number of strings [required] 
        $2 - string length [optional] defaults to 12 
        $3 - mode [optional] defaults to random letter string 
        $4 - next argument [optional] depends on mode 
        $5 - write to file [strings.textout]  1 true , 0 or ommited false
        
        modes
        
        rls,10  RandomLetters,
        rla,11  RandomLettersFromCustomAlphabet,
        rlaf,12  RandomLettersFromAlphabetFile,
        raw,21  RandomWord,
        rawl,22  RandomWordFromListFile,
        cow,31  CoupledWordsNouns,
        cowe,32  CoupledWordsNames,
        cowf,33  CoupledWordsListFiles,
        
        
        rls|1x 
        mode 
        10 random leter string 
        $4 -language [optional] defaults to english alphabet 
        values 
        ka georgian 
        en english 
        
        mode 
        rla,11 $4 alphabet 
        
        rlaf,12 randstr - $4 alphabet.file name 
        
        row 2x list string generator 
        
        mode 
        $2 is ignored
        21 random word  -> app-list
        app will look for list in lists directory for following file name patters 
        [listtype].[language].list 
        types:nouns,adjectives,names 
        
        $4 -language 
        values 
        ka georgian 
        en english 
        de german 
        
        22 srandom word  -> list file 
        $4 list file name 
        example program 16 64 raw sample.list 
        
        sample.list 
        benevolent,violent,active,passive,repressed,fearful,brave,heroic,punishable,desperate 
        Benjamin,Brooklyn,Brooks,Bennett,Bella,Beau,Brayden,Bryson,Blake,Braxton 
        
        notice : $2 here refers to minimal length
        
        3x coupled words generator 
        cow,31 coupled words -> app-list adjective_noun 
        cowe,32 coupled words -> app-list adjective_name 
        app will look for list in lists directory for following file name patters 
        [listtype].[language].list 
        types:nouns,adjectives,names 
        
        $4 -language values same as in raw,2x
        
        33 coupled words -> from list files 
        $4 list file names  -> single string names separated with ':' 
        example program 16 64 33 sample.list:sample2.list 
        notice:: $2(length) does not have effect here 
        
        example: program 8 12 cow en 
        example: program 8 12 cow en 1 if you want to write to file ";
        println!("{}", help_string);
    }
}

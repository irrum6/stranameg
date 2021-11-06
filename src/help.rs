pub mod help {
    pub fn print_help() {
        let help_string = "# strgen
        String Generator in rust \n
        
        usage \n
        pass -h as first parameter to display this message
        pass parameters (command line arguments) to get strings ($1, $2,...... $n ) \n
        $1 - first paramter -number of strings [required] \n
        $2 - string length [optional] defaults to 12 \n
        $3 - mode [optional] defaults to random letter string \n
        $4 - next argument [optional] depends on mode \n
        
        
        1x random leter string \n
        mode \n
        10 random leter string \n
        $4 -language [optional] defaults to latin alphabet \n
        values \n
        10 latin \n
        11 Georgian \n
        
        mode \n
        11 randstr - alphabet as command line argument \n
        $4 string acts as alphabet \n
        
        12 randstr - alphabet.file  as command line argument \n
        $4 alphabet.file name \n
        
        2x list string generator \n
        
        mode \n
        21 string by word  -> app-list \n
        app will look for list in lists directory for following file name patters \n
        [listtype].[language].list \n
        types:nouns,adjectives,names \n
        
        $4 -language [optional] defaults to latin alphabet \n
        values \n
        11 georgian \n
        12 english \n
        
        
        22 string by word  -> list file \n
        $4 list file name \n
        example program 16 64 22 sample.list \n
        
        sample.list \n
        benevolent,violent,active,passive,repressed,fearful,brave,heroic,punishable,desperate \n
        Benjamin,Brooklyn,Brooks,Bennett,Bella,Beau,Brayden,Bryson,Blake,Braxton \n
        
        notice : $2 here refers to minimal length
        
        3x coupled words generator \n
        31 coupled words -> app-list adjective_noun \n
        32 coupled words -> app-list adjective_name \n
        app will look for list in lists directory for following file name patters \n
        [listtype].[language].list \n
        types:nouns,adjectives,names \n
        
        $4 -language [optional] defaults to latin alphabet \n
        values \n
        11 georgian \n
        12 english \n
        
        33 coupled words -> from list files \n
        $4 list file names  -> single string names separated with : \n
        example program 16 64 33 sample.list:sample2.list \n
        notice:: $2(length) does not have effect here \n
        
        add zero(0) to mode to make program write to file\n
        example: program 8 12 310 12\n
        As of now program writes to file named strings.textout which will be created \n
        in the same directory , from where it was run. \n";
        println!("{}", help_string);
    }
}
